use std::future::Future;
use std::time::Duration;

use crate::log_info;

#[derive(Clone, Debug)]
pub enum BackoffStrategy {
    Fixed,
    Linear,
    Exponential,
}

#[derive(Clone, Debug)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub delay_ms: u64,
    pub backoff: BackoffStrategy,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            delay_ms: 500,
            backoff: BackoffStrategy::Fixed,
        }
    }
}

impl RetryConfig {
    pub fn new(max_retries: u32, delay_ms: u64, backoff: BackoffStrategy) -> Self {
        Self {
            max_retries,
            delay_ms,
            backoff,
        }
    }

    fn delay_duration(&self, attempt: u32) -> Duration {
        Duration::from_millis(self.delay_ms(attempt))
    }

    fn delay_ms(&self, attempt: u32) -> u64 {
        match self.backoff {
            BackoffStrategy::Fixed => self.delay_ms,
            BackoffStrategy::Linear => self.delay_ms * (attempt as u64),
            BackoffStrategy::Exponential => self.delay_ms * 2u64.pow(attempt.saturating_sub(1)),
        }
    }
}

pub fn retry<F, T, E>(f: F) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
    E: std::fmt::Display,
{
    retry_with_config(f, &RetryConfig::default())
}

pub fn retry_with_config<F, T, E>(f: F, config: &RetryConfig) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut last_err = None;

    for attempt in 1..=config.max_retries {
        match f() {
            Ok(val) => return Ok(val),
            Err(e) => {
                log_info!(
                    "Retry",
                    "Attempt {}/{} failed: {}",
                    attempt,
                    config.max_retries,
                    e
                );
                last_err = Some(e);

                if attempt < config.max_retries {
                    std::thread::sleep(config.delay_duration(attempt));
                }
            }
        }
    }

    Err(last_err.expect("retry_with_config: no error captured"))
}

pub async fn retry_async<F, Fut, T, E, S, SFut>(f: F, sleep: S) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    S: Fn(Duration) -> SFut,
    SFut: Future<Output = ()>,
    E: std::fmt::Display,
{
    retry_async_with_config(f, &RetryConfig::default(), sleep).await
}

pub async fn retry_async_with_config<F, Fut, T, E, S, SFut>(
    f: F,
    config: &RetryConfig,
    sleep: S,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    S: Fn(Duration) -> SFut,
    SFut: Future<Output = ()>,
    E: std::fmt::Display,
{
    let mut last_err = None;

    for attempt in 1..=config.max_retries {
        match f().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                log_info!(
                    "Retry",
                    "Attempt {}/{} failed: {}",
                    attempt,
                    config.max_retries,
                    e
                );
                last_err = Some(e);

                if attempt < config.max_retries {
                    sleep(config.delay_duration(attempt)).await;
                }
            }
        }
    }

    Err(last_err.expect("retry_async_with_config: no error captured"))
}
