
fn main() {
    let input = std::fs::read_to_string("input/day2").unwrap();
    let foo = input.lines().map(|i| i.split(":")).collect::<Vec<_>>();
    for mut f in foo {
        println!("{:?}", f.next());
        println!("{:?}", f.next())
    }
}