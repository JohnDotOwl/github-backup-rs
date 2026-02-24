use std::time::Duration;

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, LINK, USER_AGENT};
use serde::de::DeserializeOwned;

use crate::{
    api::pagination::parse_next_link,
    config::RuntimeConfig,
    error::{ApiError, Result},
};

#[derive(Clone)]
pub struct GitHubClient {
    http: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl GitHubClient {
    pub fn from_runtime(runtime: &RuntimeConfig, token: Option<String>) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(runtime.request_timeout_seconds))
            .build()
            .map_err(ApiError::from)?;

        Ok(Self {
            http,
            base_url: runtime.api_base_url.clone(),
            token,
        })
    }

    pub async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> std::result::Result<T, ApiError> {
        let (value, _) = self.get_json_with_headers(path).await?;
        Ok(value)
    }

    pub async fn get_paginated<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> std::result::Result<Vec<T>, ApiError> {
        let mut url = self.build_url(path);
        let mut items = Vec::new();

        loop {
            let (page, headers) = self.get_json_from_url_with_headers::<Vec<T>>(&url).await?;
            items.extend(page);

            let next_link = headers
                .get(LINK)
                .and_then(|value| value.to_str().ok())
                .and_then(parse_next_link);

            if let Some(next_link) = next_link {
                url = next_link;
            } else {
                break;
            }
        }

        Ok(items)
    }

    pub async fn get_json_with_headers<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> std::result::Result<(T, HeaderMap), ApiError> {
        let url = self.build_url(path);
        self.get_json_from_url_with_headers(&url).await
    }

    fn build_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    async fn get_json_from_url_with_headers<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> std::result::Result<(T, HeaderMap), ApiError> {
        let mut request = self
            .http
            .get(url)
            .header(
                USER_AGENT,
                format!("github-backup-rs/{}", env!("CARGO_PKG_VERSION")),
            )
            .header(ACCEPT, "application/vnd.github+json");

        if let Some(token) = &self.token {
            request = request.header(AUTHORIZATION, format!("token {token}"));
        }

        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(ApiError::UnexpectedStatus {
                status,
                message: response
                    .text()
                    .await
                    .unwrap_or_else(|_| "<failed to read error body>".to_string()),
            });
        }

        let headers = response.headers().clone();
        let value = response.json::<T>().await.map_err(ApiError::from)?;
        Ok((value, headers))
    }
}
