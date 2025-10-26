fn main() {
    println!("{}" , sum(1,2))
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
