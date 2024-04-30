use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder, Method, Request};
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub trait HttpClient<R>: Sized {
    fn construct(method: Method, path: &str) -> R;

    fn sign(self) -> R;

    fn request(self) -> Request;

    fn request_mut(&mut self) -> &mut Request;

    fn get(path: &str) -> R { Self::construct(Method::GET, path) }

    fn post(path: &str) -> R { Self::construct(Method::POST, path) }

    fn delete(path: &str) -> R {
        Self::construct(Method::DELETE, path)
    }

    fn put(path: &str) -> R {
        Self::construct(Method::PUT, path)
    }

    fn params(mut self, args: &[(&str,&str)]) -> Self {
        self.request_mut()
            .url_mut()
            .query_pairs_mut()
            .extend_pairs(args);
        self
    }

    fn body(mut self, value: Value) -> Self {
        *self.request_mut().body_mut() = Some(value.to_string().into());
        self
    }

    async fn execute<T: DeserializeOwned>(mut self) -> reqwest::Result<T> {
        static HTTP: Lazy<Client> = Lazy::new(||{
            ClientBuilder::new()
                .gzip(true)
                .tcp_keepalive(Duration::from_secs(5))
                .pool_idle_timeout(Duration::from_secs(300))
                .pool_max_idle_per_host(10)
                .timeout(Duration::from_secs(15))
                .redirect(reqwest::redirect::Policy::limited(5))
                .build().expect("create client failed")
        });
        self.request_mut()
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.request_mut()
            .headers_mut()
            .insert("Accept",HeaderValue::from_static("application/json"));

        let response = HTTP.execute(self.request()).await?;

        response.json::<T>().await
    }
}