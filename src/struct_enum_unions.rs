#![allow(dead_code)]

struct Point
{
    x: f64,
    y: f64

}

struct Line
{
    start: Point,
    end: Point
}

pub fn structures(){
    let p = Point { x:3.0, y: 4.0};
    println!("point p is at coordinates ({}, {})", p.x, p.y);

    let p2 = Point{ x: 5.0, y: 10.0};
    let myline = Line{ start:p, end:p2};
    println!("line starts at  ({}, {}) and ends at  ({}, {})", myline.start.x, myline.end.x, myline.start.y, myline.end.y);
}

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple, components cant be named
CmykColor {cyan:u8, magenta:u8, yellow:u8,black:u8} //struct-like, components can be named
}

pub fn enumerations(){
    //let c:Color = Color::Red;
    //let c:Color = Color::RgbColor(0,10,0);
    //let c:Color = Color::RgbColor(0,0,0);
    //let c:Color = Color::CmykColor { cyan: 0, magenta: 128, yellow: 0, black: 0 };
    let c:Color = Color::CmykColor { cyan: 0, magenta: 0, yellow: 0, black: 255 };
    match  c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        // dont forget that it has to be exhaustive
        // "_" case means catchall (the rest)
        //_ => println!("some other colors") // gives "unreachable pattern" warning atm
        Color::RgbColor(0,0,0)
        | Color::CmykColor { cyan:_, magenta:_, yellow:_, black: 255 }=>println!("black"),
        Color::RgbColor(r,g,b) =>println!("rgb({},{},{})", r, g, b),
        Color::CmykColor { cyan:c, magenta:m, yellow:y, black: k }=>println!("cmyk({},{},{},{})", c, m, y, k),
        //_ => println!("Unknown color")//unnecessary
    }

}

// UNIONS
union IntOrFloat //camel case !!
{
    i: i32,
    f: f32 // but we dont actually know whats inside (int or float)
}


pub fn unions()
{
    let mut iof = IntOrFloat{i:123};
    iof.i = 234;
    //self exploration:
    unsafe {
        //println!("iof is {}", iof.f);// some random number I guess??
        println!("iof is {}", iof.i);
    }
    // end of self exploration
    let value = unsafe{ iof.i };
    println!("value of iof is {}", value);


    process_value(iof);
    let iof2 = IntOrFloat{f:47.2};
    process_value(iof2);
    let iof3 = IntOrFloat{i:5};
    process_value(iof3);
}

fn process_value(iof: IntOrFloat)
{
    println!("process_value:");
    unsafe
    {
        match iof {
            IntOrFloat {i:42} => {println!("meaning of life")},
            IntOrFloat{f} =>{println!("value = {}", f)}
            //IntOrFloat{i} =>{println!("value = {}", i)}// cant do that, cause i or f are not related to the union definition
            // so here any non-42 int will be treated as floats!
            //_ => {println!("we dont know!")}
        }
    }
}