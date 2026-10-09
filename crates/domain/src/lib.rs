pub mod settings {
    use std::time::Duration;

    pub const DEFAULT_LIMIT: u32 = 100;
    pub const WINDOW_SECONDS: u64 = 1;
    pub const RETRY_LIMIT: usize = 3;
    pub const CACHE_LIMIT: usize = 100;
    pub const CACHE_TTL: Duration = Duration::from_mins(15);
    pub const CACHE_TTI: Duration = Duration::from_mins(5);
}

mod rate_limter;
mod tools;
