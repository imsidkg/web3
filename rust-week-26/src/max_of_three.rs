pub fn max_of_three(a:u32 , b:u32 , c:u32) -> u32{
    if a>b && a> c  {
        return a;
    }
    if b>a && b> c  {
        return b;
    }
    else {
        return c;
    }

}