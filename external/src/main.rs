#[cfg(all(feature = "use_reqwest", feature = "use_hyper"))]
// compile_error!("features `crate/a` and `crate/b` are mutually exclusive");

fn main() {
    println!("Hello, world!");
}
