#![allow(dead_code)]


fn sum_and_product(a:i32,b:i32) -> (i32,i32)
{
    /* not idiomatic
    let sum = a+b;
    let prod = a*b;
    return(sum,prod);
     */
    (a+b,a*b)// should not put a ";" at the end!?
}

pub fn tuples(){
    let x =3;
    let y=4;
    let sp = sum_and_product(x,y);
    println!("sum and product of {} and {} is {:?}",x,y, sp);
    println!("{0}+{1}={2}, {0}*{1}={3}",x,y,sp.0,sp.1);

    // destructuring
    let (s,p) = sp;
    println!("destructured {0}+{1}={2}, {0}*{1}={3}",x,y,s,p);

    //tuples of tuples
    let sp2 = sum_and_product(4,7);
    let combined = (sp,sp2);
    println!("{:?}", combined);
    println!("last element of the combined tuple is {}", (combined.1).1);

    let((c,d),(e,f)) = combined; // destructuring a tuple of tuples
    //structure is checked at compile time
    println!("c={} d={} e={} f={}",c,d,e,f);

    let foo = (true, 42, -1i8);
    println!("foo= {:?}",foo);

    let single_elmt_tuple = (42,);//needs to have "," to show it is a single element tuple
    println!("single element tuple is {:?}",single_elmt_tuple);
}