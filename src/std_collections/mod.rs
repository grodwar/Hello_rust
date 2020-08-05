#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
fn messing_with_idx(mut v: Vec<i32>){
    println!("START OF messing_with_idx");
    //let idx:i32 = 0;// is NOT OK!
    let mut idx:usize = 0;// is OK
    println!("v is {:?}",v);
    println!("v[{}] is {}",idx, v[idx]);
    v[idx]=321;
    println!("v[{}] is updated with {}",idx, v[idx]);

    idx = 111;
    /*v[idx]=321;// creates the usual "index out of bounds" panic
    println!("v[{}] is updated with {}",idx, v[idx]);*/
    //INSTEAD you should use the get function
    // Option type
    //let val = v.get(idx);
    //println!("v[{}] is updated with {:?}",idx, val);// remember to use "{:?}" here cause its an option (???)

    // better way to use an Option
    match v.get(idx) {
        Some(x) => println!("the value is {:?}", x),
        None => println!("Sorry {} is not a correct index!", idx),
    }
    println!("END OF messing_with_idx");
}
pub fn vectors(){
    let mut a = Vec::new();//dont forget it has to be mutable to be useful
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a is {:?}",a);

    a.push(42);
    println!("a is {:?}",a);

    println!("a[0] is {}",a[0]);
    println!("a[{}] is the last element and its value {}",a.len(), a[a.len()-1]);
    messing_with_idx(a);

    //iterating over a vector (had to create b cause something messed up happened to a!
    // error was "borrow of moved value"
    let mut b = Vec::new();//dont forget it has to be mutable to be useful
    b.push(1);
    b.push(2);
    b.push(3);
    for x in &b {println!("{}",x);}
    b.push(44);
    println!("b is {:?}",b);
    b.pop();
    b.pop();
    b.pop();
    b.pop();//if you pop too much you'll get  None
    let last_elm = b.pop();
    //yet  I can still do that with b?!?
    for x in &b {println!("{}",x);}
    println!("last element was {:?}, the vector is now {:?}",last_elm, b);

    let mut c = Vec::new();//dont forget it has to be mutable to be useful
    c.push(1);
    c.push(2);
    c.push(3);
    //while the result of c.pop() yields "Some(x)", we can process and println it
    // once the pop yields a None, we break out of the loop
    while let Some(x) = c.pop()
    {
        println!("popped {}", x);
    }
    println!("c is now empty: {:?}", c);
}

pub fn hashmaps()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides",shapes["square".into()]);
    // if we try to access a missing key from the hashmap
    // we get "panicked at 'no entry found for key'"
    // println!("a circle has {} sides",shapes["circle".into()]);
    let my_shape = "circle";
    //did not manage to do it with a match (cause not sure what happens when the key is not found
    //see v2
    if shapes.contains_key(my_shape) {
        println!("a {} has {} sides", my_shape, shapes[my_shape.into()]);
    }
    else {
        println!("{} is unknown", my_shape);
    }
    //updating value
    shapes.insert("square".into(), 42);
    println!("{:?}", shapes); // not very useful, better to iterate, see v2
    // insert a value if the key is not there yet
    shapes.entry("circle".into()).or_insert(1);
    println!("circle has now {} side",shapes["circle".into()]);
    println!("all shapes BEFORE updating circle: {:?}", shapes);
    {//borrowing the hashmap as mutable ???
        //reference to the actual value of the circle key
        let actual = shapes.entry("circle".into()).or_insert(2);
        // this is really STUPID!!
        *actual = 0; // that changes the value to 0, so now circle's value is 0
    }
    println!("all shapes AFTER updating circle: {:?}", shapes);
}

pub fn hashmaps_v2(){

    // actually got it from https://doc.rust-lang.org/stable/rust-by-example/std/hash.html
    println!("Cleaner version of the hashmaps!!");
    let mut shapes = HashMap::new();
    shapes.insert("triangle", 3);
    shapes.insert("square", 4);
    shapes.insert("square", 42);
    let my_shape = "circle";
    match shapes.get(my_shape) {
        Some(&number) => println!("{} has {} sides", my_shape, number),
        None => println!("{} is unknown", my_shape)
    }

    for (key, value) in &shapes //don't forget the &
        // if not it has weird side effects
    {
        println!("{}: {} sides", key,value);
    }
    println!("Using the iter() function");
    for (key, value) in shapes.iter()
    {
        println!("{}: {} sides", key,value);
    }
}

pub fn sets()
{
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("alpha");
    greeks.insert("omega");
    println!("{:?}",greeks);
    greeks.insert("gamma");//its set, stores only unique element
    println!("{:?}",greeks);

    let added_vega = greeks.insert("vega");
    //let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega")
    }
    if !greeks.contains("kappa")
    {
        println!("we dont have kappa yet")
    }
    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta")
    }
    let removed2 = greeks.remove("delta");
    if removed2 {
        println!("we removed delta AGAIN")
    }
    // playing with sets
    let _1_5: HashSet<_> = (1..=5).collect();// collect to make it a collection
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (6..=8).collect();

    //subset
    println!("is {:?} a subset of {:?}: {}",_2_8,_1_10, _2_8.is_subset(&_1_10));
    // disjoint
    println!("is {:?} and  {:?} disjoint?: {}",_1_5,_6_10, _1_5.is_disjoint(&_6_10));
    //union
    println!(" {:?} +  {:?} =  {:?}",_1_5,_6_10, _1_5.union(&_6_10));
    //intersection
    println!(" {:?} intersection  {:?} =  {:?}",_1_10,_6_10, _1_10.union(&_6_10));
    //difference
    println!(" {:?} -  {:?} =  {:?}",_1_10,_6_10, _1_10.difference(&_6_10));
    //symmetric difference = union - intersection
    println!(" {:?} sym diff  {:?} =  {:?}",_2_8,_6_10, _2_8.symmetric_difference(&_6_10));
}