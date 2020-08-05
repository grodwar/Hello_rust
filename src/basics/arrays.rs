#![allow(dead_code)]
use std::mem;

pub fn arrays(){
    let mut a:[i32;5] = [1,2,3,4,5];
    //its ok too==> let a: = [1,2,3,4,5];
    println!("a has  {} elements and the first element is {}",a.len(),a[0]);
    a[0]=42;
    println!("a[0] is {}",a[0]);
    println!("{:?}",a); // debug output

    if a != [1,2,3,4,5]
    {
        println!("[1,2,3,4,5] does not match!")
    }
    if a == [42,2,3,4,5]
    {
        println!("[42,2,3,4,5] match!")
    }
    // cannot compile cause of different size of arrays!
    /*if a == [4,5]
    {
        println!("match!")
    }*/

    let b = [1;10]; // creates a 10 long arrays of "1'
    //let b = [1u16;10]; // can change the type here
    //let b:[u16;10] = [1;10]; // can change the type here too
    for i in 0..b.len()
    {
        println!("{}",b[i]);
    }
    println!("b takes up {} bytes", mem::size_of_val(&b));

    // MATRICES
    let mtx:[[f32;3];2] =
    [
        [1.0, 0.0,0.0],
        [0.0,2.0,0.0]
    ];
    println!("{:?}",mtx);

    for i in 0..mtx.len() //go through the ROWS
    {
        for j in 0..mtx[i].len()
        {
            if i==j
            {
                println!("mtx[{}][{}]={}",i,j, mtx[i][j]);
            }
        }
    }
}