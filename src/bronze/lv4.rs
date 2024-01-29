
pub fn problem2440(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number : i32 = input.trim().parse().unwrap();
    for i in (0..number).rev(){
        for _ in 0..i+1{
            print!("*");
        }
        println!();
    }
}