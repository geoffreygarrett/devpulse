use nject::{injectable, provider};

mod internal {
    use nject::injectable;
    use nject::module;

    #[injectable]
    pub struct InternalDependency;

    #[injectable]
    #[module]
    pub struct InternalModule {
        #[export]
        pub internal_dep: InternalDependency,
    }
}

#[injectable]
#[provider]
struct AppProvider {
    #[import]
    internal_mod: internal::InternalModule,
}

fn main() {
    let provider = AppProvider { internal_mod: internal::InternalModule { internal_dep: internal::InternalDependency } };
    println!("Internal module has been initialized and exported successfully.");
}
