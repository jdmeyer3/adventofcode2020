
#[derive(Debug)]
struct TobogganPwd<'a> {
    pos1: i32,
    pos2: i32,
    char: char,
    pwd: &'a str,
}

#[derive(Debug)]
struct UsrPwd<'a> {
    min: i32,
    max: i32,
    char: char,
    pwd: &'a str
}

trait Password {
    fn valid(&self) -> bool;
}

impl UsrPwd<'_> {
    fn from_file_str(file: Vec<&str>) -> Vec<UsrPwd> {
        let f = file.iter().map(|f|read_pwd_str(*f)).collect::<Vec<_>>();
        f.iter().map(|f| UsrPwd{min: f.0, max: f.1, char: f.2, pwd: f.3}).collect::<Vec<UsrPwd>>()
    }
}

impl Password for UsrPwd<'_> {
    fn valid(&self) -> bool {
        let char_count = self.pwd.chars().filter(|c| *c==self.char).count();
        if char_count >= self.min as usize && char_count <= self.max as usize {
            return true;
        }
        false
    }

}

impl TobogganPwd<'_> {
    fn from_file_str(file: Vec<&str>) -> Vec<TobogganPwd> {
        let f = file.iter().map(|f|read_pwd_str(*f)).collect::<Vec<_>>();
        f.iter().map(|f| TobogganPwd{pos1: f.0, pos2: f.1, char: f.2, pwd: f.3}).collect::<Vec<TobogganPwd>>()
    }
}

impl Password for TobogganPwd<'_> {
    fn valid(&self) -> bool {
        let foo = self.pwd.chars().into_iter().skip(self.pos1 as usize -1).next();
        let bar = self.pwd.chars().into_iter().skip(self.pos2 as usize -1).next();
        println!("{:?}", foo);
        println!("{:?}", bar);
        println!("{:?}", self.char);
        if (foo.is_some() && foo.unwrap() == self.char) ^ (bar.is_some() && bar.unwrap() == self.char) {
            println!("true");
            return true;
        }
        println!("false");
        false
    }
}

fn read_pwd_str(pwd_list: &str) -> (i32, i32, char, &str) {

    let p_split = pwd_list.split(" ").collect::<Vec<_>>();
    let min_max = p_split[0].split("-").collect::<Vec<_>>();
    let char = p_split[1].replace(":", "").parse::<char>().unwrap();
    let pwd = p_split[2];
    let min = min_max[0].parse::<i32>().unwrap();
    let max = (min_max[1]).parse::<i32>().unwrap();
    return (min, max, char, pwd)

}

fn main() {
    let input = std::fs::read_to_string("input/day2").unwrap();
    let passwd = input.lines().collect::<Vec<_>>();

    // pt 1

    let old_pwd_list = UsrPwd::from_file_str(passwd.clone());
    let p = old_pwd_list.iter().filter(|p| p.valid()).collect::<Vec<_>>().len();
    println!("{:?}", p);

    // pt2
    let old_pwd_list = TobogganPwd::from_file_str(passwd);
    let p = old_pwd_list.iter().filter(|p| p.valid()).collect::<Vec<_>>().len();
    println!("{:?}", p)

}