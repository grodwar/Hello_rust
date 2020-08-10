

pub fn lifetime_and_memory()
{
    //just for fun
    let mut _w = Vec::new();
    _w.push(1);
    _w.push(2);
    _w.push(3);
    let _w2 = Vec::from([1,2,3,4]);
    //just for fun
    // v is on the stack but the data inside is on the heap
    // and v owns the memory
    let v = vec![1,2,3];//a macro
    let v2 = v;// it copies a pointer to that memory but ican introduce a race condition
    //println!("{:?}",v); // error[E0382]: borrow of moved value: `v`
    println!("{:?}",v2); // this is ok
    // Rust guarantees that only one variable point to the data
    // so it invalidated v

    //it's OK with primitive types cause creating a new prim type is copied as it does not cost
    // much memory (and that it is not a pointer)
    let u = 1;
    let u2 = u;
    println!("u is {}",u);
    println!("u2 is {}",u2);
    // but if we Box it to put it on the heap it does not work anymore
    let u3 = Box::new(1);
    let u4 = u3;
    //println!("u3 is {}",u3);// error[E0382]: borrow of moved value: `u3`
    println!("u4 is {}",u4);

    // how to avoid this issue with functions
    let print_vector = |x:Vec<i32>|->Vec<i32>
        {
            println!("{:?}",x);
            x//returns x !!!
        };
    let v3 = vec![1,2,3];
    let v4  = print_vector(v3); // so we can still use whats inside v3
    // but its inconvenient so that's why borrowing exist
    println!("v3 or v4 is {:?}", v4);

}