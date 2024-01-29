pub fn problem2440() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    for i in (0..number).rev() {
        for _ in 0..i + 1 {
            print!("*");
        }
        println!();
    }
}

pub fn problem23795() {
    let mut sum = 0;
    while true {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let number: i32 = input.trim().parse().unwrap();
        if number == -1 {
            println!("{}", sum);
            break;
        }
        sum += number;
    }
}
