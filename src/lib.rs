pub fn hello(name: &str) -> String {
    let hello_string = format!("Hello, {}!", name);
    hello_string
}

pub fn goodbye(name: &str) -> String {
    let goodbye_string = format!("Goodbye, {}!", name);
    goodbye_string
}
