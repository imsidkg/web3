pub fn sum_of_nums(limit : u32) -> u32 {
    let mut ans:u32 =0;
    for i in 1..=limit {
        ans = ans+i
    }
    return ans;
}