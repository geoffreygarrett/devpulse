use nject::{injectable, provider};

struct BaseDependency;

#[injectable]
struct LifetimeConsumer<'a> {
    dep: &'a BaseDependency,
}

#[provider]
struct LifetimeProvider {
    #[provide]
    base: BaseDependency,
}

fn main() {
    let provider = LifetimeProvider {
        base: BaseDependency,
    };
    let consumer: LifetimeConsumer = provider.provide();
    println!("Consumer has been initialized with a reference to the base dependency.");
}
