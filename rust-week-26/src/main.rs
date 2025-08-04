use rect::Rect;
pub mod rect;


fn main() {
   let r = Rect {
    width:10.0,
    height:2.0
   };

//    print!("{} , {}" , r.width , r.width);
   print!("{}" , r.area())
}
