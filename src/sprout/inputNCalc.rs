pub mod problem1000{
    pub fn problem() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        println!("{}", a + b);
    }
}

pub mod problem10998{
    use std::io::Read;

    pub fn problem() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        println!("{}", a * b);
    }
}

pub mod problem10869{
    use std::io::Read;
    pub fn problem() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        println!("{}", a + b);
        println!("{}", a - b);
        println!("{}", a * b);
        println!("{}", a / b);
        println!("{}", a % b);
    }
}

pub mod problem1008{
    use std::io::Read;
    pub fn problem() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: f64 = iter.next().unwrap().parse().unwrap();
        let b: f64 = iter.next().unwrap().parse().unwrap();
        println!("{}", a / b);
    }
}

pub mod problem11382{
    use std::io::Read;
    pub fn problem() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();
        println!("{}", a + b + c);
    }
}