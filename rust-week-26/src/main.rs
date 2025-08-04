fn main() {
    let mut str = String::from("Siddharth");
    let ref1 = &mut str;
    ref1.push_str("gope");
    let ref2 = &str;
    print!("{}", str);
    print!("{}", ref2)
}
