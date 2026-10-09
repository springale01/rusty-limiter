use std::{
    any::Any,
    collections::VecDeque,
    marker::PhantomData,
    sync::{Arc, OnceLock, atomic::AtomicUsize},
};

use moka::ops::compute::Op;

use crate::{settings, tools::timer::Timer};

pub struct RateLimiter<T, S> {
    counter: AtomicUsize,
    cache: moka::sync::Cache<uuid::Uuid, u8>,
    _marker: PhantomData<fn() -> T>,
    timer: S,
}

pub struct AtLimit;
pub struct Avalible;

impl<T> RateLimiter<T, tokio::sync::broadcast::Receiver<()>> {
    async fn recive<P: Send>(&self, req: IncomingReqeust<P>) -> bool {
        let IncomingReqeust { id, package } = req;
        let mut removed_flag = false;
        let thing = self.cache.entry(id).and_compute_with(|v| {
            let Some(entry) = v else {
                return Op::Put(1);
            };
            let mut value = *entry.value();

            if entry.value() >= &3u8 {
                removed_flag = true;
                return Op::Remove;
            } else {
                return Op::Put(value + 1);
            }
        });
        todo!()
    }
}

impl<T, S> RateLimiter<T, S> {
    fn new(timer: S) -> Self {
        let cache = moka::sync::Cache::builder()
            .max_capacity(settings::CACHE_LIMIT as u64)
            .time_to_idle(settings::CACHE_TTI)
            .time_to_live(settings::CACHE_TTL)
            .name("PineappleCache")
            .build();
        Self {
            counter: AtomicUsize::new(0),
            cache: cache,
            _marker: PhantomData,
            timer,
        }
    }
}

pub struct IncomingReqeust<P>
where
    P: Send,
{
    id: uuid::Uuid,
    package: P,
}
