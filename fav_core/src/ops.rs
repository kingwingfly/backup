//! The `Operations` trait,
//! making app able to perform more operations

use crate::{
    api::ApiProvider,
    config::HttpConfig,
    error::FavCoreError,
    res::{Res, Set, Sets},
    FavCoreResult,
};
use core::future::Future;
use futures::StreamExt;
use protobuf::MessageFull;
use protobuf_json_mapping::{parse_from_str_with_options, ParseOptions};
use reqwest::{header, Client, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio_util::sync::{CancellationToken, WaitForCancellationFutureOwned};
use tracing::error;

const PARSE_OPTIONS: ParseOptions = ParseOptions {
    ignore_unknown_fields: true,
    _future_options: (),
};

/// Making a client able to perform operations.
///
/// Work with [`ApiProvider`] and [`HttpConfig`] to perform operations in `K`.
///
/// # Example
/// ```no_run
/// # #[path = "test_utils/mod.rs"]
/// # mod test_utils;
/// use test_utils::data::App;
/// use fav_core::ops::{AuthOps};
///
/// # #[tokio::main]
/// # async fn main() {
/// let mut app = App::default();
/// app.login().await.unwrap();
/// # }
/// ```
/// `App` above is a struct that implements `LocalAuthOps`/`AuthOps`,
/// see [concret implementation](https://github.com/kingwingfly/fav/blob/dev/fav_core/src/test_utils/impls.rs).
/// # Hint
/// Since `LocalXXOps` is not `Send`, one should use it in a single-threaded runtime.
/// If you need async operations in a multi-threaded runtime, use `XXOps`.
///
/// To let your editor generate `XXOps` required methods signatures, use `LocalXXOps` first,
/// after your editor generating the signatures, change `LocalXXOps` to `XXOps`.
pub trait Ops<ApiKind>: AuthOps + SetsOps + SetOps + ResOps {}

/// Making a client able to auth acount
/// - [`LocalAuthOps`]'s async methods cannot be Send.
/// - [`AuthOps`] is generated by [`trait_variant::make`], which implements `Send`.
/// For more information, see [Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits).
#[trait_variant::make(AuthOps: Send)]
pub trait LocalAuthOps: Net + HttpConfig {
    /// Login to the account.
    async fn login(&mut self) -> FavCoreResult<()>;
    /// Logout from the account.
    async fn logout(&mut self) -> FavCoreResult<()>;
}

/// Making a client able to operate on resource sets
/// - [`LocalSetsOps`]'s async methods cannot be Send.
/// - [`SetsOps`] is generated by [`trait_variant::make`], which implements `Send`.
/// For more information, see [Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits).
#[trait_variant::make(SetsOps: Send)]
pub trait LocalSetsOps: Net + HttpConfig {
    /// The sets type the operations on
    type Sets: Sets;
    /// Fetch all resource sets
    async fn fetch_sets(&self, sets: &mut Self::Sets) -> FavCoreResult<()>;
}

/// Making a client able to operate resource set
/// - [`LocalSetOps`]'s async methods cannot be Send.
/// - [`SetOps`] is generated by [`trait_variant::make`], which implements `Send`.
/// For more information, see [Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits).
#[trait_variant::make(SetOps: Send)]
pub trait LocalSetOps: Net + HttpConfig {
    /// The set type the operations on
    type Set: Set;
    /// Fetch one resource set,
    /// `cancelled: Future<...>`, if Future is ready, one can cleanup and
    /// shutdown gracefully, then return `FavCoreError::Cancel`.
    async fn fetch_set<Fut, Any>(&self, set: &mut Self::Set, cancelled: Fut) -> FavCoreResult<()>
    where
        Fut: Future<Output = Any> + Send,
        Any: Send;
}

/// Making a client able to operate on resource
/// - [`LocalResOps`]'s async methods cannot be Send.
/// - [`ResOps`] is generated by [`trait_variant::make`], which implements `Send`.
/// For more information, see [Rust Blog](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits).
#[trait_variant::make(ResOps: Send)]
pub trait LocalResOps: Net + HttpConfig {
    /// The resource type the operations on
    type Res: Res;
    /// Fetch one resource,
    /// `cancelled: Future<...>`, if Future is ready, one can cleanup and
    /// shutdown gracefully, then return `FavCoreError::Cancel`.
    async fn fetch_res<Fut, Any>(
        &self,
        resource: &mut Self::Res,
        cancelled: Fut,
    ) -> FavCoreResult<()>
    where
        Fut: Future<Output = Any> + Send,
        Any: Send;
    /// Pull one resource,
    /// `cancelled: Future<...>`, if Future is ready, one can cleanup and
    /// shutdown gracefully, then return `FavCoreError::Cancel`.
    async fn pull_res<Fut, Any>(
        &self,
        resource: &mut Self::Res,
        cancelled: Fut,
    ) -> FavCoreResult<()>
    where
        Fut: Future<Output = Any> + Send,
        Any: Send;
}

/// `SetsOpsExt`, including methods to batch fetch set in sets.
/// # Example
/// ```no_run
/// # #[path = "test_utils/mod.rs"]
/// # mod test_utils;
/// # use test_utils::data::{App, TestSets};
/// # use fav_core::{status::{Status, StatusFlags}, res::Sets, ops::SetOpsExt};
/// # async {
/// let app = App::default();
/// let mut sets = TestSets::default();
/// let mut sub = sets.subset(|r| r.check_status(StatusFlags::TRACK));
/// app.batch_fetch_set(&mut sub);
/// # };
/// ```
pub trait SetOpsExt: SetOps {
    /// **Asynchronously** fetch sets in sets using [`SetOps::fetch_set`].
    fn batch_fetch_set<'a, SS>(&self, sets: &'a mut SS) -> impl Future<Output = FavCoreResult<()>>
    where
        SS: Sets<Set = Self::Set>,
    {
        batch_op_set(sets, |s, fut| self.fetch_set(s, fut))
    }
}

