pub fn problem25270() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let price: i32 = input.trim().parse().unwrap();

    let up = (price / 100 + 1) * 100 - 1;
    let down: i32 = {
        if price < 100 {
            99
        } else {
            price / 100 * 100 - 1
        }
    };

    if up - price < price - down || up - price == price - down {
        println!("{}", up);
    } else {
        println!("{}", down);
    }
}
