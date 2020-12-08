// . = no tree  ;  # = tree



struct Treeline(Vec<bool>);


impl Treeline {
    fn navigate_treeline(&self, slope_size: usize, x_axis: usize, y_axis: usize) -> usize {
        let mut current_pos = 0;
        let mut trees_hit = 0;

        while current_pos < self.0.len() {
            let slope_row = current_pos / slope_size;
            // let slope_start_idx = slope_row * slope_size;
            let slope_end_idx = (slope_row + 1) * slope_size - 1;

            // x axis
            if (current_pos + x_axis) > slope_end_idx {
                current_pos = current_pos - slope_size + x_axis;
            } else {
                current_pos = current_pos + x_axis;
            }

            // y axis
            current_pos = current_pos + (slope_size * y_axis);

            if self.0.get(current_pos).is_some() && *self.0.get(current_pos).unwrap() {
                trees_hit += 1;
            }
        }
        trees_hit
    }
}

fn generate_treemap(trees: String) -> Treeline {
    let mut treeline: Vec<bool> = Vec::new();
    for t in trees.lines() {
        treeline.extend_from_slice(&t.chars().map(|t| match t {
            '.' => false,
            '#' => true,
            _ => panic!("wtf is this")
        }).collect::<Vec<bool>>());
    }
    Treeline(treeline)
}

fn main() {
    let input = std::fs::read_to_string("input/day3").unwrap();
    let g = generate_treemap(input);
    //pt1
    println!("# of trees hit: {}", g.navigate_treeline(31, 3, 1));

    //pt2
    let first = g.navigate_treeline(31, 1, 1);
    let second = g.navigate_treeline(31, 3, 1);
    let third = g.navigate_treeline(31, 5, 1);
    let fourth = g.navigate_treeline(31, 7, 1);
    let fifth = g.navigate_treeline(31, 1, 2);

    println!("# of trees hit: {}", first * second * third * fourth * fifth);
}