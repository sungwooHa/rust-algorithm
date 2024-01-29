pub mod problem15964 {

    pub fn problem() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();
        let a = iter.next().unwrap().parse::<i64>().unwrap();

        let b = iter.next().unwrap().parse::<i64>().unwrap();

        println!("{}", (a + b) * (a - b));
    }
}

pub mod problem2475 {
    pub fn problem() {
        println!("{}", {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let mut sum = 0;
            for _ in 0..5 {
                let number = iter.next().unwrap().parse::<i32>().unwrap();
                sum += number * number;
            }
            sum % 10
        });
    }
}
