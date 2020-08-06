use std::ops::{Add, AddAssign, Neg};
//use std::cmp::PartialEq; // for some reason its not required for PartialEq



#[derive(Debug)]

//Version 1 (without the Add trait)
struct Complex<T>
{
    re: T,
    im: T
}

//factory
impl<T> Complex<T>
{
    fn new (real:T, imaginary:T) -> Complex<T>
    {
        Complex::<T> { re: real,im:imaginary }
    }
}

//Version 2 (with the Add trait)
/*
impl Add for Complex<i32>
{
    type Output = Complex<i32>;//type you have to specify for the trait to work
    //type of the result of the "+" operation in this case  (i32 cause we add i32s)
    //a+b
    // self is a reference to the object, Self is a reference to the type (Complex<i32>)
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}
 */

//Version 3 (with the Add trait for any type T support Add trait)
impl<T> Add for Complex<T>
where T: Add<Output = T>
{
    type Output = Complex<T>;//type you have to specify for the trait to work
//type of the result of the "+" operation in this case  (i32 cause we add i32s)
//a+b
// self is a reference to the object, Self is a reference to the type (Complex<i32>)
fn add(self, rhs: Self) -> Self::Output {
    Complex {
        re: self.re + rhs.re,
        im: self.im + rhs.im
    }
}
}

//part 3
impl<T> AddAssign for Complex<T>
where T:AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re+=rhs.re;
        self.im+=rhs.im;
    }
}

//part 4
impl<T> Neg for Complex<T>
where T:Neg<Output=T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

//part 5
// full equality: x == x ... doesn't work with float cause
//NAN not a number, e.g. e/0 inf/inf
// NAN == NAN -> false
// so for numbers we go for...:
// partial equality

impl<T> PartialEq for Complex<T>
where T:PartialEq<T>
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

// part 6
// you  can also simply #[derive(PartialEq, Eq, Ord, PartialOrd)]


pub fn operator_overloading()
{
    //part 1
    let  a = Complex::new(1,2);
    let  b = Complex::new(3,4);

    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("a+b={:?}", a+b); // before implementation we get
    //error[E0369]: cannot add
    // `traits::operator_overloading::Complex<{integer}>` to
    // `traits::operator_overloading::Complex<{integer}>`


    //part 2
    // trying with f64
    //before implementation of the overloaded + operator for f64, we get:
    //  error[E0369]: cannot add `traits::operator_overloading::Complex<{float}>` to
    // `traits::operator_overloading::Complex<{float}>`
    let  c = Complex::new(1.0,2.0);
    let  d = Complex::new(3.0,4.0);

    println!("c={:?}", c);
    println!("d={:?}", d);
    println!("c+d={:?}", c+d); // before implementation we get

    //part 3
    let  e = Complex::new(1,2);
    let  mut f = Complex::new(3,4);
    println!("e is ={:?}", e);
    println!("f is ={:?}", f);
    f+=e;//error[E0368]: binary assignment operation `+=` cannot be applied to type
    // `traits::operator_overloading::Complex<{integer}>`
    println!("f is now ={:?}", f);

    //part 4
    let  g = Complex::new(1,2);
    println!("g is ={:?}", g);
    println!("-g is ={:?}", -g);// before implementation we get
    // error[E0600]: cannot apply unary operator `-`
    // to type `traits::operator_overloading::Complex<{integer}>`
    // part 5
    let  h = Complex::new(1,2);
    let  i = Complex::new(1,2);
    println!("h==i -> {:?}", h==i);//when not implemented
    //error[E0369]: binary operation `==` cannot be applied to type
    //`traits::operator_overloading::Complex<{integer}>`
    let  j = Complex::new(10.1,42.0);
    let  k = Complex::new(10.0,42.0);
    println!("j==k -> {:?}", j==k);//when not implemented


}