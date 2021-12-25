fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(5, 'h');

    // statement and expression
    // * Statements are instructions that perform some 
    //   action and do not return a value.
    // * Expressions evaluate to a resulting value. 
    let y = 6; // it's satement ,end with semicolon

    let y = {
        let x = 3;
        x+1 // expression , 
    };
    println!("The value of y is: {}", y);

    // Functions with Return Values
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);


}

fn another_function(){
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {}{}", value, unit_label);
}

fn five()->i32{
    5 // return is expression so no-semi colon
}

fn plus_one(x: i32) -> i32{
    x + 1
}