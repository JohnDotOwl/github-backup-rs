use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn calculate_retry_delay(
    attempt: u32,
    retry_after_seconds: Option<u64>,
    reset_epoch: Option<u64>,
) -> Duration {
    if let Some(seconds) = retry_after_seconds {
        return Duration::from_secs(seconds.max(1));
    }

    if let Some(reset) = reset_epoch {
        let now_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if reset > now_epoch {
            return Duration::from_secs(reset - now_epoch);
        }
    }

    let capped_attempt = attempt.min(10);
    let base = 2_u64.saturating_pow(capped_attempt);
    let jitter = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_millis() as u64
        % 400;

    Duration::from_millis(base.saturating_mul(1000).saturating_add(jitter))
}
