#[allow(warnings)]
mod bindings {
    wit_bindgen::generate!({
        world: "example",
    });
}

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
