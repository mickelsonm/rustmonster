//Hello Rusty!

struct Point{
	x: i32, //int 32
	y: i32
}

fn main() {
	let xx = 42; //int obviously
	let name = "rusty";

    println!("Hello, world!");
    println!("xx = {}, name = {}", xx, name);

    //this is kind of cool, hmm
    let isRusty = if name == "rusty"{
    	true
    } else{
    	false
    };

    if isRusty {
    	println!("Hi Rusty!");
    }


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

    for x in 0..10 {
    	println!("x = {}", x);
    	match x{
    		1 => println!("one for the money"),
    		2 => println!("two for the show"),
    		3 => println!("three to get ready"),
    		4 => println!("four to go!"),
    		5 => println!("Johnny Five is alive!"),
    		_ => println!("just a number..."),
    	}

    	let pt = Point{x: x, y: x*2};
    	println!("pt.x = {}, pt.y = {}", pt.x, pt.y);
    }
}