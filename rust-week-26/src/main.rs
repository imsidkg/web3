use crate::{max_of_three::max_of_three, sum_of_nums::sum_of_nums};

mod odd_even;
mod max_of_three;
mod sum_of_nums;
fn main() {
    // let ans = odd_even::odd_even(31);
    // let ans = max_of_three(10, 20, 30);
    let ans = sum_of_nums(10);
    println!("{}", ans)

}
