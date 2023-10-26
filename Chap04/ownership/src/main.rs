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

    // gives_ownership 함수의 리턴값이 변수 s1로 옮겨진다.
    let s1 = gives_ownership();
    // 변수 s2가 범위 내에 생성된다.
    let s2 = String::from("hello");
    // 변수 s2가 takes_and_give_back 함수로 옮겨간 후 리턴값은 변수 s3로 옮겨진다.
    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership 함수의 리턴값은 호출한 함수로 옮겨진다.
    // 변수 some_string이 범위 내에 생성된다.
    let some_string = String::from("hello");

    // some_string 변수가 리턴되면 호출한 함수로 옮겨진다.
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // 변수 a_string을 리턴하면 그 값이 호출한 함수로 옮겨진다.
    a_string
}
