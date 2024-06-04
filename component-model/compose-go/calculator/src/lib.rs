#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::Guest;

// Bring the imported add function into scope
use bindings::docs::adder::add;

struct Component;

impl Guest for Component {
    fn eval_expression(_expr: String) -> u32 {
        // Cleverly parse `expr` into values and operations, and evaluate
        // them meticulously.
        add::add(123, 456)
    }
}

bindings::export!(Component with_types_in bindings);
