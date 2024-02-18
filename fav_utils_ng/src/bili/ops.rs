use super::api::ApiKind;
use crate::{
    proto::bili::{Bili, BiliRes, BiliSet, BiliSets},
    utils::qr::show_qr_code,
    FavUtilsError, FavUtilsResult,
};
use fav_core::{prelude::*, status::SetStatusExt as _};
use reqwest::Response;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

const POLL_INTERVAL: u64 = 3;
const EXPIRED_DURATION: u64 = 120;
const HINT: &str = "Never Login";

impl Operations<BiliSets, BiliSet, BiliRes, ApiKind> for Bili {
    async fn login(&mut self) -> FavCoreResult<()> {
        let QrInfo { url, qrcode_key } = self.request_json(ApiKind::Qr, &[], "/data").await?;
        show_qr_code(url)?;
        for _ in 0..EXPIRED_DURATION / POLL_INTERVAL {
            sleep(Duration::from_secs(POLL_INTERVAL)).await;
            let resp = self
                .request(ApiKind::QrPoll, &[qrcode_key.as_str()])
                .await?;
            if let Ok(cookies) = try_extract_cookie(&resp) {
                self.extend_cookies(cookies);
                return Ok(());
            }
        }
        Err(FavCoreError::UtilsError(Box::new(FavUtilsError::QrExpired)))
    }

    async fn logout(&mut self) -> FavCoreResult<()> {
        let params = &[self.cookies().get("bili_jct").expect(HINT).as_str()];
        match self.request_json(ApiKind::Logout, params, "/code").await? {
            0 => Ok(()),
            _ => Err(FavCoreError::UtilsError(Box::new(
                FavUtilsError::LogoutError,
            ))),
        }
    }

    async fn fetch_sets(&self, sets: &mut BiliSets) -> FavCoreResult<()> {
        let params = &[self.cookies().get("DedeUserID").expect(HINT).as_str()];
        *sets |= self
            .request_proto(ApiKind::FetchSets, params, "/data")
            .await?;
        Ok(())
    }

    async fn fetch_set(&self, set: &mut BiliSet) -> FavCoreResult<()> {
        let id = set.id.to_string();
        for pn in 1..=set.media_count.saturating_sub(1) / 20 + 1 {
            let pn = pn.to_string();
            let params = &[id.as_str(), pn.as_str(), "20"];
            *set |= self
                .request_proto::<BiliSet>(ApiKind::FetchSet, params, "/data")
                .await?
                .with_res_status_on(StatusFlags::FAV);
        }
        Ok(())
    }

    async fn fetch_res(&self, resource: &mut BiliRes) -> FavCoreResult<()> {
        let id = resource.bvid.clone();
        let params = &[id.as_str()];
        tokio::select! {
            res = self.request_proto::<BiliRes>(ApiKind::FetchRes, params, "/data") => {
                    *resource |= res?;
                    resource.on_status(StatusFlags::FETCHED);
                },
            _ = tokio::signal::ctrl_c() => {
                return Err(FavCoreError::Cancel);
            }
        }
        Ok(())
    }

    async fn pull(&self, resource: &mut BiliRes) -> FavCoreResult<()> {
        let Wbi { img_url, sub_url } = self
            .request_json::<Wbi>(ApiKind::Wbi, &[], "/data/wbi_img")
            .await?;
        let bvid = resource.bvid.as_str();
        let cid = resource.cid.to_string();
        let qn = "80";
        let fnval = (16 | 1024).to_string();
        let fourk = "1";
        let params = &[
            bvid,
            cid.as_str(),
            qn,
            fnval.as_str(),
            fourk,
            sub_url.as_str(),
            img_url.as_str(),
        ];
        let json: serde_json::Value = self.request_json(ApiKind::Pull, params, "").await?;
        dbg!(json);
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
struct QrInfo {
    url: String,
    qrcode_key: String,
}

#[derive(Debug, serde::Deserialize)]
struct Wbi {
    img_url: String,
    sub_url: String,
}

fn try_extract_cookie(resp: &Response) -> FavUtilsResult<HashMap<String, String>> {
    let cookies = resp.cookies().collect::<Vec<_>>();
    if cookies.is_empty() {
        return Err(FavUtilsError::NoCookie);
    }
    Ok(cookies
        .iter()
        .map(|c| (c.name().to_string(), c.value().to_string()))
        .collect())
}

#[cfg(test)]
mod tests {
    use fav_core::ops::OperationsExt;

    use super::*;
    use crate::proto::bili::BiliSets;

    #[tokio::test]
    #[ignore = "need to scan qr code manually"]
    async fn login_test() {
        let mut bili = Bili::default();
        bili.login().await.unwrap();
    }

    #[tokio::test]
    async fn ops_test() {
        let bili = Bili::read().unwrap();
        let mut sets = BiliSets::default();
        bili.fetch_sets(&mut sets).await.unwrap();
        let set = sets.iter_mut().min_by_key(|s| s.media_count).unwrap();
        bili.fetch_set(set).await.unwrap();
        bili.fetch_all(set).await.unwrap();
        bili.pull_all(set).await.unwrap();
        dbg!(set);
    }
}
