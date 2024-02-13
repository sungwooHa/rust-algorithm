pub fn problem11502() {
    //3 - primes problem
    //5보다 큰 임의의 홀수는 3개의 소수들의 합으로 나타낼 수 있다.
    //이때, 3개의 소수들을 구하시오.

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number_test: i32 = input.trim().parse().unwrap();
    let numbers: Vec<i32> = (0..number_test)
        .map(|_| {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().parse().unwrap()
        })
        .collect();

    for idx in 0..number_test {
        let number = numbers[idx as usize];
        //소수 군집 만들기
        //에라토스테네스의 체
        //지우기로 날림!
        let mut prime = vec![true; number as usize + 1];
        prime[0] = false; //안씀
        prime[1] = false; //안씀
        for i in 2..(number as f64).sqrt() as i32 + 1 {
            if prime[i as usize] {
                //소수라면, 뒤에 배수는 다 짜름.
                for j in (i * i..number + 1).step_by(i as usize) {
                    prime[j as usize] = false;
                }
            }
        }

        println!("{:?}", prime);
        let mut result = Vec::new();
        for i in 2..number + 1 {
            if prime[i as usize] {
                result.push(i);
            }
        }

        let mut is_possible = false;
        for i in 0..result.len() {
            for j in i..result.len() {
                for k in j..result.len() {
                    if result[i] + result[j] + result[k] == number {
                        println!("{} {} {}", result[i], result[j], result[k]);
                        is_possible = true;
                        break;
                    }
                }
                if is_possible {
                    break;
                }
            }
            if is_possible {
                break;
            }
        }
    }
}
