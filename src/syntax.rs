fn main() {
    println!("Hello, world!");

    //Variables
    let a: i32 = 7;  //Specified value
    let mut b: i32 = 10;
    let c = 12;
    let d = "Hello";

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);

    //Expressions
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
    
    b = 12;
    println!("Changed b value to {}", b);

    println!("Modified b value: a + b = {}", a + b);

    //Conditions
    if a == 8 {
        println!{"a = 8"};
    }

    else if a == 7 {
        println!{"a = 7"}
    }

    else {
        println!{"a is neither 8 or 7"}
    }

    //Loop
    let mut i = 0;

    while i < 12 {
        println!("i = {}", i);
        i = i + 1;
    }

    let mut sum = 0;
    for n in 1..12 {
        sum += n;
    }
    println!("sum: {}", sum);
}
