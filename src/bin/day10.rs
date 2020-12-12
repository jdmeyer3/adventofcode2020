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

    fn get_permutations(&mut self) -> usize {
        let perms = 0;
        self.0.push(self.0.last().unwrap() + 3);
        self.0.insert(0, 0);
        let mut ways_to_get_there = vec![0usize; self.0.len()];
        ways_to_get_there[0] = 1;
        for i in 0..self.0.len() {
            let ways_to_get_here = ways_to_get_there[i];
            for j in 1..=3 {
                if i + j >= self.0.len() || self.0[i + j] - self.0[i] > 3 {
                    break;
                }
                ways_to_get_there[i + j] += ways_to_get_here;
            }
        }
        return *ways_to_get_there.last().unwrap()
    }
    // // assumes already sorted
    // // calculations decimated my cpu
    // fn get_permutations(&mut self) -> i32 {
    //     //add the device
    //     self.0.push(self.0.last().unwrap() + 3);
    //     println!("evaluating array {:?}", self.0);
    //     return calculate(&self.0, 0, 0, 0, self.0[self.0.len() - 1] as i32);
    //
    //     fn calculate(data: &Vec<usize>, mut index: usize, mut last_num: i32, mut perm: i32, last_eval: i32) -> i32 {
    //         for j in data[index..].iter() {
    //             // println!("evaluating num: {} - last_num: {}", j, last_num);
    //             index += 1;
    //             if *j as i32 - last_num <= 3 {
    //                 if index == data.len() - 1 {
    //                     // println!("big money");
    //                     perm += 1;
    //                     return perm
    //                 }
    //
    //                 perm = calculate(data, index, *j as i32, perm, last_eval);
    //                 continue
    //             }
    //             return perm
    //         }
    //         return perm
    //     }
    // }

}

fn main() {
    let input = std::fs::read_to_string("input/day10").unwrap();
    let mut output_j = OutputJoltage(input.lines().map(|j| j.parse().unwrap()).collect::<Vec<usize>>());
    let (one_j, three_j) = output_j.get_difference();
    let perm = output_j.get_permutations();
    println!("pt1: diff 1: {} diff 2: {} result: {}", one_j, three_j, one_j*three_j);
    println!("pt2: perm: {}", perm);
}