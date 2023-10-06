# Chapter 03. 일반 프로그래밍 개념

- 이번 장에서는 거의 모든 프로그래밍 언어에서 찾아볼 수 있는 개념들을 러스트는 어떻게 표현하는지 중점적으로 살펴본다.
- 이번 장에서는 변수, 기본 타입, 함수, 주석, 흐름 제어 등에 대해 알아본다.

<details>
<summary>Table of Contents</summary>

- [3-1 변수와 가변성](#3-1-변수와-가변성)

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
- 
