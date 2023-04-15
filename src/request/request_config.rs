#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub save_cookies: bool,
    pub content_type: Option<String>,
}
