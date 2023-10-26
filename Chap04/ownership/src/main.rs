fn main() {
    // 변수 s가 범위 내에 생성된다.
    let s = String::from("hello");

    // 변수 s의 값이 함수 내로 이동한다.
    // 그리고 이 시점부터 변수 s는 더 이상 유효하지 않다.
    takes_ownership(s);

    // 변수 x가 범위 내에 생성된다.
    let x = 5;

    // 변수 x의 값이 함수 내로 이동한다.
    // 하지만 i32 타입은 복사를 수행하므로 유효하다.
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
