# Chapter 03. 일반 프로그래밍 개념

- 이번 장에서는 거의 모든 프로그래밍 언어에서 찾아볼 수 있는 개념들을 러스트는 어떻게 표현하는지 중점적으로 살펴본다.
- 이번 장에서는 변수, 기본 타입, 함수, 주석, 흐름 제어 등에 대해 알아본다.

<details>
<summary>Table of Contents</summary>

- [3-1 변수와 가변성](#3-1-변수와-가변성)
- [3-2 데이터 타입](#3-2-데이터 타입)

</details>

---

# 3-1 변수와 가변성

- 기본적으로 변수는 변경이 불가능하다.
- 안전하면서도 쉽게 동시성의 장점을 활용할 수 있는 코드를 작성하도록 돕기 위해 러스트가 제공하는 다양한 특징 중 하나다.
- 러스트는 불변성(immutability)을 기본으로 하는데, 아래의 코드를 실행하면 다음과 같은 오류가 발생한다.

```
fn main() {
    let x = 5;
    println!("x의 값 : {}", x);
    x = 6;
    println!("x의 값 : {}", x);
}
```

```
> cargo run
   Compiling variables v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap03/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("x의 값 : {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to previous error
```

- 위 에러의 원인은 불변 변수 `x`에 값을 두 번 할당했기 때문이다.
- 하지만 때로는 가변성이 필요한 경우도 있다.
- 변수의 불변성은 기본 변수 선언문을 사용했을 때만 적용된다.
- `mut` 키워드는 값을 변경할 수 있는 변수를 선언할 수 있다.
- 아래는 코드를 변경하여 컴파일 및 실행한 예시이다.

```
fn main() {
    let mut x = 5;
    println!("x의 값 : {}", x);
    x = 6;
    println!("x의 값 : {}", x);
}
```

```
> cargo run
   Compiling variables v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap03/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/variables`
x의 값 : 5
x의 값 : 6
```

- 이번에는 `mut` 키워드를 사용했으므로 변수 `x`의 값이 변경되었다.
- 대용량 데이터 구조를 사용하면, 데이터의 인스턴스를 가변형으로 선언하는 것이 훨씬 더 빠르다.
- 따라서, 명확성을 위해 약간의 성능 하락을 감수하는 것이 더 나을 수도 있다.

## 변수와 상수의 차이점

- 변수의 값을 변경할 수 없다는 개념은 상수(constants)에 가깝다.
- 상수에는 `mut` 키워드를 사용할 수 없다.
- 상수는 기본 선언만으로 불변 속성이 아니라 항상 불변이다.
- 상수는 `let` 대신 `const` 키워드를 이용해 선언하며, 타입을 반드시 지정해야 한다.
- 상수는 전역 범위를 비롯해 원하는 어떤 범위 내에도 선언할 수 있으므로 필요한 곳이라면 코드의 어느 곳에서도 그 값을 쉽게 사용할 수 있다.
- 러스트는 상수 이름에 대문자만 사용하며 단어 사이에 밑줄을 추가하는 규칙을 사용한다.

```
const MAX_POINTS: u32 = 100_000;
```

## 변수 가리기

- 첫 번째 변수가 두 번째 변수에 의해 가려졌다고 표현한다.
- 변수의 이름을 참조하면 두 번째 변수의 값이 사용된다.
- 변수를 가리려면 이미 선언된 변수와 똑같은 이름 변수를 `let` 키워드를 통해 다시 선언하면 된다.

```
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("x의 값 : {}", x);
}
```

```
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/variables`
x의 값 : 12
```

- 변수 가리기는 `mut` 키워드를 이용하는 방법과는 다르다.
- `let` 키워드를 이용해서 변수를 새로 선언하지 않고 값만 할당하면 컴파일 에러가 발생하기 때문이다.
- 다음과 같이 `mut` 키워드를 사용하면 컴파일 에러가 발생한다.

```
let mut spaces = "\t";
spaces = spaces.len();
```

```
> cargo run
   Compiling variables v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap03/variables)
error[E0308]: mismatched types
  --> src/main.rs:18:14
   |
17 |     let mut spaces = "\t";
   |                      ---- expected due to this value
18 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`
   |
help: try removing the method call
   |
18 -     spaces = spaces.len();
18 +     spaces = spaces;
   |

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` (bin "variables") due to previous error
```

---

# 3-2 데이터 타입

- 데이터 타입의 두 가지 집합인 `Scalar`와 `Compound`에 대해 알아보자.
- 러스트는 정적 타입 언어이다. 즉, 컴파일 시점에 모든 변수의 타입이 결정되어야 한다는 뜻이다.
- 여러 타입을 사용할 수 있을 때는 `Type Annotation`을 이용해 타입을 명시해 주어야 한다.

```
let guess: u32 = "42".parse().expect("Not a number.");
```

## 스칼라 타입

- `Scalar` 타입은 하나의 값을 표현한다.
- 러스트는 `integer`와 `floating point numbers`, `booleans`, `characters` 등 네 가지 종류의 스칼라 타입을 정의하고 있다.

### 정수 타입

- 아래는 러스트의 정수 타입이다.

| Size   | Signed | Unsigned |
| ------ | ------ | -------- |
| 8 bit  | i8     | u8       |
| 16 bit | i16    | u16      |
| 32 bit | i32    | u32      |
| 64 bit | i64    | u64      |
| arch   | isize  | usize    |

- 아래는 러스트의 정수 리터럴이다.
- 바이트를 제외한 모든 숫자 리터럴에는 타입 접미사`(suffix)`를 붙일 수 있으며, `1_000`과 같이 밑줄을 이용해 자릿수를 표현할 수도 있다.

| Literal Type   | Sample      |
| -------------- | ----------- |
| Decimal        | 98_2222     |
| Hex            | 0xff        |
| Octal          | 0o77        |
| Binary         | 0b1111_0000 |
| Byte (u8 only) | b'A'        |

### 정수 오버플로

- 디버그 모드로 컴파일할 경우 러스트는 정수 오버플로에 대한 검사를 추가해서 런타임에 프로그램에서 패닉이 발생하지 않도록 한다.
- 러스트는 에러가 발생해서 프로그램이 중단되는 현상을 `panicking`이라고 부른다.
- 러스트는 `--release` 플래그를 이용해서 릴리즈 모드로 컴파일할 때는 오버플로에 대한 검사를 추가하지 않는다.
- 대신 오버플로가 발생하면 러스트는 이를 보완하기 위해 두 개의 `wrapping`을 추가한다.
- `u8` 형식의 경우 `256`은 `0`이 되고 `257`은 `1`이 되는 식이다.
- 이렇게 하면 변수가 기대하지 않은 값을 가질 수도 있게 된다.

### 부동 소수점 타입

- 러스트는 `f64`를 기본 타입으로 규졍하고 있다.

```
// f64
let x = 2.0;

// f32
let y: f32 = 3.0;
```

- IEEE754 표준이며, `f32` 타입은 `single-precision` 부동 소수점이고, `f64`는 `double-precision` 부동 소수점을 표현한다.

### 사칙 연산

- 러스트는 기본적인 사칙 연산을 지원한다.

```
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
```

### 불리언 타입

- 불리언은 `true`, `false` 값 중 하나를 표현한다.

```
let t = true;
let f: bool = false;
```

### 문자 타입

- 러스트의 `char` 타입은 `4 byte` 크기의 유니코드 스칼라 값이다.

```
let c = 'z';
let z = 'Z';
let heart_eyed_cat = '😻';
```

## 컴파운드 타입

- 컴파운드 타입은 하나의 타입으로 여러 개의 값을 그룹화한 타입이다.

### 튜플 타입

- 튜플은 고정된 길이를 가지며, 한 번 정의하면 그 크기를 키우거나 줄일 수 없다.

```
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- 튜플에서 개별 값을 읽으려면 패턴 매칭을 이용해 튜플 값을 해체할 수 있다.

```
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;

println!("y : {}", y);
```

- 마침표 다음에 요소의 인덱스를 지정해서 요소를 직접 참조할 수도 있다.

```
let x: (i32, f64, u8) = (500, 6.4, 1);

let first = x.0;
let second = x.1;
let third = x.2;
```

### 배열 타입

- 데이터를 `stack` 메모리에 할당한다.

```
let months = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- 같은 값을 가진 배열을 선언할 때는 다음과 같이 선언하면 된다.

```
let b = [3; 5];
```

### 유효하지 않은 배열 요소에 접근하는 경우

- 컴파일 시점에는 아무런 에러가 발생하지 않지만, 프로그램을 실행하면 런타임 에러가 발생한다.

```
> cargo run
error: this operation will panic at runtime
  --> src/main.rs:79:19
   |
79 |     let element = a[999];
   |                   ^^^^^^ index out of bounds: the length is 5 but the index is 999
   |
   = note: `#[deny(unconditional_panic)]` on by default

warning: `variables` (bin "variables") generated 31 warnings
error: could not compile `variables` (bin "variables") due to previous error; 31 warnings emitted
```

## 함수

- 러스트 코드는 `snake case`를 사용한다.

```
> cargo run
   Compiling func v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap03/func)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/func`
Hello, world!
Another function!
```

