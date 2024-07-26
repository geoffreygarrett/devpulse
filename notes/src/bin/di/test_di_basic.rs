use nject::{injectable, provider};

#[injectable]
struct Dependency;

#[injectable]
struct Consumer {
    dep: Dependency,
}

#[provider]
struct AppProvider;

fn main() {
    let consumer: Consumer = AppProvider.provide();
    assert!(true, "Consumer should be initialized with its dependencies.");
}
