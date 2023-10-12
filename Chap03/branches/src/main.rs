fn main() {
    let number = 3;

    if number < 5 {
        println!("It's true");
    } else {
        println!("It's false");
    }

    if number % 4 == 0 {
        println!("It's multiple of four");
    } else if number % 3 == 0 {
        println!("It's multiple of three");
    } else if number % 2 == 0 {
        println!("It's multiple of two");
    } else {
        println!("It's not a multiple of two, three, or four");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("number : {}", number);
}
