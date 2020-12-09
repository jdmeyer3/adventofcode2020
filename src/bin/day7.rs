extern crate regex;

use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

use regex::Regex;

#[derive(Default, Debug)]
struct RawRule(String, Vec<(usize, String)>);

#[derive(Default, Debug)]
struct Rule {
    bag: usize,
    count: usize,
    contains: usize,
}

#[derive(Debug, Default)]
struct BagGraph {
    bag_to_id: HashMap<String, usize>,
    id_to_bag: Vec<String>,
    // an array that contains the index of the rule that connects the bag to its parent bag. If
    // rule (rule index is 3) stated bag (of index 1) contained bag (of index 2), then the result will look like
    // [[],[],[3]]. The third array index (index 2) contains the index of the rule that connects it together
    connection_rule_id: Vec<Vec<usize>>,
    // bag_contains is an array of bag_index arrays, the first index corresponds to its
    // bag index. Within that index is an array of bag indexes that correspond to the bags that are
    // contained within
    bag_contains: Vec<Vec<usize>>,
    rules: Vec<Rule>,
}

impl BagGraph {
    fn add_bag(&mut self, bag_color: &str) -> usize {
        let bag_id = self.bag_to_id.len();
        self.bag_to_id.insert(bag_color.to_owned(), bag_id);
        self.id_to_bag.push(bag_color.to_owned());
        self.connection_rule_id.push(Default::default());
        self.bag_contains.push(Default::default());
        bag_id
    }

    fn calculate_sub_bag_total(&self, bag_color: &str) -> usize {
        // println!("bag: {:?}", self);
        let bag_id = *self.bag_to_id.get(bag_color).unwrap();
        let total = calculate(self, bag_id);
        return total;
        // println!("total: {}", total);

        // TODO: speed this up
        fn calculate(bag_graph: &BagGraph, bag_id: usize) -> usize {

            // println!("\n\nstarting calculation");
            // get all the rules that start with the bag id
            let next = bag_graph.rules.iter().filter(|f| f.bag == bag_id).collect::<Vec<&Rule>>();

            // println!("next rules are {:?}", next);
            if next.len() == 0 {
                return 1;
            }
            let mut count = 0;
            for n in next {
                // println!("evaluating rule: {:?}", n);

                // recursively go through each contains rule, multiplying by the parent
                // if the recursive count is 1, then that means we don't count the bag and whats in the bag
                // we just count the number of sub bags
                let sub_count = calculate(bag_graph, n.contains);
                if sub_count == 1 {
                    count += n.count * sub_count;
                    continue
                }

                count += n.count + (n.count * calculate(bag_graph, n.contains));

                // println!("new count is {}", count);
            }
            count
        }
    }
}

fn edge_rules(input: String) -> Vec<RawRule> {
    let mut rules = Vec::new();
    for i in input.lines() {
        let mut rule = RawRule::default();

        let rule_re = Regex::new(r"(.*)\sbags\scontain\s(.*).$").unwrap();
        let cap = rule_re.captures(i).unwrap();
        rule.0 = cap.get(1).unwrap().as_str().parse().unwrap();
        let bag_rules = cap.get(2).unwrap().as_str();

        let contains_re = Regex::new(r"(\d) (.*?) bag[s]?").unwrap();
        for cap in contains_re.captures_iter(bag_rules) {
            let qty = cap.get(1).unwrap().as_str();
            let color = cap.get(2).unwrap().as_str();
            rule.1.push((qty.parse().unwrap(), color.parse().unwrap()));
        }

        rules.push(rule);
    }
    return rules;
}

fn main() {
    let input = std::fs::read_to_string("input/day7").unwrap();
    let rules = edge_rules(input);

    let mut bag_graph = BagGraph::default();
    let my_bag = bag_graph.add_bag("shiny gold");

    // add nodes
    // every bag in the input has a bag and what it contains except for the target bag, which was
    // specified above. So that means we only have to go through each bag rather than its corresponding nodes
    for rule in rules.iter() {
        let bag = rule.0.clone();
        if !bag_graph.bag_to_id.contains_key(&bag) {
            bag_graph.add_bag(&bag);
        }
    }

    // add rules
    // What adding rules will do will is go through each bag. It will retrieve that bag id. Then for
    // each bag, it will look what that bag contains. For each contains, that sub bag's id is pulled. The
    // graph is then updated with the bag id's "bag_contains". Meaning parent bag will show that it contains
    // the index of sub bags in the bag contain. It will then push the rule that connected the two bags together
    // and set the sub bag's "bag within rule" with the id of the rule that connected it to it.
    for rule in rules {
        let bag_id = *bag_graph.bag_to_id.get(&rule.0).unwrap();
        for (qty, sub_bags) in rule.1 {
            let sub_bag_id = *bag_graph.bag_to_id.get(&sub_bags).unwrap();
            bag_graph.bag_contains[bag_id].push(sub_bag_id);
            let rule_index = bag_graph.rules.len();
            bag_graph.connection_rule_id[sub_bag_id].push(rule_index);
            bag_graph.rules.push(
                Rule {
                    bag: bag_id,
                    count: qty,
                    contains: sub_bag_id,
                }
            )
        }
    }

    // creates an array of bags representing whether that bag could contain "my bag". This representation
    // is done by having all positions in the index set to false
    let mut contains_my_bag = vec![false; bag_graph.id_to_bag.len()];

    // This stack pulls the index of the rules that directly connect a bag.
    let mut stack = VecDeque::from_iter(bag_graph.connection_rule_id[my_bag].iter().copied());

    // pulls a rule index off of the stack
    while let Some(elem) = stack.pop_front() {
        // pulls the rule using the rule index
        let rule = &bag_graph.rules[elem];

        // looks at the rule to see which bag is pointing to the target bag. Using that bag index
        // we set the corresponding "contains_my_bag" index to true to indicate the bag that connects
        // to the target bag
        contains_my_bag[rule.bag] = true;

        // pulls the rule of any bag that connects to the bag that connects to the target bag. Those
        // rules are then added to the stack as it becomes the 2nd level connection, then maybe a third
        // then a forth
        for rule_id in bag_graph.connection_rule_id[rule.bag].iter() {
            stack.push_back(*rule_id)
        }
    }

    let cnt = contains_my_bag.iter().filter(|f| **f).count();
    println!("pt1 count: {:?}", cnt);


    let cnt = bag_graph.calculate_sub_bag_total("shiny gold");
    println!("pt2 count: {:?}", cnt)
}