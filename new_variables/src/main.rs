fn main() {
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant value: {}", THREE_HOURS_IN_SECONDS);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {}", y)
    }
    println!("The value of x is : {}", x);

    // Floating Point
    let x = 2.0; // f64 is default
    let y: f32 = 3.0; // f32

    // numeric opertions
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // result in 0
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    // The char type

    let c = 'c'; // must be single qoutes
    let z = 'z';
    let heart_eyed_cat = '😻';

    // Compound types
    // tupes and arrays

    // The Tuple - same as in python
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; // 0 is index , index calling is different
    let six_point_fout = x.1;
    let one = x.2;

    // Array - fixed length, like tuples. (this different)
    let a = [1, 2, 3, 4, 5]; // array must be a same data type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5] ;// is equals to let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
}
