#![allow(dead_code)]


/*
struct Point // TO BE REPLACED
{
    x:f64,
    y:f64
}
 */
struct Point<T>
{
    x:T,
    y:T
}

struct Line<T>
{
    start: Point<T>,
    end: Point<T>
}
pub fn generics()
{
    //NB we prefix a,b,myLine with "_" to avoid warnings at compile time
    let _a = Point {x:0,y:0};
    let _b:Point<f64> = Point {x:1.2,y:4.2}; //explicitly define the type of T
    //println!("a is {:?}", a); //dont work
    //println!("b is {}", b); //dont work

    //let myline = Line {start:a, end:b};//mismatched types
    let _myline = Line {start:Point{x:2,y:3}, end:Point{x:10,y:42}};//same types so its ok
    //println!("myline is {}", myline); // dont work


}