use ::hyper::http::Method;

#[derive(Debug, Clone)]
pub struct RequestDetails {
    pub method: Method,
    pub path: String,
}
