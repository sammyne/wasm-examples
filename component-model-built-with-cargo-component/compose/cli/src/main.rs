#[allow(warnings)]
mod bindings;

use bindings::docs::calculator::calculate;

fn main() {
    let result = calculate::eval_expression("1 + 1");
    println!("1 + 1 = {result}");
}
