#[allow(warnings)]
mod bindings;

use bindings::sammyne::hello2::api::{self, Msg};

fn main() {
    let msg = Msg {
        who: "sammyne".to_string(),
    };
    api::greet_twice(&msg);
}
