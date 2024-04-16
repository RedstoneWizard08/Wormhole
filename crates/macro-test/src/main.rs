#[macros::serde_call]
pub async fn something() {}

#[macros::serde_call]
pub async fn something_two(a: String, b: i32, c: String) -> bool {
    false
}

macros::serde_funcs!(something, something_two);

#[tokio::main]
pub async fn main() {
    // println!("{}", __serde_invoke_something_two("{\"a\": \"\", \"b\": 1, \"c\": \"\"}".into()));
    let val = handle_serde_call("something_two", "{\"a\": \"\", \"b\": 1, \"c\": \"\"}").await;

    println!("{}", val);
}
