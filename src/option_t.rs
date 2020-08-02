#![allow(dead_code)]

pub fn option_t()
{
    let x= 3.0;
    let y= 2.0;

    // Option -> Some(v) | None
    let result =
        if y != 0.0 {Some(x/y)}
        else {None}
        ;

    match result
    {
        Some(z) => println!("{}/{}={}",x,y,z),
        None => println!("cannot divide by zero")
    }

    if let Some(z) = result {
        println!("result ={}",z)
    }
    /* //this is a while(true)
    while let  Some(z) = result{
        println!("(while) result ={}",z);
    }
     */
}