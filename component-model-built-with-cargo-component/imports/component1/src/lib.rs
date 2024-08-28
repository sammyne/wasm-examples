#[allow(warnings)]
mod bindings;

use crate::bindings::{Guest, Msg};

struct Component;

impl Guest for Component {
    fn greet(m: Msg) {
        println!("hello {}", m.who);
    }
}

bindings::export!(Component with_types_in bindings);
