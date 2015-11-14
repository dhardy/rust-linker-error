// Issue is that Greeter is not publically exported. Replace 'use' with 'pub use' to fix.
use detail::Greeter;

mod detail {
    pub struct Greeter;
    
    impl Greeter {
        pub fn greet(&self) {
            println!("Hello!");
        }
    }
}

pub fn make_greeter() -> Greeter {
    Greeter
}

#[test]
fn internal_usage() {
    let greeter = make_greeter();
    greeter.greet();
}
