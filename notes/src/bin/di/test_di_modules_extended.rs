// use nject::{injectable, provider};
//
// mod internal {
//     use nject::{injectable, module};
//
//     #[injectable]
//     #[derive(Clone)]
//     pub struct InternalDependency {
//         pub message: &'static str,
//     }
//
//     #[injectable]
//     #[module]
//     pub struct InternalModule {
//         #[export]
//         pub internal_dep: InternalDependency,
//     }
// }
//
// // Define a Logger service that depends on InternalDependency
// #[injectable]
// struct Logger {
//     dep: internal::InternalDependency,
// }
//
// impl Logger {
//     fn log(&self) {
//         println!("Logging message: {}", self.dep.message);
//     }
// }
//
// // Define the App struct that depends on Logger
// #[injectable]
// struct App {
//     logger: Logger,
// }
//
// // Define the provider for App
// #[provider]
// #[provide(Logger, |internal_mod: &internal::InternalModule| Logger {
// dep: internal_mod.internal_dep.clone(),
// })]
// struct AppProvider {
//     #[import]
//     internal_mod: internal::InternalModule,
// }
//
// fn main() {
//     // Initialize the internal dependency and module
//     let internal_dependency = internal::InternalDependency {
//         message: "Hello from InternalDependency",
//     };
//     let internal_module = internal::InternalModule {
//         internal_dep: internal_dependency,
//     };
//
//     // Initialize the provider with the internal module
//     let provider = AppProvider {
//         internal_mod: internal_module,
//     };
//
//     // Use the provider to create an instance of App
//     let app: App = provider.provide();
//
//     // Use the logger service within the app
//     app.logger.log();
//     println!("App has been initialized and the logger service has been used.");
// }

fn main() {}