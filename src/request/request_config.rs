use ::hyper::http::Method;

#[derive(Debug, Clone)]
pub(crate) struct RequestConfig {

    pub method: Method,
    pub path: String,
    pub save_cookies: bool,
}
