#![allow(dead_code)]


//fn use_slice(slice: &[i32]) // takes a part of an array of i32
fn use_slice(slice: &mut[i32]) // now it is mutable
{
    println!("the second element is {}", slice[1]);
    println!("the length of the slice is  {}", slice.len());
    //println!("the length is  {}", slice[10]); // usual index out of bounds error at runtime
    println!("{:?}", slice);
    slice[0]=42;
}
pub fn slices()
{
    let mut data = [1,2,3,4,5]; // size known at compile time
    //use_slice(&data[1..4]);// takes elements at index 1,2,3
    use_slice(&mut data[1..4]);
    use_slice(&mut data);
}