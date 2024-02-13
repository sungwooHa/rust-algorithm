pub fn problem23739() {
    //n개의 챕터
    //i번째 챕터를 공부하는데 Ti 시간이 걸림
    //30분 쉬고 30분 공부하고
    //30분 하면 그 챕터는 끝
    //절반이상 공부한 챕터 구하기

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number_chapter: i32 = input.trim().parse().unwrap();

    let mut half_over: Vec<bool> = Vec::new();
    let mut remaining_time = 30;
    for _ in 0..number_chapter {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let time: i32 = input.trim().parse().unwrap();

        remaining_time -= time;
        if remaining_time > 0 {
            half_over.push(true);
        } else {
            if time / 2 < remaining_time.abs() {
                half_over.push(false);
            } else {
                half_over.push(true);
            }

            remaining_time = 30;
        }
    }
    println!("{}", half_over.iter().filter(|x| **x).count());
}
