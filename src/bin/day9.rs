#[derive(Debug, Default)]
struct Encoding {
    preamble: usize,
    data: Vec<usize>,
}

impl Encoding {
    fn find_invalid(&self) -> Option<usize> {
        'outer: for (val_idx, val) in self.data[self.preamble..self.data.len()].iter().enumerate() {
            // get the previous preamble to determine validity
            let preamble = self.data[val_idx..self.preamble + val_idx].to_vec();

            for (i, preamble_val) in preamble.iter().enumerate() {
                if val >= preamble_val {
                    let needed_val = val - preamble_val;

                    // check if the needed value is contained in the preamble. If it is, then move on
                    // to the next value to validate
                    if preamble[i..].contains(&(needed_val as usize)) {
                        continue 'outer;
                    }
                }
            }
            // this means that there was no numbers that matched val if the code go here
            return Some(*val);
        }
        None
    }
    fn find_contiguous(&self, target_val: usize) -> Option<Vec<&usize>> {
        let mut c_list = Vec::new();
        'outer: for (val_idx, val) in self.data.iter().enumerate() {
            c_list.push(val);
            if val > &target_val {
                c_list = Vec::new();
                continue
            }
            let mut remaining_val = target_val - val;
            for sub_val in self.data[val_idx+1..].iter() {
                c_list.push(sub_val);
                if sub_val > &remaining_val {
                    c_list = Vec::new();
                    continue 'outer
                }
                remaining_val -= sub_val;
                if remaining_val == 0 {
                    return Some(c_list)
                }
            }
        }
        None
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day9").unwrap();
    let mut enc: Encoding = Encoding::default();
    enc.preamble = 25;
    for l in input.lines() {
        enc.data.push(l.parse().unwrap())
    }
    let invalid = enc.find_invalid().unwrap();
    // let invalid = Some(127);

    println!("pt1: {:?}", invalid);
    let mut contiguous = enc.find_contiguous(invalid).unwrap();
    contiguous.sort();
    println!("pt2: {:?}", contiguous[0] + contiguous[contiguous.len()-1])
}