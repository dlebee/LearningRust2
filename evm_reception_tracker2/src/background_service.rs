use std::future::Future;
use std::pin::Pin;

pub trait BackgroundService {
    fn cleanup(&self) -> Pin<Box<dyn Future<Output = ()>>>;
}