use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

/// Configuration for an AI model with rate limits
#[derive(Debug, Clone)]
pub struct AiModelConfig {
    pub name: String,
    pub rpm_limit: u32,               // Requests per minute
    pub tpm_limit: u32,               // Tokens per minute
    pub rpd_limit: u32,               // Requests per day
    pub assumed_words_per_token: f32, // e.g., 0.75 for Gemini
}

/// Tracks usage for rate limiting
#[derive(Debug)]
pub struct RateLimiter {
    pub(crate) config: AiModelConfig,
    request_timestamps: VecDeque<Instant>, // For RPM and RPD
    token_usage: VecDeque<(Instant, u32)>, // (timestamp, tokens_used) for TPM
}

impl RateLimiter {
    pub fn new(config: AiModelConfig) -> Self {
        Self {
            config,
            request_timestamps: VecDeque::new(),
            token_usage: VecDeque::new(),
        }
    }

    /// Calculate estimated tokens from word count
    pub fn estimate_tokens(&self, word_count: usize) -> u32 {
        ((word_count as f32) / self.config.assumed_words_per_token) as u32
    }

    /// Check if we can make a request with the given token count
    pub fn can_request(&mut self, token_count: u32) -> bool {
        let now = Instant::now();

        // Clean old entries (older than 1 minute for RPM/TPM, 24 hours for RPD)
        self.clean_old_entries(now);

        // Check RPM
        if self.request_timestamps.len() >= self.config.rpm_limit as usize {
            return false;
        }

        // Check TPM
        let recent_tokens: u32 = self.token_usage.iter().map(|(_, tokens)| tokens).sum();
        if recent_tokens + token_count > (self.config.tpm_limit as f32 * 0.8) as u32 {
            return false;
        }

        // Check RPD (simplified - just count requests in last 24h)
        let _one_day_ago = now - Duration::from_secs(86400);
        let recent_requests = self
            .request_timestamps
            .iter()
            .filter(|&&t| t > _one_day_ago)
            .count();
        if recent_requests >= self.config.rpd_limit as usize {
            return false;
        }

        true
    }

    /// Record a request with token usage
    pub fn record_request(&mut self, token_count: u32) {
        let now = Instant::now();
        self.request_timestamps.push_back(now);
        self.token_usage.push_back((now, token_count));
    }

    /// Wait until we can make a request (respects RPM limit)
    pub async fn wait_for_request(&mut self, token_count: u32) {
        loop {
            if self.can_request(token_count) {
                break;
            }

            // Sleep for a short time and check again
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Clean entries older than the relevant time windows
    fn clean_old_entries(&mut self, now: Instant) {
        let one_minute_ago = now - Duration::from_secs(60);
        let _one_day_ago = now - Duration::from_secs(86400);

        // Clean RPM entries (keep only last minute)
        while let Some(&front) = self.request_timestamps.front() {
            if front < one_minute_ago {
                self.request_timestamps.pop_front();
            } else {
                break;
            }
        }

        // Clean TPM entries (keep only last minute)
        while let Some(&(timestamp, _)) = self.token_usage.front() {
            if timestamp < one_minute_ago {
                self.token_usage.pop_front();
            } else {
                break;
            }
        }

        // For RPD, we clean in can_request as needed
    }
}

/// Thread-safe wrapper for RateLimiter
pub type SharedRateLimiter = Arc<Mutex<RateLimiter>>;

pub fn create_rate_limiter(config: AiModelConfig) -> SharedRateLimiter {
    Arc::new(Mutex::new(RateLimiter::new(config)))
}
