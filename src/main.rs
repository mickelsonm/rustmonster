//Hello Rusty!

struct Point{
	x: i32, //int 32
	y: i32
}

fn main() {
    println!("Hello, world!");

    basic_if_example();
    switch_example();
    for_loop_example();
    while_loop_example();
    infinite_loop_example();
}

fn basic_if_example(){
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