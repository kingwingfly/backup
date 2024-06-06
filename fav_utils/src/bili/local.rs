use super::{Bili, BiliSets};
use crate::{utils::bar::download_bar, FavUtilsError};
use core::future::Future;
use fav_core::prelude::*;
use reqwest::header::CONTENT_LENGTH;
use std::io::{BufWriter, Write as _};
use tracing::error;

impl PathInfo for Bili {
    #[cfg(test)]
    const PATH: &'static str = concat!(env!("OUT_DIR"), "/fav_cache/bili");
    #[cfg(not(test))]
    const PATH: &'static str = ".fav/bili";
}

impl PathInfo for BiliSets {
    #[cfg(test)]
    const PATH: &'static str = concat!(env!("OUT_DIR"), "/fav_cache/sets");
    #[cfg(not(test))]
    const PATH: &'static str = ".fav/sets";
}

impl SaveLocal for Bili {
    async fn download<R, F, Fut, Any>(
        &self,
        res: &mut R,
        urls: Vec<reqwest::Url>,
        f: F,
    ) -> FavCoreResult<()>
    where
        R: Res,
        F: Fn() -> Fut + Send,
        Fut: Future<Output = Any> + Send,
        Any: Send,
    {
        let title = res.title();
        let id = String::from(res.id());
        let mut urls = urls;
        let mut resp_v = self.client().get(urls.pop().unwrap()).send().await?;
        let mut resp_a = self.client().get(urls.pop().unwrap()).send().await?;
        let size = resp_v.headers()[CONTENT_LENGTH]
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap()
            + resp_a.headers()[CONTENT_LENGTH]
                .to_str()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        let pb = download_bar(size, title);

        let mut file_v = BufWriter::new(tempfile::NamedTempFile::new()?);
        let mut file_a = BufWriter::new(tempfile::NamedTempFile::new()?);
        let mut finish_v = false;
        let mut finish_a = false;
        let mut failed_reason = None;
        loop {
            tokio::select! {
                chunk = resp_v.chunk(), if !finish_v => {
                    match chunk {
                        Ok(Some(chunk)) => {
                            pb.inc(chunk.len() as u64);
                            file_v.write_all(&chunk).unwrap();
                        }
                        Ok(None) => finish_v = true,
                        Err(e) => {
                            error!("Failed to download video: {}", res.id());
                            failed_reason = Some(e);
                        }
                    }
                },
                chunk = resp_a.chunk(), if !finish_a => {
                    match chunk {
                        Ok(Some(chunk)) => {
                            pb.inc(chunk.len() as u64);
                            file_a.write_all(&chunk).unwrap();
                        }
                        Ok(None) => finish_a = true,
                        Err(e) => {
                            error!("Failed to download video: {}", res.id());
                            failed_reason = Some(e);
                        }
                    }
                },
                _ = async {}, if finish_v && finish_a => {
                    file_v.flush().unwrap();
                    file_a.flush().unwrap();
                    pb.finish();
                    merge(
                        title,
                        &id,
                        file_v.into_inner().unwrap().path().to_str().unwrap(),
                        file_a.into_inner().unwrap().path().to_str().unwrap(),
                    )
                    .await?;
                    res.on_status(StatusFlags::SAVED);
                    return Ok(())
                },
                _ = async {}, if failed_reason.is_some() => {
                    file_v.into_inner().unwrap().close()?;
                    file_a.into_inner().unwrap().close()?;
                    return Err(failed_reason.unwrap().into());
                },
                _ = f() => {
                    file_v.into_inner().unwrap().close()?;
                    file_a.into_inner().unwrap().close()?;
                    return Err(FavCoreError::Cancel)
                }
            }
        }
    }
}

async fn merge(title: &str, id: &str, path_v: &str, path_a: &str) -> FavCoreResult<()> {
    let mut title = sanitize_filename::sanitize(title);
    title.push_str(id);
    let status = tokio::process::Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            path_v,
            "-i",
            path_a,
            "-codec",
            "copy",
            "-f",
            "mp4",
            &format!("./{}.mp4", title),
        ])
        .stderr(std::process::Stdio::null())
        .status()
        .await
        .unwrap();
    match status.success() {
        true => Ok(()),
        false => Err(FavCoreError::UtilsError(Box::new(
            FavUtilsError::MergeFailed,
        ))),
    }
}
