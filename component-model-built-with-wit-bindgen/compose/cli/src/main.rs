mod bindings {
    wit_bindgen::generate!({});
}

use bindings::docs::calculator::calculate;
use bindings::docs::adder::add;

fn main() {
    let result = calculate::eval_expression("1 + 1");
    println!("1 + 1 = {result}");

    let sum = add::add(1,1);
    println!("1 + 1 = {sum}");
}
