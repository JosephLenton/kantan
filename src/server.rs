use ::anyhow::Context;
use ::anyhow::Result;
use ::cookie::Cookie;
use ::cookie::CookieJar;
use ::hyper::http::Method;
use ::hyper::http::Uri;
use ::hyper::http::Error as HttpError;
use ::std::sync::Arc;
use ::std::sync::Mutex;

use crate::Request;

mod inner_server;
pub(crate) use self::inner_server::*;

///
/// The `Server` represents your application, running as a web server,
/// and you can make web requests to your application.
///
/// For most people's needs, this is where to start when writing a test.
/// This allows you Allowing you to create new requests that will go to this server.
///
/// You can make a request against the `Server` by calling the
/// `get`, `post`, `put`, `delete`, and `patch` methods (you can also use `method`).
///
#[derive(Debug)]
pub struct Server {
    inner: Arc<Mutex<InnerServer>>,
}

impl Server {
    /// This will take the given app, and run it.
    /// It will use a randomly selected port for running.
    ///
    /// This is the same as creating a new `Server` with a configuration,
    /// and passing `ServerConfig::default()`.
    pub fn new<U>(uri: U) -> Result<Self>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<HttpError>,
    {
        let inner_test_server = InnerServer::new(uri)?;
        let inner_mutex = Mutex::new(inner_test_server);
        let inner = Arc::new(inner_mutex);

        Ok(Self { inner })
    }

    /// Clears all of the cookies stored internally.
    pub fn clear_cookies(&mut self) {
        InnerServer::clear_cookies(&mut self.inner)
            .with_context(|| format!("Trying to clear_cookies"))
            .unwrap()
    }

    /// Adds extra cookies to be used on *all* future requests.
    ///
    /// Any cookies which have the same name as the new cookies,
    /// will get replaced.
    pub fn add_cookies(&mut self, cookies: CookieJar) {
        InnerServer::add_cookies(&mut self.inner, cookies)
            .with_context(|| format!("Trying to add_cookies"))
            .unwrap()
    }

    /// Adds a cookie to be included on *all* future requests.
    ///
    /// If a cookie with the same name already exists,
    /// then it will be replaced.
    pub fn add_cookie(&mut self, cookie: Cookie) {
        InnerServer::add_cookie(&mut self.inner, cookie)
            .with_context(|| format!("Trying to add_cookie"))
            .unwrap()
    }

    /// Creates a HTTP GET request to the path.
    pub fn get(&self, path: &str) -> Request {
        self.method(Method::GET, path)
    }

    /// Creates a HTTP POST request to the given path.
    pub fn post(&self, path: &str) -> Request {
        self.method(Method::POST, path)
    }

    /// Creates a HTTP PATCH request to the path.
    pub fn patch(&self, path: &str) -> Request {
        self.method(Method::PATCH, path)
    }

    /// Creates a HTTP PUT request to the path.
    pub fn put(&self, path: &str) -> Request {
        self.method(Method::PUT, path)
    }

    /// Creates a HTTP DELETE request to the path.
    pub fn delete(&self, path: &str) -> Request {
        self.method(Method::DELETE, path)
    }

    /// Creates a HTTP request, to the path given, using the given method.
    pub fn method(&self, method: Method, path: &str) -> Request {
        let debug_method = method.clone();
        InnerServer::send(&self.inner, method, path)
            .with_context(|| {
                format!(
                    "Trying to create internal request for {} {}",
                    debug_method, path
                )
            })
            .unwrap()
    }
}
