


pub fn borrowing(){
    //compare this ownership.rs
    let print_vector = |x:&Vec<i32>|
        {
            println!("x[0]:{}", x[0]);
            //immutable reference so we cant do
            //x.push(5);
        };
    //1 basic case
    let v = vec![3,2,1];
    print_vector(&v);//print_vector does not have control v but it borrows v
    println!("v[0]:{}", v[0]);//still ok

    //2 mutable reference ... it doesn't work the way the lecture shows
    let  mut a = 40;
    //within a scope
    {
        let b = &mut a; // mutable reference
        *b +=2; // access the value of what is referenced
    }
    println!("a after b ={}",a);
    let c = &mut a; // mutable reference
    *c+=2;
    println!("a after c ={}",a); // doesn't work in the lecture, but here it works

    //part 3 ?
    //let mut z = vec![3,2,1];
    let z = vec![3,2,1];
    for i in &z
    {
        println!("i={}",i);
        //z.push(4); //error[E0502]: cannot borrow `z` as mutable because
        // it is also borrowed as immutable
        //also  it would not make sense to add element everytime we access one

    }

}