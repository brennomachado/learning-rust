use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of X in the inner scope is: {}", x);
    }

    println!("The value of X is: {}", x);

    //characters
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("Characters {}, {}, {}", c, z, heart_eyed_cat);

    //tuples

    let tup = (500, 6.4, 1);

    let (x, y, _) = tup;

    println!(
        "The values of tup is {:?}, and x is {}, y is {} and z is {}",
        tup, x, y, tup.2
    );

    // arrays

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!(
        "The array a is: {:?}. The 1°element is {} and the second element is {}",
        a, first, a[1]
    );

    // trying to access invalid values of an array
    println!("\n ## Trying to access invalid values of an array ##\n");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index]; //this line crash if user input number greater than 4, the max index of array "index"

    println!("The value of the elment at index {} is: {}", index, element);
}
