fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut count = 0;
    'counting_up: loop {
        // label 'counting_up
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Returning Values from Loops

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // While Loop
    let mut number = 3;
    
    while number !=0 {
        println! ("{}!", number);
        number -=1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { // this while loop is slow and error prone
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // For in Loop
    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev(){ // rev is reverse, (1..4) is range
        println! ("{}!", number);
    }
    println!("LIFTOFF!!!");

}
