fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };

    println!("result : {}, counter : {}", result, counter);

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("Shoot!");

    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("{}", i);
    }

    for i in (1..4).rev() {
        println!("{}", i);
    }
    println!("Shoot!");
}
