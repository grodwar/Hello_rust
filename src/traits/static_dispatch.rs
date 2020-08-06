
//version 1
// simple trait  to make it printable
trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32:  {}",*self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("String:  {}",*self)
    }
}

// version 2
fn print_it<T: Printable>(z:T)
{
    println!("{}", z.format());
}//monomorphisation: there will be a concrete implementation for each type T that you actually use:
//fn print_it(z:String){...} AND fn print_it(z:i32){...} in the present case
// ==> this is static dispatch (cause its done a compile time)
pub fn static_dispatch()
{
    let a = 23;
    let b = "hello".to_string();
    //version 1
    println!("{}",a.format());
    println!("{}",b.format());
    //version 2
    println!("--version2--");
    print_it(a);
    print_it(b);
}