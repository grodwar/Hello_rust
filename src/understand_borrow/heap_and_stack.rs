#![allow(dead_code)]

//////// the STACK
// NB underscored the variable names to get rid of the warnings
fn foo(){
    let _y = 5;
    let _z = 100;
}

fn italic() {
    let _i = 6;
}

fn bold() {
    let _a = 5;
    let _b = 100;
    let _c = 1;

    italic();
}

pub fn fake_main()
{
    let _x = 42;// that basic values ‘go on the stack’
        //Well, when a function gets called, some memory gets allocated for all of its local variables and
        // some other information. This is called a ‘stack frame’
    //When the function exits, its stack frame gets deallocated
    //foo();
    bold();
}
/*
When main is called:
Address	Name	Value
0	x	42

When bold is called
Address	Name	Value
3	c	1
2	b	100
1	a	5
0	x	42

When italic  is called
Address	Name	Value
4	i	6
3	c	1
2	b	100
1	a	5
0	x	42

When italic  is finished
Address	Name	Value
3	c	1
2	b	100
1	a	5
0	x	42

When bold  is finished
Address	Name	Value
0	x	42

and then nothing
 */



//////// the HEAP
//Sometimes, you need to pass some memory between different functions, or keep it alive for longer
// than a single function’s execution. For this, we can use the heap.

fn fake_main2() {
    let _x = Box::new(5);
    let _y = 42;
// We allocate space for two variables on the stack. y is 42, as it always has been,
// but what about x? Well, x is a Box<i32>, and boxes allocate memory on the heap.
// The actual value of the box is a structure which has a pointer to ‘the heap’.
}
/*
when main is called
Address	Name	Value
1	y	42
0	x	??????

When we start executing the function, and Box::new() is called
Address	Name	Value
(230) - 1		5
...	...	...
1	y	42
0	x	→ (230) - 1

because the heap can be allocated and freed in any order, it can end up with ‘holes’.
 */


////// ARGUMENTS AND BORROWING
fn foo3(_i: &i32) {
    let _z = 42;
}

fn fake_main3() {
    let x = 5;
    let y = &x;

    foo3(y);
}

//  when we call foo(), passing y as an argument
//
// Address	Name	Value
// 3	z	42
// 2	i	→ 0
// 1	y	→ 0
// 0	x	5


///// A COMPLEX EXAMPLE

fn foo4(x: &i32) {
    let y = 10;
    let z = &y;

    baz(z);
    bar(x, z);
}

fn bar(_a: &i32, _b: &i32) {
    let _c = 5;
    let d = Box::new(5);
    let e = &d;

    baz(e);
}

fn baz(_f: &i32) {
    let _g = 100;
}

fn fake_main4() {
    let h = 3;
    let _i = Box::new(20);
    let j = &h;

    foo4(j);
}
