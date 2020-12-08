use itertools::Itertools;

// It is # of people, and # of questions
#[derive(Default, Debug)]
struct Questionnaire(usize, usize, Vec<char>);

fn read_results(input: String) -> Vec<Questionnaire> {
    let mut q: Vec<Questionnaire> = Vec::new();
    q.push(Questionnaire::default());
    for l in input.lines() {
        let q_ptr = q.last_mut().unwrap();
        if l.is_empty()  {
            q_ptr.2.iter().unique().collect::<Vec<_>>();
            q_ptr.1 = q_ptr.2.iter().unique().collect::<Vec<_>>().len();
            println!("{:?}", q_ptr);
            q.push(Questionnaire::default());
            continue;
        }
        q_ptr.0 += 1;
        q_ptr.2.extend_from_slice(&l.chars().collect::<Vec<char>>());
        println!("{:?}", q_ptr);

    }
    // done once more since last line doesn't get evaluated
    let mut tmp_ptr = q.last_mut().unwrap();
    tmp_ptr.2.iter().unique().collect::<Vec<_>>();
    tmp_ptr.1 = tmp_ptr.2.iter().unique().collect::<Vec<_>>().len();
    println!("{:?}", tmp_ptr);
    q
}

fn main() {
    let input = std::fs::read_to_string("input/day6").unwrap();
    let q_results = read_results(input);
    println!("{:?}", q_results);
    let sum: usize = q_results.iter().map(|q| q.1).sum();
    println!("{:?}", sum);
}