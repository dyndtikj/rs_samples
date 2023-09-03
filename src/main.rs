fn main() {
    println!("example debug");
    let str = String::from("123e2.5");
    let s = str.chars().collect::<Vec<char>>();
    println!("{:?}", s);
    let is_number = solutions::s_0065_valid_number::Solution::is_number(str.clone());
    println!("testing is str {} a number ?... {}", str, is_number);
    println!("{}", solutions::s_0115_distinct_subseq::Solution::num_distinct(String::from("babababababababa"), String::from("aa")));
}
