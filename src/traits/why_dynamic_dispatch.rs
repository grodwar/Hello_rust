
struct Circle{radius:f64}
struct Square{side:f64}

//self note: a trait cannot derive another trait
// because we dont know what will be the actual type of the object
trait Shape
{
    fn area(&self) -> f64;
}

impl Shape for Square
{
    fn area(&self) -> f64 {self.side*self.side}
}
impl Shape for Circle
{
    fn area(&self) -> f64 {std::f64::consts::PI*self.radius*self.radius}
}

pub fn why()
{
    let shapes:[&dyn Shape; 4] = [//"dyn" was not present in the lecture (its a new feature?)
        &Circle{radius:1.0},
        &Square{side:2.0},
        &Circle{radius:3.0},
        &Square{side:4.0},
    ];

    for (i, shape) in shapes.iter().enumerate()
    {
        println!("Shape #{} has an area of {}",i,shape.area());
        //here we DEFINITELY need DYNAMIC DISPATCH
    }
}
