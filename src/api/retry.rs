use std::future::Future;

use tokio::time::sleep;

use crate::error::ApiError;

use super::rate_limit::calculate_retry_delay;

pub async fn with_retry<F, Fut, T>(
    max_attempts: u32,
    mut operation: F,
) -> std::result::Result<T, ApiError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = std::result::Result<T, ApiError>>,
{
    let mut attempt = 0;
    loop {
        attempt += 1;
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                if attempt >= max_attempts || !error.is_retryable() {
                    return Err(ApiError::RetriesExhausted(format!(
                        "{error}; attempts={attempt}"
                    )));
                }

                sleep(calculate_retry_delay(attempt, None, None)).await;
            }
        }
    }
}
