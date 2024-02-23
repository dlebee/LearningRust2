use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub enum Registration {
    Transient(Box<dyn Fn() -> Box<dyn Any>>),
    Singleton(Box<dyn Fn() -> Box<dyn Any>>),
    AsyncSingleton(fn() -> Pin<Box<dyn Future<Output = Box<dyn Any>> + Send>>),
    AsyncTransient(fn() -> Pin<Box<dyn Future<Output = Box<dyn Any>> + Send>>),
}

pub struct ServiceCollection {
    service_types: HashMap<TypeId, Registration>,
}

impl ServiceCollection {

    pub fn new() -> Self {
        Self {
            service_types: HashMap::new()
        }
    }

    pub fn register_transient<T>(&mut self, factory: fn() -> T) -> &Self {
        let type_id = TypeId::of::<T>();
        self.service_types.insert(type_id, Registration::Transient(Box::new(|| Box::new(factory()))));
        self
    }

    pub fn register_async_transient<T>(&mut self, async_factory: fn() -> Pin<Box<dyn Future<Output = Box<dyn Any>> + Send>>) -> &Self {
        let type_id = TypeId::of::<T>();
        self.service_types.insert(type_id, Registration::AsyncTransient(async_factory));
        self
    }

    pub fn register_singleton<T>(&mut self, factory: fn() -> T) -> &Self {
        let type_id = TypeId::of::<T>();
        self.service_types.insert(type_id, Registration::Singleton(Box::new(|| Box::new(factory()))));
        self
    }

    pub fn register_async_singleton<T>(&mut self, async_factory: fn() -> Pin<Box<dyn Future<Output = Box<dyn Any>> + Send>>) -> &Self {
        let type_id = TypeId::of::<T>();
        self.service_types.insert(type_id, Registration::AsyncSingleton(async_factory));
        self
    }
}
