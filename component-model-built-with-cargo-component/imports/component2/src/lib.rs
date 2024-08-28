#[allow(warnings)]
mod bindings;

use crate::bindings::exports::sammyne::hello2::api::Guest;
use crate::bindings::sammyne::hello1::types::Msg;

struct Component;

impl Guest for Component {
    fn greet_twice(m: Msg) {
        println!("hello {}", m.who);
        println!("hello {}", m.who);
    }
}

bindings::export!(Component with_types_in bindings);
