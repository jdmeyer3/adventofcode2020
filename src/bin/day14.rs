use regex::Regex;

#[derive(Default, Debug)]
struct DocProgram {
    // mask is a vector of tuples. The tuple is the
    // bit value and which bit position to AND it.
    mask: Vec<(usize, usize)>,
    mem: Vec<usize>,
    // inst is the instructions for the memory. It is a tuple of
    // the memory location as well as the value it is applying
    inst: Vec<(usize, usize)>,
    /// the largest memory position, this will be used to reinitialize the memory
    max_mem: usize
}

impl DocProgram {
    fn from_file(input: &str) -> Self {
        let mut mem = DocProgram::default();
        let re = Regex::new(r"\[(\d+)\]\s=\s(\d+)").unwrap();
        for (i, l) in input.lines().enumerate() {
            if i == 0 {
                let mask = l.split(" = ").collect::<Vec<_>>()[1].chars();
                for (mask_i, m ) in mask.into_iter().rev().enumerate() {
                    if m == '0' {
                        mem.mask.push((0, mask_i));
                        continue;
                    }
                    if m == '1' {
                        mem.mask.push((1, mask_i));
                        continue;
                    }

                }
                continue;
            }
            let cap = re.captures(l).unwrap();
            let mem_loc = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let val = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            if val > mem.max_mem {
                mem.max_mem = val;
            }
            mem.inst.push((mem_loc, val));
        }
        mem.mem = vec![0; mem.max_mem];
        mem
    }
    fn apply_mask(&mut self) {
        for (i, val) in self.inst.iter_mut() {
            for (bit_val, bit_pos) in self.mask.iter() {
                // *val &= (*bit_val << *bit_pos);
                println!("{:b}", *val & (*bit_val << *bit_pos));
            }
            self.mem[*i] = *val;

        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day14").unwrap();
    let mut mem = DocProgram::from_file(&input);
    println!("{:?}", mem);
    mem.apply_mask();
    println!("{:?}", mem);
    // let mut byte: usize = 0b00_0000_0000_0000_0000_0000_0000_0000_0010;
    // println!("{:b}", byte & (0 << 1))
}