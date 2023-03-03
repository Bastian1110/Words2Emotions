pub mod model;
pub mod model_interactive;

//A little playgorund to test the model without compiling it to WASM
fn main() {
    model_interactive::main().unwrap();
}
