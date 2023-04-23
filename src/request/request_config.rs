use ::hyper::http::Method;
use ::hyper::Uri;

#[derive(Debug, Clone)]
pub(crate) struct RequestConfig {
    pub method: Method,
    pub request_path: Uri,
    pub save_cookies: bool,
    pub content_type: Option<String>,
}
