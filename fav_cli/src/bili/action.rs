use fav_core::ops::{ResOpsExt as _, SetOpsExt as _};
use fav_core::prelude::*;
use fav_core::status::SetStatusExt as _;
use fav_core::visual::{TableRes as _, TableSet as _, TableSets as _};
use fav_utils::bili::{Bili, BiliRes, BiliSet, BiliSets};
use std::io::Write as _;
use tracing::{info, warn};

pub(super) fn init() -> FavCoreResult<()> {
    let path = std::path::PathBuf::from(BiliSets::PATH);
    if path.exists() {
        let mut stdout = std::io::stdout();
        stdout
            .write_all(b"The .fav folder already exists, do you want to overwrite it? (y/n): ")?;
        stdout.flush()?;
        let stdin = std::io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        if buf.trim() != "y" {
            return Ok(());
        }
    }
    BiliSets::default().write()?;
    info!("The .fav folder has been initialized.");
    Ok(())
}

pub(super) async fn login() -> FavCoreResult<()> {
    let mut bili = Bili::default();
    bili.login().await?;
    bili.write()
}

pub(super) async fn logout() -> FavCoreResult<()> {
    let mut bili = Bili::read()?;
    bili.logout().await
}

pub(super) fn status(sets: &mut BiliSets, id: String) -> FavCoreResult<()> {
    let id_ = Id::from(&id);
    if let Some(s) = find_set(sets, &id_) {
        s.table();
    } else if let Some(r) = find_res(sets, &id_) {
        r.table();
    } else {
        return Err(FavCoreError::IdNotUsable {
            id,
            msg: "ID not found".to_string(),
        });
    }
    Ok(())
}

pub(super) fn status_all(
    sets: &mut BiliSets,
    show_sets: bool,
    show_res: bool,
    only_track: bool,
) -> FavCoreResult<()> {
    if show_sets {
        let sub = sets.subset(|s| s.check_status(StatusFlags::TRACK) | !only_track);
        sub.table();
    }
    if show_res {
        for set in sets.iter_mut() {
            let sub = set.subset(|r| r.check_status(StatusFlags::TRACK) | !only_track);
            sub.table();
        }
    }
    Ok(())
}

pub(super) async fn fetch(sets: &mut BiliSets) -> FavCoreResult<()> {
    let bili = Bili::read()?;
    bili.fetch_sets(sets).await?;
    let mut sub = sets.subset(|s| s.check_status(StatusFlags::TRACK));
    bili.batch_fetch_set(&mut sub).await?;
    for set in sub.iter_mut() {
        let mut sub = set.subset(|r| {
            r.check_status(StatusFlags::TRACK)
                & !r.check_status(StatusFlags::FETCHED)
                & !r.check_status(StatusFlags::EXPIRED)
        });
        bili.batch_fetch_res(&mut sub).await?;
    }
    Ok(())
}

pub(super) fn track(sets: &mut BiliSets, id: String) -> FavCoreResult<()> {
    let id_ = Id::from(&id);
    if let Some(s) = find_set(sets, &id_) {
        s.on_status(StatusFlags::TRACK);
        s.on_res_status(StatusFlags::TRACK);
    } else if let Some(r) = find_res(sets, &id_) {
        r.on_status(StatusFlags::TRACK);
    } else {
        return Err(FavCoreError::IdNotUsable {
            id,
            msg: "ID not found".to_string(),
        });
    }
    info!("Tracked ID: {id}");
    Ok(())
}

pub(super) fn untrack(sets: &mut BiliSets, id: String) -> FavCoreResult<()> {
    let id_ = Id::from(&id);
    if let Some(s) = find_set(sets, &id_) {
        s.off_status(StatusFlags::TRACK);
        s.medias.clear();
    } else if let Some(r) = find_res(sets, &id_) {
        r.off_status(StatusFlags::TRACK);
    } else {
        return Err(FavCoreError::IdNotUsable {
            id,
            msg: "ID not found".to_string(),
        });
    }
    info!("Untracked ID: {id}");
    Ok(())
}

pub(super) async fn pull_all(sets: &mut BiliSets) -> FavCoreResult<()> {
    fetch(sets).await?;
    let bili = Bili::read()?;
    let mut sub = sets.subset(|s| s.check_status(StatusFlags::TRACK));
    for set in sub.iter_mut() {
        let mut sub = set.subset(|r| {
            !r.check_status(StatusFlags::SAVED)
                & !r.check_status(StatusFlags::EXPIRED)
                & r.check_status(StatusFlags::TRACK | StatusFlags::FETCHED)
        });
        bili.batch_pull_res(&mut sub).await?;
    }
    Ok(())
}

pub(super) async fn pull(sets: &mut BiliSets, id: String) -> FavCoreResult<()> {
    fetch(sets).await?;
    let bili = Bili::read()?;
    let id_ = Id::from(&id);
    if let Some(s) = find_set(sets, &id_) {
        let mut sub = s.subset(|r| {
            !r.check_status(StatusFlags::SAVED)
                & !r.check_status(StatusFlags::EXPIRED)
                & r.check_status(StatusFlags::TRACK | StatusFlags::FETCHED)
        });
        bili.batch_pull_res(&mut sub).await?;
    } else if let Some(r) = find_res(sets, &id_) {
        if !r.check_status(StatusFlags::EXPIRED)
            & r.check_status(StatusFlags::TRACK | StatusFlags::FETCHED)
        {
            bili.pull_res(r).await?;
        } else {
            return Err(FavCoreError::IdNotUsable {
                id,
                msg: "EXPIRED or UNTRACK, unable to fetch".to_string(),
            });
        }
    } else {
        return Err(FavCoreError::IdNotUsable {
            id,
            msg: "Not Found".to_string(),
        });
    }
    Ok(())
}

pub(super) async fn daemon(sets: &mut BiliSets, interval: u64) -> FavCoreResult<()> {
    if interval < 15 {
        warn!("Interval would better to be greater than 15 minutes.");
    }
    let duration = tokio::time::Duration::from_secs(interval * 60);
    let interval = chrono::Duration::try_minutes(interval as i64).expect("invalid interval.");

    let mut fire: bool = true;
    loop {
        tokio::select! {
            _ = pull_all(sets), if fire => {
                let next_ts_local = (chrono::Utc::now() + interval)
                    .with_timezone(&chrono::Local)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
                info!(
                    "Next job will be {} minutes later at {}.\n",
                    interval.num_minutes(),
                    next_ts_local
                );
                fire = false;
            }
            _ = tokio::time::sleep(duration), if !fire => fire = true,
            _ = tokio::signal::ctrl_c() => {
                info!("Received Ctrl-C, exiting.");
                break;
            }
        }
    }
    Ok(())
}

fn find_set<'a>(sets: &'a mut BiliSets, id: &Id) -> Option<&'a mut BiliSet> {
    sets.iter_mut().find(|s| s.id() == *id)
}

fn find_res<'a>(sets: &'a mut BiliSets, id: &Id) -> Option<&'a mut BiliRes> {
    for set in sets.iter_mut() {
        if let Some(r) = set.iter_mut().find(|r| r.id() == *id) {
            return Some(r);
        }
    }
    None
}
