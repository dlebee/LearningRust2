use crate::service_locator::ServiceLocator;

mod service_locator;


#[derive(Debug)]
struct AService {
    yay: String
}

#[derive(Debug)]
struct BService {
    name: String
}

fn main() {

    let mut locator = ServiceLocator::new();

    locator.register(AService{ yay: String::from("yay") });
    locator.register(BService { name: String::from("b") });

    let a_service_resolved = locator.resolve::<AService>();
    if let Some(a_service) = a_service_resolved {
        println!("{:?}", *a_service);
    }
}
