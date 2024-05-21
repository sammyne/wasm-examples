#[allow(warnings)]
mod bindings;

use bindings::component::example::greeter;
use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        const WHO: &str = "sammyne";

        greeter::greet(WHO);

        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
