use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ServiceLocator {
    services: HashMap<TypeId, Box<dyn Any>>
}

impl ServiceLocator {

    pub fn new() -> Self {
        Self {
            services: HashMap::new()
        }
    }

    pub fn register<T: 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|service| service.downcast_ref::<T>())
    }
}