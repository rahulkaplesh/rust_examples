fn printreturn(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn main() {
    printreturn(15);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = printreturn(12);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = printreturn(12);
        assert_eq!(12, value);
    }
}