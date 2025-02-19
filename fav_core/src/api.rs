//! API,
//! making the api easy to use

use reqwest::Method;
use url::Url;

#[cfg(feature = "derive")]
pub use fav_derive::Api;

#[allow(missing_docs)]
/// The DefaultApiKind
pub enum DefaultApiKind {
    Login,
    QrLogin,
    QrCheck,
    Logout,
    FetchResSet,
    FetchRes,
    PullRes,
}

/// The trait `ApiProvider` makes it able to provide the releted Apis that implemented [`Api`] trait.
/// # Example
/// ```
/// # use fav_core::api::{ApiProvider, Api, DefaultApiKind};
/// # use url::Url;
/// struct App;
/// struct LoginApi;
///
/// impl Api for LoginApi {
///     fn endpoint(&self) -> &'static str {
///         "http://abc.com"
///     }
///
///     fn params(&self) -> &[&str] {
///         &["id", "pwd"]
///     }
/// }
///
/// #[derive(Api)]
/// #[api(endpoint("http://abc.com"), params(&["id", "pwd"]))]
/// struct LogoutApi;
///
/// impl ApiProvider for App {
///     type ApiKind = DefaultApiKind;
///     fn api(&self, api_kind: DefaultApiKind) -> &dyn Api {
///         match api_kind {
///             DefaultApiKind::Login => &LoginApi,
///             DefaultApiKind::Logout => &LogoutApi,
///             _ => unimplemented!()
///         }
///     }
/// }
///
/// let app = App;
/// let api = app.api(DefaultApiKind::Login);
/// let url = api.url(vec!["Jake".to_string(), "123".to_string()]);
/// let expected = Url::parse_with_params("http://abc.com", vec![("id", "Jake"), ("pwd", "123")]).unwrap();
/// assert_eq!(url, expected);
pub trait ApiProvider {
    /// The ApiKind this provider can provide
    type ApiKind;
    /// Return the Api which implemented [`Api`]
    fn api(&self, api_kind: Self::ApiKind) -> &dyn Api;
}

/// The trait `Api` is the base trait for all API endpoints.
/// This trait should be object-safe.
/// # Example
/// See [`ApiProvider`].
/// For derive example, see [`fav_derive::Api`].
pub trait Api {
    /// Provide the api endpoint
    fn endpoint(&self) -> &'static str;
    /// Provide param keys needed
    fn params(&self) -> &[&str];
    /// Provide cookie keys needed
    fn cookie_keys(&self) -> &[&str] {
        &[]
    }

    /// Return a `Url` parsed with the API endpoint and the given params.
    ///
    /// The number and order of the params should match the keys provided by [`Api::params`].
    fn url(&self, params: Vec<String>) -> Url {
        Url::parse_with_params(self.endpoint(), self.params().iter().zip(params)).unwrap()
    }

    /// Return `Method::GET` by default.
    fn method(&self) -> Method {
        Method::GET
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Remote;

    impl ApiProvider for Remote {
        type ApiKind = DefaultApiKind;
        fn api(&self, api_kind: DefaultApiKind) -> &dyn Api {
            match api_kind {
                DefaultApiKind::Login => todo!(),
                _ => todo!(),
            }
        }
    }

    #[allow(dead_code)]
    struct LoginApi;

    impl Api for LoginApi {
        fn endpoint(&self) -> &'static str {
            "http://abc.com"
        }

        fn params(&self) -> &[&str] {
            &["id", "pwd"]
        }
    }
}
