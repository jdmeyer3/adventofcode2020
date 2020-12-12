use std::collections::HashMap;

struct OutputJoltage(Vec<usize>);

impl OutputJoltage {
    // returns # of 1 jolt differences and the # of 3 jolt differences
    // tries to achieve O(n) on checking voltage

    fn get_difference(&mut self) -> (i32, i32) {
        self.0.sort_unstable();
        let mut one_j = 0;
        let mut three_j = 0;

        let mut last_num: i32 = 0;
        for (i, j) in self.0.iter().enumerate() {
            let diff = *j as i32 - last_num;
            println!("num: {}   last num: {}  diff: {}", j, last_num, diff);
            if diff == 1 {
                one_j += 1;
            } else if diff == 3 {
                three_j += 1;
            }
            last_num = *j as i32;
        }
        // add one more to three because the device has a difference of 3
        three_j += 1;

        (one_j, three_j)
    }

    fn get_permutations(&mut self) -> isize {
        let perms = 0;
        self.0.push(self.0.last().unwrap() + 3);
        self.0.insert(0, 0);
        let mut dp: HashMap<isize, isize> = HashMap::new();

        fn dynamic_programming(data: &Vec<usize>, index: usize, dp: &mut HashMap<isize, isize>) -> isize {
            if index == data.len() - 1 {
                return 1;
            }
            match dp.get(&(index as isize)) {
                None => {}
                Some(i) => return *i
            }
            let mut ans: isize = 0;
            for j in 1..=3 {
                if j + index == data.len() {
                    break;
                }
                if data[j + index] - data[index as usize] <= 3 {
                    ans += dynamic_programming(data, j + index, dp);
                }
            }
            dp.insert(index as isize, ans);
            return ans;
        }

        return dynamic_programming(&self.0, 0, &mut dp);
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day10").unwrap();
    let mut output_j = OutputJoltage(input.lines().map(|j| j.parse().unwrap()).collect::<Vec<usize>>());
    let (one_j, three_j) = output_j.get_difference();
    let perm = output_j.get_permutations();
    println!("pt1: diff 1: {} diff 2: {} result: {}", one_j, three_j, one_j * three_j);
    println!("pt2: perm: {}", perm);
}