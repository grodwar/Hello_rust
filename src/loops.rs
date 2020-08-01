#![allow(dead_code)]

pub fn while_loop()
{
    let mut x = 1;
    while x < 1000
    {
        x*=2;
        if x == 64 {continue;} // back to top of the loop without executing the rest of that iteration
        println!("x ={}",x);
    }
    let mut y = 1;
    loop  //  while true
    {
        y *=2;
        println!("y = {}", y);
        if y == 1<<10 // equqls to 2^10
            {break; }// jump out of the loop
    }
}
pub fn for_loop()
{
    for x in 1..11 //11 is excluded... similar to for i in range(11) in Python
    {
        if x == 3 {continue;} // same same as while loop
        if x == 8 {break;} // same same as while loop
        println!("x = {}", x);

    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("pos={}: y={}", pos, y);
    }
}