### 매개변수

- 매개변수는 다음과 같이 지정할 수 있다.

```
fn parameter_function(x: i32, y: i32) {
    println!("x : {}, y : {}", x, y);
}
```

### 함수 본문의 구문과 표현식

- 함수 본문은 여러 개의 구문`(statements)`으로 구성되며, 선택적으로 표현식`(expression)`으로 끝나기도 한다.
- 표현식은 최종 결괏값으로 평가`(evaluate)`된다.
- `let` 선언문은 리턴 값이 없으므로, 다음의 코드는 오류가 발생한다.

```
let x = (let y = 6);
```

```
> cargo run
   Compiling func v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap03/func)
error: expected expression, found `let` statement
  --> src/main.rs:20:14
   |
20 |     let x = (let y = 6);
   |              ^^^

error: expected expression, found statement (`let`)
  --> src/main.rs:20:14
   |
20 |     let x = (let y = 6);
   |              ^^^^^^^^^
   |
   = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
  --> src/main.rs:20:14
   |
20 |     let x = (let y = 6);
   |              ^^^^^^^^^
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
```

- `let y = 6` 구문은 값을 리턴하지 않으므로 `x`에 아무런 값도 대입되지 않는다.
- 이 점이 값을 리턴하는 C나 Ruby 같은 언어와 다른 점이다.
- 이런 언어에서 `x = y = 6`과 같은 구문을 작성하면 `x`와 `y`의 변수 값이 모두 `6`이지만, 러스트에서 이런 결과는 얻을 수 없다.
- 매크로 호출도 표현식이다. 새로운 범위를 선언하기 위한 코드 블록 역시 표현식이다.

```
let x = 5;

let y = {
    let x = 3;
    x + 1
};
```

- 위의 코드는 `y`가 `4`로 평가된다.
- `y`의 코드 블록에 있는 `let x = 3;`은 코드 블록 안에서만 유효한다.
- 즉, `let x = 5;`를 통해 선언된 `x` 변수의 값이 수정되지 않는다.
- 아래는 실행 결과이다.

```
println!("x : {}, y : {}", x, y);
```

```
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/func`
x : 5, y : 4
```

### 값을 리턴하는 함수

- 리턴할 값의 타입은 화살표 다음에 지정해 주어야 한다.
- 함수 본문의 실행이 완전히 끝나지 않은 부분에서도 `return` 키워드와 함께 특정한 값을 리턴할 수 있다.

```
fn five() -> i32 {
    5
}
```

- 세미콜론을 추가하면, 이 표현식이 구문으로 바뀌어 에러가 발생한다.

## 주석

- 다음은 주석을 사용한 예다.

```
// Hi
```

## 흐름 제어
