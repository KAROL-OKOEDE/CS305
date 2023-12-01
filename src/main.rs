///associate greetings module with this crate
mod greetings;
mod how_to_hold_data_for_operations;
use how_to_hold_data_for_operations::primitive;
use how_to_hold_data_for_operations::derived;
use primitive::{compound, scalar};
extern crate hello_world_lib;

///Optionally load each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
use greetings:: {spanish, french, english};
fn main() {
println!("Hello, world!");
println!(
    "My first greeting is '{}' and my second is '{}'",
    english::default_greeting(),
   english:: default_greeting2(),
);

println!("{}", english::default_greeting());
println!("{}", spanish::default_greeting());
println!("{}", french::default_greeting());
println!("{}", hello_world_lib::greeting_from_lib());
//scalar::scalar();
compound::main();
derived::triangle::run2();
derived::circle::run2();

}