#![allow(dead_code)]

pub fn if_statement()
{
    let temp = 35;
    if temp > 30
    {
        println!("it's hot outside!")
    }
    else if temp <10
    {
        println!("it's really cold outside!")
    }
    else
    {
        println!("it's OK outside")
    }
    // if statements are expression:
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("is it....{}",
             if temp > 20 {"sunny"} else if temp < 10 {"cloudy"} else {"OK"}
    );

    // ifs inside if
    println!("it is {}",
        if temp > 20
        {
            if temp > 30 {"very hot"}
            else {"hot"}
        }
        else if temp <10 {"cold"}
        else {"OK"}

    );

}