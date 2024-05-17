#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn greet(msg: bindings::Msg) {
        println!("hello: {}", msg.who);
    }
}

bindings::export!(Component with_types_in bindings);