/// A helper function to batch do operation on sets.
/// You can use it like [`batch_op_set`]
/// However, it's better to use [`Sets::subset`] and [`SetOpsExt`] instead.
/// See [`SetOpsExt`] for more information.
pub async fn batch_op_set<'a, SS, F, T>(sets: &'a mut SS, mut f: F) -> FavCoreResult<()>
where
    SS: Sets + 'a,
    F: FnMut(&'a mut SS::Set, WaitForCancellationFutureOwned) -> T,
    T: Future<Output = FavCoreResult<()>>,
{
    let token = CancellationToken::new();
    let mut stream = tokio_stream::iter(sets.iter_mut())
        .map(|s| {
            let shutdown = token.clone().cancelled_owned();
            let fut = f(s, shutdown);
            let token = token.clone();
            async move {
                if token.is_cancelled() {
                    Err(FavCoreError::Cancel)
                } else {
                    fut.await
                }
            }
        })
        .buffer_unordered(8);
    let mut result = Ok(());
    tokio::select! {
        _ = async {
            while let Some(res) = stream.next().await {
                match res {
                    Err(FavCoreError::Cancel) => {}
                    Err(FavCoreError::NetworkError(e)) if e.is_connect() => {
                        token.cancel();  // if already cancelled, it's handled by token itself
                        result = Err(FavCoreError::NetworkError(e));
                    }
                    Err(e) => error!("{}", e),
                    _ => {}
                }
            }
        } => {}
        _ = tokio::signal::ctrl_c() => {
            token.cancel();
            result = Err(FavCoreError::Cancel);
        }
    }
    result
}

impl<T> SetOpsExt for T where T: SetOps {}

/// `SetOpsExt`, including methods to batch fetch and pull resources in set.
/// # Example
/// ```no_run
/// # #[path = "test_utils/mod.rs"]
/// # mod test_utils;
/// # use test_utils::data::{App, TestSet};
/// # use fav_core::{status::{Status, StatusFlags}, res::Set, ops::ResOpsExt};
/// # async {
/// let app = App::default();
/// let mut set = TestSet::default();
/// let mut sub = set.subset(|r| r.check_status(StatusFlags::TRACK));
/// app.batch_fetch_res(&mut sub);
/// # };
/// ```
pub trait ResOpsExt: ResOps {
    /// **Asynchronously** fetch resourses in set using [`ResOps::fetch_res`].
    fn batch_fetch_res<'a, S>(&self, set: &'a mut S) -> impl Future<Output = FavCoreResult<()>>
    where
        S: Set<Res = Self::Res>,
    {
        batch_op_res(set, |r, fut| self.fetch_res(r, fut))
    }

    /// **Asynchronously** pull resourses in set using [`ResOps::pull_res`].
    fn batch_pull_res<'a, S>(&self, set: &'a mut S) -> impl Future<Output = FavCoreResult<()>>
    where
        S: Set<Res = Self::Res>,
    {
        batch_op_res(set, |r, fut| self.pull_res(r, fut))
    }
}

