fn main() {
    println!("Hello, world!");

    let number = 3;

    if number !=0 { // if condition must be bool, if number {} cause error
        println!("number was something other than zero");
    }

    if number < 6 { 
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0{
        println!("number is divisible by 3");

    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement

    let condition = true;
    let number = if condition {5} else {6}; // must be single data type if and else block
    println!("The value of number is: {}", number);
}
