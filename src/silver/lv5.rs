pub fn problem2018() {
    //수들의 합
    //연속된 자연수의 합으로 나타낼 수 있는 경우의 수
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    let mut sum = 0;
    let mut count = 0;

    for i in 1..number + 1 {
        sum = 0;
        for j in i..number + 1 {
            sum += j;
            if sum == number {
                count += 1;
                break;
            } else if sum > number {
                break;
            }
        }
    }
    println!("{}", count);
}