/// A helper function to batch do operation on resources.
///
/// # Example
/// ```no_run
/// # #[path = "test_utils/mod.rs"]
/// # mod test_utils;
/// # use test_utils::data::{App, TestSet, TestRes};
/// # use fav_core::{status::{Status, StatusFlags}, res::{Set, Res}, ops::{batch_op_res, ResOps}};
/// struct Sub<'a, F: Fn(&dyn Res) -> bool> {
///     set: &'a mut TestSet,
///     f: F,
/// }
/// impl<F: Fn(&dyn Res) -> bool> Set for Sub<'_, F> {
///     type Res = TestRes;
///     fn iter(&self) -> impl Iterator<Item = &Self::Res> {
///         self.set.iter().filter(|r| (self.f)(*r))
///     }
///
///     fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Res> {
///         self.set.iter_mut().filter(|r| (self.f)(*r))
///     }
/// }
/// # async {
/// let app = App::default();
/// let mut set = TestSet::default();
/// let mut sub = Sub {
///     set: &mut set,
///     f: |r| r.check_status(StatusFlags::TRACK)
/// };
/// batch_op_res(&mut sub, |r, fut| app.fetch_res(r, fut)).await.unwrap();
/// # };
/// ```
/// However, it's better to use [`Set::subset`] and [`ResOpsExt`] instead.
/// See [`ResOpsExt`] for more information.
pub async fn batch_op_res<'a, S, F, T>(set: &'a mut S, mut f: F) -> FavCoreResult<()>
where
    S: Set + 'a,
    F: FnMut(&'a mut S::Res, WaitForCancellationFutureOwned) -> T,
    T: Future<Output = FavCoreResult<()>>,
{
    let token = CancellationToken::new();
    let mut stream = tokio_stream::iter(set.iter_mut())
        .map(|s| {
            let shutdown = token.clone().cancelled_owned();
            let fut = f(s, shutdown);
            let token = token.clone();
            async move {
                if token.is_cancelled() {
                    Err(FavCoreError::Cancel)
                } else {
                    fut.await
                }
            }
        })
        .buffer_unordered(8);
    let mut result = Ok(());
    tokio::select! {
        _ = async {
            while let Some(res) = stream.next().await {
                match res {
                    Err(FavCoreError::Cancel) => {}
                    Err(FavCoreError::NetworkError(e)) if e.is_connect() => {
                        token.cancel();  // if already cancelled, it's handled by token itself
                        result = Err(FavCoreError::NetworkError(e));
                    }
                    Err(e) => error!("{}", e),
                    _ => {}
                }
            }
        } => {}
        _ = tokio::signal::ctrl_c() => {
            token.cancel();
            result = Err(FavCoreError::Cancel);
        }
    }
    result
}

impl<T> ResOpsExt for T where T: ResOps {}

/// Making it able to perform network operations.
pub trait Net: HttpConfig + ApiProvider {
    /// Return a `&'static reqwest::Client`, use it to perform operations during the lifetime of the client.
    /// # Example
    /// ```no_run
    /// use std::sync::OnceLock;
    /// use reqwest::Client;
    /// // In `Operations`'s implementation
    /// fn client() -> &'static Client {
    ///     static CLIENT: OnceLock<Client> = OnceLock::new();
    ///     CLIENT.get_or_init(Client::new)
    /// }
    /// ```
    /// In practice, one should use [`HttpConfig`] to make a `Client` that meet the demand.
    fn client(&self) -> &'static Client {
        use std::sync::OnceLock;
        let headers = self.headers();
        static CLIENT: OnceLock<Client> = OnceLock::new();
        CLIENT.get_or_init(|| Client::builder().default_headers(headers).build().unwrap())
    }

    /// Request the api returned by [`ApiProvider::api`],
    /// and with the method returned by [`Api::method`](crate::api::Api::method)
    /// and cookie returned by [`HttpConfig::cookie_value`].
    ///
    /// Use the provided params, and client with `HttpConfig::headers`.
    fn request(
        &self,
        api_kind: Self::ApiKind,
        params: Vec<String>, // Todo make this arg more generic
    ) -> impl Future<Output = FavCoreResult<Response>> {
        async {
            let client = self.client();
            let api = self.api(api_kind);
            let cookie = self.cookie_value(api.cookie_keys());
            let resp = client
                .request(api.method(), api.url(params))
                .header(header::COOKIE, cookie)
                .send()
                .await?;
            Ok(resp)
        }
    }

    /// Serde json response from [`Self::request`] to json through [`resp2json`].
    /// pointer is the pointer to the json, see [RFC6901](https://tools.ietf.org/html/rfc6901).
    fn request_json<T>(
        &self,
        api_kind: Self::ApiKind,
        params: Vec<String>,
        pointer: &str,
    ) -> impl Future<Output = FavCoreResult<T>>
    where
        T: DeserializeOwned,
    {
        async {
            let resp = self.request(api_kind, params).await?;
            resp2json(resp, pointer).await
        }
    }

    /// Serde json response from [`Self::request_json`] to json first,
    /// then map it to protobuf msg through [`json2proto`].
    /// pointer is the pointer to the json, see [RFC6901](https://tools.ietf.org/html/rfc6901).
    fn request_proto<T>(
        &self,
        api_kind: Self::ApiKind,
        params: Vec<String>,
        pointer: &str,
    ) -> impl Future<Output = FavCoreResult<T>>
    where
        T: MessageFull,
    {
        async {
            let json = self.request_json(api_kind, params, pointer).await?;
            json2proto(&json)
        }
    }
}

impl<T> Net for T where T: HttpConfig + ApiProvider {}

/// Serde `Response` to json.
pub async fn resp2json<T>(resp: Response, pointer: &str) -> FavCoreResult<T>
where
    T: DeserializeOwned,
{
    match resp.json::<Value>().await?.pointer_mut(pointer) {
        Some(json) => {
            let ret = serde_json::from_value(json.take())?;
            Ok(ret)
        }
        None => Err(FavCoreError::SerdePointerNotFound),
    }
}

/// Map json to proto message.
pub fn json2proto<T>(json: &Value) -> FavCoreResult<T>
where
    T: MessageFull,
{
    let json = json.to_string();
    Ok(parse_from_str_with_options(&json, &PARSE_OPTIONS)?)
}

/// Map `Response` to proto message.
pub async fn resp2proto<T>(resp: Response, pointer: &str) -> FavCoreResult<T>
where
    T: MessageFull,
{
    let json = resp2json::<Value>(resp, pointer).await?;
    json2proto(&json)
}
