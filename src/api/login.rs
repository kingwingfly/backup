use super::{client, error::Result};
use crate::proto::data::Cookie;
use qrcode::{render::unicode, QrCode};
use tracing::{instrument, warn};

const QR_API: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
const QR_RET_API: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";
const POLL_INTERVAL: u64 = 3;

/// Login with QR code.
#[instrument(name = "QR Login", ret)]
pub(crate) async fn qr_login() -> Result<()> {
    let QrInfo { url, qrcode_key } = qr_info().await?;
    show_qr_code(url).await?;
    qr_ret(qrcode_key).await?;
    Ok(())
}

#[cfg(feature = "bili")]
#[derive(serde::Deserialize)]
struct QrInfo {
    url: String,
    qrcode_key: String,
}

impl<'a, T> From<T> for Cookie
where
    T: Iterator<Item = reqwest::cookie::Cookie<'a>> + 'a,
{
    fn from(cookies: T) -> Self {
        let mut cookie = Self::default();
        for c in cookies {
            match c.name() {
                "DedeUserID" => cookie.DedeUserID = c.value().to_string(),
                "DedeUserID__ckMd5" => cookie.DedeUserID__ckMd5 = c.value().to_string(),
                "SESSDATA" => cookie.SESSDATA = c.value().to_string(),
                "bili_jct" => cookie.bili_jct = c.value().to_string(),
                name => warn!("unknown cookie: {}", name),
            }
        }
        cookie
    }
}

async fn qr_info() -> Result<QrInfo> {
    let resp = reqwest::get(QR_API).await?;
    let mut json: serde_json::Value = resp.json().await?;
    tracing::debug!("{:#?}", json);
    Ok(serde_json::from_value(json.pointer_mut("/data").unwrap().take()).unwrap())
}

async fn show_qr_code(url: String) -> Result<()> {
    let code = QrCode::new(url).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("\n{}", image);
    Ok(())
}

async fn qr_ret(qrcode_key: String) -> Result<()> {
    let url = reqwest::Url::parse_with_params(QR_RET_API, [("qrcode_key", qrcode_key)]).unwrap();
    loop {
        let resp = client().get(url.clone()).send().await?;
        let cookie: Cookie = resp.cookies().into();
        let json: serde_json::Value = resp.json().await?;
        match json.pointer("/data/code").unwrap().as_i64().unwrap() {
            0 => {
                cookie.persist();
                break;
            }
            86038 => warn!("QR code expired"),
            _ => tracing::debug!("{:#?}", json.pointer("/data/message").unwrap()),
        }
        tokio::time::sleep(std::time::Duration::from_secs(POLL_INTERVAL)).await;
    }
    Ok(())
}
