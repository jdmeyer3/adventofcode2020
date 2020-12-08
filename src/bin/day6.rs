use itertools::Itertools;


#[derive(Default, Debug)]
struct Questionnaire(Vec<Vec<char>>);


impl Questionnaire {
    fn answer_everyone(&self) -> usize {
        if self.0.len() == 1 {
            return self.0[0].len();
        }
        self.0[0].iter().map(|f| self.check_everyone_answer(1, f)).filter(|f| *f).count()
    }

    fn check_everyone_answer(&self, mut person: usize, answer: &char) -> bool {
        if person + 1 == self.0.len() {
            return self.0[person].contains(answer)
        }
        if self.0[person].contains(answer) {
            person += 1;
            return self.check_everyone_answer(person, answer)
        }
        false
    }

    fn answer_flat(&self) -> usize {
        let answer: Vec<char> = Vec::new();
        let foo = self.0.clone().into_iter().flatten().unique().collect::<Vec<char>>();
        println!("{}", foo.len());
        foo.len()
    }
}

fn read_results(input: String) -> Vec<Questionnaire> {
    let mut q: Vec<Questionnaire> = Vec::new();
    q.push(Questionnaire::default());
    for i in input.lines() {
        if i.is_empty() {
            q.push(Questionnaire::default());
            continue;
        }
        let foo = i.chars().unique().collect::<Vec<char>>();
        q.last_mut().unwrap().0.push(foo);
    }
    q
}


fn main() {
    let input = std::fs::read_to_string("input/day6").unwrap();
    let q_results = read_results(input);
    println!("total pt1: {:?}", q_results.iter().map(|f| f.answer_flat()).sum::<usize>());
    println!("total pt2: {:?}", q_results.iter().map(|f| f.answer_everyone()).sum::<usize>());
}