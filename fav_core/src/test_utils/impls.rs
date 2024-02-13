use super::data::App;
use ::core::future::Future;
use reqwest::{header::HeaderMap, Client, Method, Response};
use std::future::IntoFuture;
use url::Url;

#[cfg(test)]
use crate::prelude::*;

#[cfg(not(test))]
use fav_core::prelude::*;

impl HttpConfig for App {
    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }

    fn cookies(&self) -> &std::collections::HashMap<String, String> {
        todo!()
    }

    fn set_cookies(&mut self, cookies: std::collections::HashMap<String, String>) {
        todo!()
    }
}

impl PathInfo for App {
    const PATH: &'static str = "temp/app";
}

struct LoginApi;

impl Api for LoginApi {
    fn raw_api(&self) -> &'static str {
        "http://www.example.com"
    }

    fn params(&self) -> &[&str] {
        &[]
    }
}

impl ApiProvider<DefaultApiKind> for App {
    fn api(&self, api_kind: DefaultApiKind) -> Box<dyn Api + Send> {
        Box::new(match api_kind {
            DefaultApiKind::Login => LoginApi,
            _ => unimplemented!(),
        })
    }
}

impl Operations<DefaultApiKind> for App {
    async fn login(&mut self) -> FavCoreResult<()> {
        // let resp = self.request(DefaultApiKind::Login, vec![]).await?;
        Ok(())
    }

    async fn logout(&mut self) -> FavCoreResult<()> {
        todo!()
    }

    async fn fetch(&self, resource: &mut impl ResAttr) -> FavCoreResult<()> {
        todo!()
    }

    async fn pull(&self, resource: &mut impl ResAttr) -> FavCoreResult<()> {
        todo!()
    }
}
