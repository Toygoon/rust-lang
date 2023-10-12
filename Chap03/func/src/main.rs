fn main() {
    println!("Hello, world!");

    // another_function();
    // parameter_function(5, 6);
    // statement_err();
    // statement_ex();
    println!("{}", five());
}

fn another_function() {
    println!("Another function!");
}

fn parameter_function(x: i32, y: i32) {
    println!("x : {}, y : {}", x, y);
}

fn statement_ex() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x : {}, y : {}", x, y);
}

fn statement_err() {
    // let x = (let y = 6);
}

fn five() -> i32 {
    5
}