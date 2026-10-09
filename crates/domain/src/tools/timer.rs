use std::pin::Pin;

use tokio::sync::broadcast;

pub trait Timer {
    type Subscribtion: Send;

    fn tick(&mut self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn subscribe(&self) -> Self::Subscribtion;
}

pub struct TimerMan {
    interval: tokio::time::Interval,
    state: tokio::sync::broadcast::Sender<()>,
}

impl Timer for TimerMan {
    type Subscribtion = broadcast::Receiver<()>;

    fn tick(&mut self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            self.interval.tick().await;
            let _ = self.state.send(());
        })
    }

    fn subscribe(&self) -> Self::Subscribtion {
        self.state.subscribe()
    }
}
