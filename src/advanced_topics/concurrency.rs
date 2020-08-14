use std::thread;
use std::time;



pub fn concurrencyv1(){

    for _ in 1..10 {
        print!("-", );
        thread::sleep(time::Duration::from_millis(300));

    }
    println!(); // to go to new line!
    println!("end of concurrencyv1!");
}

pub fn concurrencyv2(){
    let handle = thread::spawn(||{
        for _ in 1..10 {
            print!("+", );
            thread::sleep(time::Duration::from_millis(500));

        }
    });
    for _ in 1..10 {
        print!("-", );
        thread::sleep(time::Duration::from_millis(300));

    }
    println!(); // to go to new line!
    println!("end of concurrencyv2 main thread!");
    let _result = handle.join();// we need to make sure we wait for the spawned thread to be finished
    //added "let _result" to avoid some warning
    println!(); // to go to new line!
    println!("after join!");
}