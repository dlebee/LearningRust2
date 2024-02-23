use crate::service_collection::ServiceCollection;

mod service_collection;

#[derive(Debug)]
pub struct AService {
    name: String
}

impl AService {
    fn new() -> Self {
        Self{
            name: String::from("AService")
        }
    }
}

#[derive(Debug)]
pub struct BService {
    a: AService
}

impl BService {
    fn new(a_service: AService) -> Self {
        Self {
            a: a_service
        }
    }
}

fn main() {

    let mut services = ServiceCollection::new();
    services.register_transient(AService::new);
}
