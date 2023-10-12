fn fn_mut() {
    let mut x = 5;
    println!("x의 값 : {}", x);
    x = 6;
    println!("x의 값 : {}", x);
}

fn fn_shadow() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("x의 값 : {}", x);
}

fn fn_mut_err() {
    let mut spaces = "\t";
    // spaces = spaces.len();
}

fn fn_fp() {
    // f64
    let x = 2.0;

    // f32
    let y: f32 = 3.0;
}

fn fn_arith() {
    // Sum
    let sum = 5 + 10;

    // Difference
    let diff = 95.5 - 4.3;

    // Multiply
    let mul = 4 * 30;

    // Division
    let div = 56.7 / 32.2;

    // Modular
    let modular = 43 % 5;
}

fn fn_bool() {
    let t = true;
    let f: bool = false;
}

fn fn_char() {
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
}

fn fn_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("y : {}", y);
}

fn fn_tuple_idx() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let first = x.0;
    let second = x.1;
    let third = x.2;
}

fn fn_array() {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    let element = a[999];
}

fn main() {
    // fn_mut();
    // fn_shadow();
    // fn_mut_err();
    // fn_tuple();
    fn_array();
}
