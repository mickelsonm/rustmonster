//Hello Rusty!

struct Point{
	x: i32, //int 32
	y: i32
}

fn main() {
    println!("Hello, world!");

    basic_variables();
    basic_if_example();
    switch_example();
    for_loop_example();
    while_loop_example();
    infinite_loop_example();
    we_call_a_function();
}

fn basic_variables(){
    println!("using basic variables...");
    let (x, y, z) = (1, 3, 5);
    println!("x = {}, y = {}, z = {}", x, y, z);

    //using a data type
    let xx: i32 = 5;
    println!("xx = {}", xx);

    let name: &str = "peter";
    println!("name = {}", name);

    //using a mutable type
    //without the 'mut' keyword it throws an error on reassignment
    let mut yy = 5; // mut yy: i32
    if yy == 5{ //if we don't read/do something with it...it throws a warning
        yy = 10;
    }
    println!("yy = {}", yy);

    //tuple: ordered list of a fixed size
    let tuple = ("Frank", "Tom", "Norm");
    let f = tuple.0;
    println!("f = {}", f);
}

fn basic_if_example(){
    println!("using basic if statement...");
    let name = "rusty";
    //this is kind of cool, hmm
    let is_rusty = if name == "rusty"{
        true
    } else{
        false
    };

    if is_rusty {
        println!("Hi Rusty!");
    }
}

fn switch_example(){
    let x = 5;
    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    println!("number = {}", number);
}

fn for_loop_example(){
    println!("running the for loop...");
    for x in 0..10 {
        println!("x = {}", x);
        match x{
            1 => println!("one for the money"),
            2 => println!("two for the show"),
            3 => println!("three to get ready"),
            4 => println!("four to go!"),
            5 => println!("Johnny Five is alive!"),
            _ => println!(""),
        }

        let pt = Point{x: x, y: x*2};
        println!("pt.x = {}, pt.y = {}", pt.x, pt.y);
    }   
}

fn while_loop_example(){
    println!("running while loop...");
    let mut aa = 10;
    while aa > 0{
        aa -= 1;
        println!("{}", aa);
    }
}

fn infinite_loop_example(){
    println!("running infinite loop...");
    let mut count = 0u32;
    loop {
        count += 1;
        println!("{}", count);
        if count == 3{
            println!("THREE!");
        } else if count == 5{
            println!("DONE!");
            break;
        }
    } 
}

fn we_call_a_function(){
    println!("We call a function...");
    let a = foo(42);
    println!("a = {}", a);

    void_function();
}

//this function returns a value
fn foo(x: i32) -> i32 {
    return x;
}

fn void_function() {
    println!("calling a function that doesn't return anything");
}