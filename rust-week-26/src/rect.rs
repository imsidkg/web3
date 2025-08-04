pub struct Rect {
    pub height: f32,
    pub width : f32,
}

impl Rect {
    pub fn area(&self) -> f32{
        return self.height * self.width
    }

    pub fn print_something(_a:u32) {
        println!("Static function")
    }
}
