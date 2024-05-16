#[allow(warnings)]
mod bindings;

use bindings::Guest;

use bindings::helloworld::cli::api::Greeter;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        let who = "sammyne";

        let greeter = Greeter::new();
        greeter.greet(who);

        "hello world".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
