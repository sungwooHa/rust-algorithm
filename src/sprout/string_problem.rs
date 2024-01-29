pub mod problem11654 {

    pub fn problem() {
        println!("{}", {
            let mut input = String::new();

            std::io::stdin().read_line(&mut input).unwrap();

            input.as_bytes()[0]
        });
    }
}
