
#[cfg(test)]
mod tests {
    use proc_const_regex::regex;

    // const A: bool = regex!("123").test("123");
    // const B: bool = regex!("124").test("123");


    #[test]
    fn it_works() {
        regex!("123");
    }
}
