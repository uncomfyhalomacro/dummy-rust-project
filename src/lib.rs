pub fn hello(name: &str) -> String {
    let hello_string = format!("Hello, {}!", name);
    hello_string
}

pub fn goodbye(name: &str) -> String {
    let goodbye_string = format!("Goodbye, {}!", name);
    goodbye_string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hello_stackie_matches() {
        let hello_string = hello("Stackie");
        assert_eq!("Hello, Stacker!", &hello_string)
    }

    #[test]
    fn goodbye_stackie_matches() {
        let goodbye_string = goodbye("Stackie");
        assert_eq!("Goodbye, Stackie!", &goodbye_string)
    }
}
