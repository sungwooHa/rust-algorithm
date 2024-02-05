use std::result;

pub fn problem5364() {
    //평면상의 거리 찾기.
    //기준 (X,Y)로 부터 가장 가까운 거리를 찾는다.
    //거리 = ((x1-x)^2 + (y1-y)^2).sqrt()
    //소수점 2자리까지 출력
    fn distance(cp: (i32, i32), dest: (i32, i32)) -> f32 {
        let (x, y) = cp;
        let (x1, y1) = dest;
        (((((x1 - x).pow(2) + (y1 - y).pow(2)) as f32).sqrt() * 100.0) as f32).floor() / 100.0
    }

    fn get_pt_by_input() -> (i32, i32) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let pt: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        (pt[0], pt[1])
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number_planet: i32 = input.trim().parse().unwrap();

    let cp = get_pt_by_input();

    let mut plants_with_dist: Vec<((i32, i32), f32)> = Vec::new();
    for _ in 0..number_planet - 1 {
        let plant = get_pt_by_input();
        let dist = distance(cp, plant);
        plants_with_dist.push((plant, dist));
    }
    
    let res = plants_with_dist.iter().min_by(|a, b| a.1.total_cmp(&b.1));
    //plants_with_dist.sort_by(|a, b| a.1.total_cmp(&b.1).unwrap());

    println!("{} {}", cp.0, cp.1);
    println!("{} {}", res.unwrap().0.0, res.unwrap().0.1);
    println!("{}", res.unwrap().1);
    //println!("{} {}", plants_with_dist[0].0 .0, plants_with_dist[0].0 .1);
    //println!("{}", plants_with_dist[0].1);
}
