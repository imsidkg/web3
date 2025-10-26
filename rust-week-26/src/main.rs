struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        self.width * self.height
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 10,
    };

    let r1 = Rect {
        width: 0.2,
        height: 10.0,
    };
    print!("{}", r.area());
    println!("{}", r1.area())
}
