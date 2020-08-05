#![allow(dead_code)]

fn how_many(x:i32) ->  &'static str // "'static" is weird !
{
    match x
    {
        0 => "no",
        1 | 2 => "one or two",
        //9...11 => "a few", // range patterns are deprecated
        9..=11 => "a few", //proper implementation
        //z @ 9..=11 => "a few", //naming the range for later use
        12 => "a dozen",
        _ if (x %2 == 0) => "some (even)",
        _ => "a lot of"
    }
}

pub fn pattern_matching()
{
    for x in 0..13
    {
        println!("{}: I have {} oranges!",x, how_many(x));
    }

    let point = (0,7);
    match point
    {
        (0,0) => println!("origin"),
        (0,y) => println!("lies on x axis and y={}",y),
        (x,0) => println!("lies on y axis and x={}",x),
        (x,y) => println!("({},{})",x,y)
        //(ref mutx,y) => println!("({},{})",x,y) // you can pass by reference and mutable one
    }
    // back to enumeration (check the match about black)
    enum Color
    {
        Red,
        Green,
        Blue,
        RgbColor(u8,u8,u8), //tuple, components cant be named
    CmykColor {cyan:u8, magenta:u8, yellow:u8,black:u8} //struct-like, components can be named
    }

    let c:Color = Color::CmykColor { cyan: 0, magenta: 0, yellow: 0, black: 255 };
    match  c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor {black: 255, .. }=>println!("black"),
        // here we care only about black so we put ",.." after to mean wedont care about the rest
        Color::RgbColor(r,g,b) =>println!("rgb({},{},{})", r, g, b),
        Color::CmykColor { cyan:c, magenta:m, yellow:y, black: k }=>println!("cmyk({},{},{},{})", c, m, y, k)
    }
}