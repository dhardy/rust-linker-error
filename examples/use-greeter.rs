extern crate linker_error;

use linker_error::make_greeter;

fn main() {
    let greeter = make_greeter();
    greeter.greet();
}
