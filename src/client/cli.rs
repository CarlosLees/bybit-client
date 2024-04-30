use once_cell::sync::Lazy;
use reqwest::{Method, Request, Url};

use crate::client::http::HttpClient;

pub struct Cli(Request);

impl HttpClient<Cli> for Cli {
    fn construct(method: Method, path: &str) -> Cli {
        static URL:Lazy<Url> = Lazy::new(|| Url::parse("https://console.sdhis999.com").unwrap());
        let mut url = URL.clone();
        url.set_path(format!("{path}").as_str());
        let mut request = Request::new(method, url);
        request.headers_mut().append("locale", "en-US".parse().unwrap());
        Cli(request)
    }

    fn sign(self) -> Cli {
        todo!()
    }

    fn request(self) -> Request {
        self.0
    }

    fn request_mut(&mut self) -> &mut Request {
        &mut self.0
    }
}