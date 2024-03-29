pub mod problem10871 {
    //수열 A와 정수 X

    pub fn problem() {
        let mut input = String::new();

        //N과 X를 입력받음
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let size_progression: i32 = iter.next().unwrap().parse().unwrap();
        let comp_number: i32 = iter.next().unwrap().parse().unwrap();

        //수열 A를 이루는 정소 N개
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for _ in 0..size_progression {
            let a: i32 = iter.next().unwrap().parse().unwrap();
            if a < comp_number {
                print!("{} ", a);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_problem() {
            problem();
        }
    }
}

pub mod problem10807 {
    use std::collections::HashMap;

    pub fn problem() {
        //첫번째 줄, 정수의 개수 N

        let numb_integer = {
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            iter.next().unwrap().parse().unwrap()
        };

        //두번째 줄, N개의 정수
        let numb_bag = {
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let mut numb_bag: HashMap<i32, u32> = HashMap::new();
            for _ in 0..numb_integer {
                let number = iter.next().unwrap().parse().unwrap();
                match numb_bag.get_mut(&number) {
                    Some(size) => *size += 1,
                    None => {
                        numb_bag.insert(number, 1);
                        ()
                    }
                }
            }
            numb_bag
        };

        let numb_target = {
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let target_number = iter.next().unwrap().parse().unwrap();
            match numb_bag.get_key_value(&target_number) {
                Some((_, numb)) => *numb as i32,
                None => 0,
            }
        };

        print!("{}", numb_target);
    }
}

pub mod problem5597 {
    //제줄 안한 2명 찾기.
    pub fn problem() {
        const NUMB_STUDENT: i32 = 30;
        let mut student_list: [i32; NUMB_STUDENT as usize] = [-1; NUMB_STUDENT as usize];

        for _ in 0..NUMB_STUDENT - 2 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let student_number: i32 = iter.next().unwrap().parse().unwrap();
            student_list[(student_number - 1) as usize] = student_number;
        }

        for index in 0..NUMB_STUDENT {
            if student_list[index as usize] == -1 {
                println!("{}", index + 1);
            }
        }
    }
}

pub mod problem2738 {
    //두 행렬의 덧셈
    pub fn problem() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let row: usize = iter.next().unwrap().parse().unwrap();
        let col: usize = iter.next().unwrap().parse().unwrap();

        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for _ in 0..row {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let mut row: Vec<i32> = Vec::new();
            for _ in 0..col {
                let number: i32 = iter.next().unwrap().parse().unwrap();
                row.push(number);
            }
            matrix.push(row);
        }

        for idx_row in 0..row {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            for idx_col in 0..col {
                let number: i32 = iter.next().unwrap().parse().unwrap();
                matrix[idx_row][idx_col] += number;
            }
        }

        for idx_row in 0..row {
            for idx_col in 0..col {
                print!("{} ", matrix[idx_row][idx_col]);
            }
            println!();
        }
    }
}
