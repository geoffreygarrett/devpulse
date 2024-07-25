use std::rc::Rc;

use nject::{injectable, provider};

// Define a Service trait with a serve method
trait Service {
    fn serve(&self);
}

// Implement the Service trait for a concrete type ServiceImpl
#[injectable]
struct ServiceImpl;

impl Service for ServiceImpl {
    fn serve(&self) {
        println!("Service called");
    }
}

// Define an App struct that depends on a Rc<dyn Service>
#[injectable]
struct App {
    service: Rc<dyn Service>,
}

// Define a provider for App that can create instances of ServiceImpl and Rc<dyn Service>
#[provider]
#[provide(Rc < dyn Service >, | s: ServiceImpl | Rc::new(s))]
struct AppProvider {
    // Specify that the provider can supply dyn Service by using ServiceImpl
    #[provide(dyn Service)]
    service_impl: ServiceImpl,
}

fn main() {
    // Initialize the provider with an instance of ServiceImpl
    let provider = AppProvider {
        service_impl: ServiceImpl,
    };
    // Use the provider to create an instance of App
    let app: App = provider.provide();
    // Call the serve method on the injected Service
    app.service.serve();
    println!("Service has been called successfully.");
}
