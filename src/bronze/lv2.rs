pub fn problem5346() {
    //평면상의 거리 찾기.
    //기준 (X,Y)로 부터 가장 가까운 거리를 찾는다.
    //거리 = (x1-x)^2 + (y1-y)^2
    //소수점 2자리까지 출력

    fn distance(cp : (i32, i32), dest : (i32, i32)) -> f32 {
        let (x, y) = cp;
        let (x1, y1) = dest;
        return ((x1 - x).pow(2) + (y1 - y).pow(2) * 100).floor() / 100;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number_planet: i32 = input.trim().parse().unwrap();

    let mut cp = String::new();
    std::io::stdin().read_line(&mut cp).unwrap();
    let cp: (i32, i32) = cp
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", cp);

    // let mut plants_with_dist: Vec<((i32,i32), f32)> = Vec::new();
    // for _ in 0..number_planet {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     let planet: Vec<i32> = input
    //         .split_whitespace()
    //         .map(|x| x.parse().unwrap())
    //         .collect();

    //     distance(x, y, x1, y1)

    //     planet.push(plant);
    // }
    
}