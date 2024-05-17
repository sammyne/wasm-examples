#[allow(warnings)]
mod bindings;

use bindings::Guest;

use bindings::helloworld::cli::api::Greeter;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        let who = "sammyne";

        {
            // greeter 出了作用域后，会触发对应资源的析构函数
            let greeter = Greeter::new();
            greeter.greet(who);
        }
        println!("this is guest");

        "hello world".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
