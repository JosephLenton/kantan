use ::anyhow::anyhow;
use ::anyhow::Context;
use ::anyhow::Result;
use ::cookie::Cookie;
use ::cookie::CookieJar;
use ::hyper::http::HeaderValue;
use ::hyper::http::Method;
use ::hyper::http::Uri;
use ::std::sync::Arc;
use ::std::sync::Mutex;

use crate::Request;
use crate::RequestConfig;

/// The `InnerServer` is the real server that runs.
#[derive(Debug)]
pub(crate) struct InnerServer {
    server_address: String,
    cookies: CookieJar,
    save_cookies: bool,
    default_content_type: Option<String>,
}

impl InnerServer {
    /// Creates a `Server` running your app on the address given.
    pub(crate) fn new(server_address: String) -> Result<Self> {
        let test_server = Self {
            server_address,
            cookies: CookieJar::new(),
            save_cookies: false,
            default_content_type: None,
        };

        Ok(test_server)
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

    pub(crate) fn build_request_config(
        this: &Arc<Mutex<Self>>,
        method: Method,
        path: &str,
    ) -> Result<RequestConfig> {
        InnerServer::with_this(this, "request_config", |this| {
            let request_path = build_request_path(&this.server_address, path)?;
            let config = RequestConfig {
                method,
                request_path,
                save_cookies: this.save_cookies,
                content_type: this.default_content_type.clone(),
            };

            Ok(config)
        })?
    }

    pub(crate) fn send(this: &Arc<Mutex<Self>>, method: Method, path: &str) -> Result<Request> {
        let config = InnerServer::build_request_config(this, method, path)?;

        Request::new(this.clone(), config)
    }

    pub(crate) fn with_this<F, R>(this: &Arc<Mutex<Self>>, name: &str, some_action: F) -> Result<R>
    where
        F: FnOnce(&mut Self) -> R,
    {
        let mut this_locked = this
            .lock()
            .map_err(|err| anyhow!("Failed to lock InternalServer for `{}`, {:?}", name, err,))?;

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
        let mut this_locked = this
            .lock()
            .map_err(|err| anyhow!("Failed to lock InternalServer for `{}`, {:?}", name, err,))?;

        let result = some_action(&mut this_locked);

        Ok(result)
    }
}

fn build_request_path(root: &str, sub_path: &str) -> Result<Uri> {
    if sub_path.is_empty() {
        return Ok(root.try_into()?);
    }

    if sub_path.starts_with("/") {
        let full_path = format!("{}{}", root, sub_path).try_into()?;
        return Ok(full_path);
    }

    let full_path = format!("{}/{}", root, sub_path).try_into()?;
    Ok(full_path)
}
