//떡국 하루 한개 이상
//나이 N
// N이 만들어지는 경우의 수를 구하라.
// 매일 먹어서 경우의 수를 만들 수 있음.
// 떡국을 먹지 않으면 죽는다.

//0 : 0
//1 : 1
//2 : 2번   2,0  1,1,0
//3 : 4번   3,0  2,1,0  1,1,1,0  1,2,0
//4 : 8번   4,0  3,1,0  2,2,0  2,1,1,0  1,1,1,1,0  1,3,0  1,2,1,0  1,1,2,0

// N살, M일살다 (M-1)까지 살았다.
// 떡꾹을, 일자에 맞춰 골고루 나눠준다. 단 0은 빼야함.
// (떡국-1)C(일자-2)
// (N-1)C(M-2);
// 2일 부터, N일까지
// 산날(m) <= 떡국섭취(n) + 1
// (n-1)C(0) + (n-1)C(1) + (n-1)C(2) + .... + (n-1)C(n-1) 다 더해야한다.
// 1
// 1, 1, 0, 0, ...

// 파스칼 삼각형

use std::collections::hash_map;

pub fn problem15717() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let age: i64 = input.trim().parse().unwrap();
}

//돌을 가져가는 방법은 1,3,4개씩 가져갈 수 있다.
//마지막 돌을 가져가는 사람이 승리한다.
//게임은 상근이가 먼저 시작한다
//상근이가 이기면 SK, 창영이가 이기면 CY를 출력한다.

//돌이 1~4개 있을 경우 이긴다.
//돌이 5개 있을 경우, 3개를 가져가면 이긴다.
//돌이 6개 있을 경우, 2,3,5개를 만들면 이긴다. (1,3,4개를 가져간다)
//돌이 7개 있을 경우, 3,4,6개를 만든다. 그럼 무조건 진다.  --> 7개면 진다.
//

// 1 상근(1)
// 2 창영(1 1)
// 3 상근
// 4 상근
// 5 상근 3 (1, 1)
// 6 상근 4 (1, 1, 1)
// 7

pub fn problem9660() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numb_stone: i64 = input.trim().parse().unwrap();
}

//N개의 커피가 한개씩 있다 ( 종류가 N개 인듯)
//각 커피의 카페인 Ci
//K만큼 섭취하고 싶다.
//K를 만드는 커피의 최소 개수.
pub fn problem22115() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    let mut coffees: Vec<i64> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let coffee: i64 = input.trim().parse().unwrap();
        coffees.push(coffee);
    }
}

pub fn problem20207() {
    //제일큰 직사각형 구하기.
    //직사각형으로 다 할거야.
    //1일부터 365일 표시 달력
    //코팅지 높이 최대 : 하루 최대 수행
    //코팅지 가로 최대 : 하루라도 겹치는 일정 최대
    //시작일, 종료일, 겹치는 일 중에 제일 큰 수
    //겹치는 것

    #[derive(Debug)]
    struct Rectangle {
        start_day: i64,
        end_day: i64,
        max_height: i64,
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let task_numb: i64 = input.trim().parse().unwrap();

    let mut rectangles: Vec<Rectangle> = Vec::new();

    let mut task_max: hash_map::HashMap<i64, i64> = hash_map::HashMap::new();
    for _ in 0..task_numb {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let start_day: i64 = iter.next().unwrap().parse().unwrap();
        let end_day: i64 = iter.next().unwrap().parse().unwrap();
        rectangles.push(Rectangle {
            start_day,
            end_day,
            max_height: 1,
        });

        for day in start_day..end_day + 1 {
            if task_max.contains_key(&day) {
                let count = task_max.get_mut(&day).unwrap();
                *count += 1;
            } else {
                task_max.insert(day, 1);
            }
        }
    }

    //rectangles를 정렬하자 (시작일 기준)
    rectangles.sort_by(|a, b| a.start_day.cmp(&b.start_day));
    
    //자 이제 합치자
    for i in 0..rectangles.len() {
        let mut j = i + 1;
        loop {
            if j >= rectangles.len() {
                break;
            }
            if rectangles[i].start_day <= rectangles[j].start_day
                && rectangles[i].end_day >= rectangles[j].end_day
            {
                //할 것 없음
                rectangles.remove(j);
            } else if rectangles[i].start_day >= rectangles[j].start_day
                && rectangles[i].start_day <= rectangles[j].end_day
            {
                //start day가 rectangle[j]의 start_day와 end_day 사이에 있다.
                rectangles[i].start_day = rectangles[j].start_day;
                if rectangles[i].end_day < rectangles[j].end_day {
                    rectangles[i].end_day = rectangles[j].end_day;
                }
                rectangles.remove(j);
            } else if rectangles[i].end_day >= rectangles[j].start_day
                && rectangles[i].end_day <= rectangles[j].end_day
            {
                //end day가 rectangle[j]의 start_day와 end_day 사이에 있다.
                rectangles[i].end_day = rectangles[j].end_day;
                if rectangles[i].start_day > rectangles[j].start_day {
                    rectangles[i].start_day = rectangles[j].start_day;
                }
                rectangles.remove(j);
            } else {
                j += 1;
            }
        }
    }

    //각 rectangle의 max_height를 구하자.
    for i in 0..rectangles.len() {
        for day in rectangles[i].start_day..rectangles[i].end_day+1 {
            if task_max.contains_key(&day) {
                let count = task_max.get(&day).unwrap();
                if *count > rectangles[i].max_height {
                    rectangles[i].max_height = *count;
                }
            }
        }
    }

    // 이제 면적을 구하자
    let mut max_area = 0;
    for i in 0..rectangles.len() {
        let area = (rectangles[i].end_day - rectangles[i].start_day + 1) * rectangles[i].max_height;
        max_area += area;
    }

    println!("{}", max_area);
}
