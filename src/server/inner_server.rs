use ::anyhow::anyhow;
use ::anyhow::Context;
use ::anyhow::Result;
use ::cookie::Cookie;
use ::cookie::CookieJar;
use ::hyper::http::HeaderValue;
use ::hyper::http::Method;
use ::hyper::http::Uri;
use ::hyper::http::Error as HttpError;
use ::std::sync::Arc;
use ::std::sync::Mutex;

use crate::Request;
use crate::RequestConfig;
use crate::RequestDetails;

/// The `InnerServer` is the real server that runs.
#[derive(Debug)]
pub(crate) struct InnerServer {
    server_address: Uri,
    cookies: CookieJar,
    save_cookies: bool,
}

impl InnerServer {
    /// Creates a `Server` running your app on the address given.
    pub(crate) fn new<U>(uri: U) -> Result<Self>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<HttpError>,
    {
        let server_address = uri.try_into().with_context(|| "Failed to parse server address URI")?;
        let test_server = Self {
            server_address: uri.try_into()?,
            cookies: CookieJar::new(),
            save_cookies: false,
        };

        Ok(test_server)
    }

    pub(crate) fn server_address<'a>(&'a self) -> &'a Uri {
        &self.server_address
    }

    pub(crate) fn cookies<'a>(&'a self) -> &'a CookieJar {
        &self.cookies
    }

    /// Adds the given cookies.
    ///
    /// They will be stored over the top of the existing cookies.
    pub(crate) fn add_cookies_by_header<'a, I>(
        this: &mut Arc<Mutex<Self>>,
        cookie_headers: I,
    ) -> Result<()>
    where
        I: Iterator<Item = &'a HeaderValue>,
    {
        InnerServer::with_this_mut(this, "add_cookies_by_header", |this| {
            for cookie_header in cookie_headers {
                let cookie_header_str = cookie_header
                    .to_str()
                    .context(&"Reading cookie header for storing in the `Server`")
                    .unwrap();

                let cookie: Cookie<'static> = Cookie::parse(cookie_header_str)?.into_owned();
                this.cookies.add(cookie);
            }

            Ok(()) as Result<()>
        })?
    }

    /// Adds the given cookies.
    ///
    /// They will be stored over the top of the existing cookies.
    pub(crate) fn clear_cookies(this: &mut Arc<Mutex<Self>>) -> Result<()> {
        InnerServer::with_this_mut(this, "clear_cookies", |this| {
            this.cookies = CookieJar::new();
        })
    }

    /// Adds the given cookies.
    ///
    /// They will be stored over the top of the existing cookies.
    pub(crate) fn add_cookies(this: &mut Arc<Mutex<Self>>, cookies: CookieJar) -> Result<()> {
        InnerServer::with_this_mut(this, "add_cookies", |this| {
            for cookie in cookies.iter() {
                this.cookies.add(cookie.to_owned());
            }
        })
    }

    pub(crate) fn add_cookie(this: &mut Arc<Mutex<Self>>, cookie: Cookie) -> Result<()> {
        InnerServer::with_this_mut(this, "add_cookie", |this| {
            this.cookies.add(cookie.into_owned());
        })
    }

    pub(crate) fn request_config(this: &Arc<Mutex<Self>>) -> Result<RequestConfig> {
        InnerServer::with_this(this, "request_config", |this| RequestConfig {
            save_cookies: this.save_cookies,
        })
    }

    pub(crate) fn send(this: &Arc<Mutex<Self>>, method: Method, path: &str) -> Result<Request> {
        let config = InnerServer::request_config(this)?;

        Request::new(
            this.clone(),
            config,
            RequestConfig {
                method,
                path: path.to_string(),
                save_cookies: InnerServer::coo
            },
        )
    }

    pub(crate) fn with_this<F, R>(this: &Arc<Mutex<Self>>, name: &str, some_action: F) -> Result<R>
    where
        F: FnOnce(&mut Self) -> R,
    {
        let mut this_locked = this.lock().map_err(|err| {
            anyhow!(
                "Failed to lock InternalServer for `{}`, {:?}",
                name,
                err,
            )
        })?;

        let result = some_action(&mut this_locked);

        Ok(result)
    }

    pub(crate) fn with_this_mut<F, R>(
        this: &mut Arc<Mutex<Self>>,
        name: &str,
        some_action: F,
    ) -> Result<R>
    where
        F: FnOnce(&mut Self) -> R,
    {
        let mut this_locked = this.lock().map_err(|err| {
            anyhow!(
                "Failed to lock InternalServer for `{}`, {:?}",
                name,
                err,
            )
        })?;

        let result = some_action(&mut this_locked);

        Ok(result)
    }
}
