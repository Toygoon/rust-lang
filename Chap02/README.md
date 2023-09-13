# Chapter 02. 숫자 맞히기 게임의 구현

- 이번 장에서는 실제로 동작하는 프로그램을 작성하면서, 러스트의 몇 가지 일반적인 개념을 파악해본다.
- 이 과정에서 `let`, `match`, 메서드, 연관 함수, `crates`의 사용법 등 다양한 개념을 학습한다.

<details>
<summary>Table of Contents</summary>

- [2-1 새 프로젝트 셋업하기](#2-1-새-프로젝트-셋업하기)
- [2-2 플레이어가 예측한 값 처리하기](#2-2-플레이어가-예측한-값-처리하기)
- [2-3 난수 생성하기](#2-3-난수-생성하기)

</details>

---

# 2-1 새 프로젝트 셋업하기

- 다음과 같이 카고를 이용해 새 프로젝트를 생성한다.

```
> cargo new project2-1
> cd project2-1
```

- 이 프로젝트에 생성된 `Cargo.toml` 파일의 내용은 다음과 같다.

```
[package]
name = "project2-1"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

# 2-2 플레이어가 예측한 값 처리하기

- 숫자 맞히기 게임을 구현하는 첫 번째 단계는 플레이어에게 입력할 값을 묻고 이 입력을 처리한 후, 그 값이 원하는 형태인지를 확인하는 것이다.
- 먼저, 플레이어에게 예측한 값을 묻는 코드를 작성해보자.
- `src/main.rs` 파일에 아래의 코드를 작성한다.

```
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞혀봅시다!");

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값 : {}", guess);
}
```

- 먼저, 플레이어가 입력한 값을 읽어와 출력하려면 `io` 라이브러리를 가져와야 한다.
- 러스트는 모든 프로그램의 프렐류드(Prelude)에서 기본적으로 단 몇 개의 타입을 가져온다.
- 만일 사용하고자 하는 타입이 프렐류드에 포함되지 않으면 `use` 구문을 이용해 명시적으로 프로그램의 범위로 가져와야 한다.
- `std::io` 라이브러리는 사용자가 입력한 값을 읽는 기능을 비롯해 여러 가지 유용한 기능을 제공한다.

## 변수에 값 지정하기

- 다음 코드는 사용자에게 입력받은 값을 저장할 공간을 만든다.

```
let mut guess = String::new();
```

- `let`은 변수를 생성하는 구문이다.
- 또 다른 예로 변수는 다음과 같이 선언한다.

```
let foo = bar;
```

- 러스트에서 변수는 기본적으로 값을 변경할 수 없다(immutable).
- 그러나, 다음 예제는 값을 변경할 수 있는 변수를 생성하기 위해 변수명 이전에 `mut` 키워드를 사용했다.

```
let foo = 5;        // 불변 변수
let mut bar = 5;    // 가변 변수
```

- 다시 프로그램으로 돌아가서, `guess` 변수에는 새로운 `String` 타입의 인스턴스를 생성하는 `String::new` 함수의 실행 결과를 바인딩했다.
- `String`은 표준 라이브러리가 제공하는 문자열 타입으로, 길이 조절이 가능하며 UTF-8 형식으로 인코딩된 텍스트를 표현한다.
- `::new` 줄에서 사용된 `::` 문법은 `new` 함수가 `String` 타입의 연관 함수(associated function)라는 점을 의미한다.
- 연관 함수는 특정한 인스턴스가 아니라 타입 자체에 구현된 함수이다.
- 다른 언어는 이런 메서드를 **정적 메서드(static method)**라고 부르기도 한다.
- 이 `new` 함수는 새로운 빈 문자열을 생성한다. 러스트를 사용하다 보면 많은 타입이 `new` 함수를 제공하는 것을 볼 수 있는데, 이는 어떤 타입의 새로운 값을 생성하는 함수에 일반적으로 부여하는 이름이기 때문이다.
- 이제 `io` 함수의 연관 함수인 `stdin` 함수를 호출하면 사용자의 입력 값을 읽을 수 있다.

```
io::stdin()
    .read_line(&mut guess)
    .expect("입력한 값을 읽지 못했습니다.");
```

- 만일 프로그램의 시작 부분에 `use std::io` 줄을 작성하지 않는다면 이 코드는 `std::io::stdin`과 같이 작성해도 된다.
- `stdin` 함수는 `std::io::Stdin` 타입의 인스턴스를 리턴한다.
- 표준 입력 핸들의 `read_line` 메서드를 호출해서 사용자가 입력한 값을 읽어온다.
- 이때 `&mut guess`라는 인수를 `read_line` 메서드에 전달한다.
- `read_line` 메서드의 역할은 사용자가 입력한 값을 표준 입력으로 읽어 문자열에 저장한다. 그래서 문자열 인스턴스를 인수로 전달해준다.
- 또한, `read_line` 메서드는 사용자가 입력한 값으로 문자열의 내용을 변경하기 때문에 **인수로 전달하는 문자열은 변경 가능한 문자열이어야 한다**.
- 인수에 사용한 `&` 기호는 이 인수가 **참조(reference)** 타입이라는 점을 지시한다.
- 즉, 프로그램의 다른 곳에서도 해당 데이터를 여러 번 메모리에 복사할 필요 없이 접근할 수 있다는 것을 의미한다.
- 변수와 마찬가지로 **참조 역시 기본적으로 변경할 수 없다는 점**을 알아두자.
- 그래서 변경 가능한 참조를 전달하기 위해 `&guess`가 아니라 `&mut guess`로 표기해야 한다.

## `Result` 타입을 이용해 잠재적인 오류 처리하기

```
.expect("입력한 값을 읽지 못했습니다.");
```

- 앞서 설명했듯이 `read_line` 메서드는 사용자가 입력한 값을 우리가 전달한 문자열에 대입하지만, `io::Result` 타입의 값을 리턴하기도 한다.
- 러스트는 표준 라이브러리 안에 범용의 `Result` 타입을 비롯해 `io::Result`와 같이 서브 모듈 전용의 `Result` 타입 등 여러 개의 `Result` 타입을 정의하고 있다.
- `Result` 타입은 **열거자(enumerations)**로, 줄여서 `enums`라고 표기하기도 한다.
- `Result` 열거자의 경우, 열것값은 `Ok`와 `Err`가 있다.
- `Result` 타입의 목적은 에러 처리를 위한 정보를 인코딩하기 위한 것이다.
- 다른 타입의 값과 마찬가지로 `Result` 타입의 값은 각자 자신에게 정의된 메서드를 포함하고 있다.
- 만일 `expect` 메서드를 호출하지 않는다면 프로그램이 컴파일은 되지만, 다음과 같은 경고 메시지가 나타난다.

```
warning: unused `Result` that must be used
 --> src/main.rs:8:5
  |
8 |     io::stdin().read_line(&mut guess);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
8 |     let _ = io::stdin().read_line(&mut guess);
  |     +++++++
```

- 러스트는 `read_line` 메서드가 리턴하는 `Result`를 사용하지 않으면 프로그램이 발생 가능한 에러를 제대로 처리하지 않는다는 경고 메시지를 출력한다.

## `println!` 자리지정자를 이용해 값 출력하기

```
println!("입력한 값 : {}", guess);
```

- 이 코드는 사용자의 입력을 저장한 문자열을 출력한다.
- 여기서 사용한 중괄호({})는 **자리 지정자(placeholder)**라고 부른다.

# 2-3 난수 생성하기

- 러스트의 표준 라이브러리는 난수를 추출하는 기능을 제공하지 않는다.
- 하지만 러스트 팀은 이런 기능을 위해 `rand` 크레이트를 제공한다.

## 크레이트를 이용해 필요한 기능 추가하기

- 크레이트는 소스 파일의 집합이라는 점을 기억하자.
- 작업 중인 프로젝트도 실행이 가능한 **바이너리 크레이트(binary crate)**다.
- `rand` 크레이트를 사용하는 코드를 작성하기에 앞서, 먼저 `Cargo.toml` 파일을 수정해서 `rand` 크레이트를 의존 패키지로 등록해 주어야 한다.
- `[dependencies]` 섹션 제목 아래에 다음의 코드를 추가하자.

```
[dependencies]
rand = "0.6.1"
```

- `Cargo.toml` 파일 내에서 헤더 다음에 작성하는 내용들은 다른 섹션의 헤더를 만나기 전까지는 해당 섹션에 속한다.
- `[dependencies]` 섹션에는 프로그램에 필요한 외부 크레이트의 종류와 버전을 카고에게 알려준다.
- 예제에서는 `rand` 크레이트와 함께 **시맨틱 버전(semantic version)** 식별자인 `0.6.1`을 지정했다.
- `0.6.1`이라는 버전은 사실 `^0.6.1`의 약식 표기이다.
- 이 표기는 **버전 `0.6.1`의 공개 API와 호환되는 모든 버전**을 의미한다.
- 아래는 `rand` 크레이트를 의존 패키지로 추가한 후 `cargo build` 명령의 실행 결과이다.

```
> cargo build
   Compiling autocfg v1.1.0
   Compiling rand_core v0.4.2
   Compiling libc v0.2.147
   Compiling rand_core v0.3.1
   Compiling autocfg v0.1.8
   Compiling rand_hc v0.1.0
   Compiling rand_xorshift v0.1.1
   Compiling rand_isaac v0.1.1
   Compiling rand_chacha v0.1.1
   Compiling rand_pcg v0.1.2
   Compiling rand v0.6.5
   Compiling rand_jitter v0.1.4
   Compiling rand_os v0.1.3
   Compiling project2-1 v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap02/project2-1)
    Finished dev [unoptimized + debuginfo] target(s) in 2.31s
```

- 프로그램에 필요한 외부 의존 패키지를 추가하면 카고는 패키지가 등록된 **저장소(registry)**인 `crates.io`로부터 가장 최신 버전의 복사본을 내려받는다.
- `crates.io`는 러스트 개발자들이 다른 개발자들을 위한 오픈 소스 러스트 프로젝트들을 등록하는 곳이다.
- 예제에서는 단 하나의 의존 패키지만을 사용하지만, `rand` 크레이트가 `libc` 크레이트에 의존하고 있기 때문에 `libc` 크레이트의 복사본도 다운로드한다.
- `src/main.rs'` 파일을 열어 간단히 파일을 변경한 후, 다시 저장하고 빌드하면 이번에는 두 줄의 출력 결과가 나온다.

```
> cargo build
   Compiling project2-1 v0.1.0 (/home/toygoon/workspace/rust-lang/Chap02/project2-1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
```

- 이 결과를 통해 알 수 있듯 카고는 `src/main.rs` 파일의 변경 사항만을 빌드한다.
- 프로젝트에 필요한 의존 패키지는 변경되지 않았으므로, 카도는 이미 다운로드한 패키지를 그대로 사용한다.

### `Cargo.lock` 파일을 이용해 재생산 가능한 빌드 구현하기

- `Cargo.lock` 파일은 최초로 `cargo build` 명령을 실행할 때 생성하며, 조건에 맞는 모든 의존 패키지를 `Cargo.lock` 파일에 기록한다.
- 그다음부터는 프로젝트를 빌드할 때마다 카고는 `Cargo.lock` 파일이 존재하는지 확인하고 필요한 버전을 다시 확인하는 것이 아니라 이 파일에 기록된 버전을 사용한다.
- 다른 누군가가 이 코드를 빌드하더라도 매번 같은 결과물을 재생산하는 매커니즘이다.

### 새 버전의 크레이트로 업그레이드하기

- 카고는 `update`라는 명령어로 크레이트를 업데이트할 수 있게 지원한다.
- 이 명령은 `Cargo.lock` 파일에 명시된 버전을 무시하고 `Cargo.toml` 파일에 지정된 조건에 해당하는 가장 최신 버전을 다시 찾는다.
- 이 명령이 성공적으로 실행되면, `Cargo.lock` 파일의 버전을 갱신한다.
- 만일 `rand` 크레이트가 `0.6.2`와 `0.7.0` 두 가지 버전을 출시했다면 `cargo update` 명령을 실행할 때 다음과 같은 결과가 나온다.

```
> cargo update
    Updating crates.io index
    Updating rand v0.6.1 -> v0.6.2
```

- 만일 `0.7.0`이나 `0.7.x`에 속하는 다른 버전을 사용하고 싶다면 `Cargo.toml` 파일을 다음과 같이 수정한다.

```
[dependencies]
rand = "0.7.0"
```

- 이후, `cargo build` 명령을 실행하면 다음과 같이 크레이트 저장소를 갱신하고 `rand` 크레이트의 새 버전이 요구하는 사항을 다시 평가한다.

```
> cargo update
    Updating crates.io index
    Removing autocfg v0.1.8
    Removing autocfg v1.1.0
    Removing bitflags v1.3.2
      Adding cfg-if v1.0.0
    Removing cloudabi v0.0.3
    Removing fuchsia-cprng v0.1.1
      Adding getrandom v0.1.16
      Adding ppv-lite86 v0.2.17
    Updating rand v0.6.5 -> v0.7.3
    Updating rand_chacha v0.1.1 -> v0.2.2
    Removing rand_core v0.3.1
    Removing rand_core v0.4.2
      Adding rand_core v0.5.1
    Updating rand_hc v0.1.0 -> v0.2.0
    Removing rand_isaac v0.1.1
    Removing rand_jitter v0.1.4
    Removing rand_os v0.1.3
    Removing rand_pcg v0.1.2
    Removing rand_xorshift v0.1.1
    Removing rdrand v0.4.0
      Adding wasi v0.9.0+wasi-snapshot-preview1
    Removing winapi v0.3.9
    Removing winapi-i686-pc-windows-gnu v0.4.0
    Removing winapi-x86_64-pc-windows-gnu v0.4.0
```

## 난수 생성하기

- 이제 `Cargo.toml` 파일에 `rand` 크레이트를 추가했으므로, 난수 생성 코드를 추가하겠다.

```
use rand::Rng

...

let secret_number = rand::thread_rng().gen_range(1, 101);
println!("사용자가 맞혀야 할 숫자 : {}", secret_number);

...
```

- 먼저, `use rand::Rng` 구문을 추가했다.
- `Rng` 트레이트(trait)는 난수 생성기에 구현된 메서드를 정의하며, 반드시 현재 범위(scope) 내에 선언되어야 한다.
- `rand::thread_rng` 함수는 난수 생성기를 리턴한다.
- 이 함수가 리턴하는 생성기는 현재 코드를 실행 중인 스레드 내에 존재하며 운영체제가 지정한 시드(seed) 값을 사용한다.
- 그런 다음, `gen_range` 메서드를 호출한다.
- `gen_range`의 파라미터는 `Python`의 `range()` 범위와 유사하게 작용한다.
- 불러온 크레이트의 어떤 함수나 메서드를 호출해야 하는지, 그리고 어떤 특성을 사용해야 하는지 곧바로 알 수는 없다.
- 여기서 다시 한 번 카고가 빛을 발하는데, `cargo doc --open` 명령은 모든 의존 패키지가 제공하는 문서를 로컬에서 빌드한 후 브라우저를 통해 보여준다. (??????????????????????????????????????????????????????????????????????????????)
- 아래는 해당 명령을 실행한 예시이다. (???????????????????????????????????????????????????????????????????????)

```
  Downloaded rand_core v0.5.1
  Downloaded getrandom v0.1.16
  Downloaded rand_chacha v0.2.2
  Downloaded rand v0.7.3
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.17
  Downloaded 6 crates (201.9 KB) in 0.84s
   Compiling libc v0.2.148
   Compiling getrandom v0.1.16
    Checking cfg-if v1.0.0
 Documenting cfg-if v1.0.0
    Checking ppv-lite86 v0.2.17
 Documenting ppv-lite86 v0.2.17
 Documenting libc v0.2.148
    Checking rand_core v0.5.1
    Checking rand_chacha v0.2.2
    Checking rand v0.7.3
 Documenting getrandom v0.1.16
 Documenting rand_core v0.5.1
 Documenting rand_chacha v0.2.2
 Documenting rand v0.7.3
 Documenting project2-1 v0.1.0 (/home/toygoon/workspace/rust-lang/Chap02/project2-1)
    Finished dev [unoptimized + debuginfo] target(s) in 2.65s
     Opening /home/toygoon/workspace/rust-lang/Chap02/project2-1/target/doc/project2_1/index.html
```

- ㅔ......................................................................

<img width="1267" alt="cargo_doc" src="https://github.com/Toygoon/rust-lang/assets/2356036/d4d216e5-00a1-4e98-a295-c95309b7e2ec">

- 지금까지 완성한 코드의 실행 예시는 아래와 같다.

```
> cargo run
   Compiling libc v0.2.148
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.1.16
   Compiling rand_core v0.5.1
   Compiling rand_chacha v0.2.2
   Compiling rand v0.7.3
   Compiling project2-1 v0.1.0 (/home/toygoon/workspace/rust-lang/Chap02/project2-1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.87s
     Running `target/debug/project2-1`
숫자를 맞혀봅시다!
사용자가 맞혀야 할 숫자 : 100
정답이라고 생각하는 숫자를 입력하세요.

입력한 값 :

> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/project2-1`
숫자를 맞혀봅시다!
사용자가 맞혀야 할 숫자 : 17
정답이라고 생각하는 숫자를 입력하세요.

입력한 값 :

```

# 2-4 난수와 사용자의 입력 비교하기

- 코드를 아래와 같이 추가하겠다.

```
use std::cmp::Ordering;

...

match guess.cmp(&secret_number) {
    Ordering::Less => println!("입력한 숫자가 작습니다!"),
    Ordering::Greater => println!("입력한 숫자가 큽니다!"),
    Ordering::Equal => println!("정답!"),
}
```

- 먼저, 새로운 `use` 구문을 살펴보자.
- 이 구문은 표준 라이브러리로부터 `std::cmp::Ordering` 타입을 로드한다.
- `Result` 타입과 마찬가지로 `Order` 역시 열거자이며 `Less`와 `Greater`, `Equal` 값 중 하나를 표현한다.
- 이 세 가지는 두 값을 비교해서 얻을 수 있는 세 가지 결과를 각각 의미한다.
- 그런 다음에는 `Ordering` 타입을 이용하는 다섯 줄의 코드를 프로그램의 마지막에 추가했다.
- `cmp` 메서드는 두 개의 값을 비교하며, 비교가 가능한 어떤 값에도 적용할 수 있다.
- `cmp` 메서드가 두 값을 비교한 결과인 `Ordering` 열것값에따라 적절하게 동작하고자 `match` 표현식을 사용했다.
- `match` 표현식은 여러 개의 가지(arm)로 구성된다.
- 각각의 가지는 pattern, 그리고 `match` 표현식의 시작 부분에 주어진 값이 이 패턴과 일치할 때 실행될 코드로 구성된다.
- 또한, 각각의 가지는 적합한 리턴값과 일치된 표현식을 찾은 경우 `match` 실행을 종료한다.

```
> cargo build
   Compiling project2-1 v0.1.0 (/home/toygoon/workspace/rust-lang/Chap02/project2-1)
error[E0308]: mismatched types
  --> src/main.rs:19:21
   |
19 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |                 |
   |                 arguments to this method are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: method defined here
  --> /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/cmp.rs:775:8

For more information about this error, try `rustc --explain E0308`.
error: could not compile `project2-1` (bin "project2-1") due to previous error
```

- 하지만, 코드는 실행되지 않고 위와 같은 에러를 출력한다.
- 러스트는 타입 추론(type inference) 기능을 지원한다.
- 변수 `guess`가 `String::new()` 코드를 작성해 `String` 타입이어야 한다는 점을 타입을 지정할 것을 강요하지 않는다.
- 타입 불일치를 해결하기 위해, `guess` 변수에 타입 지정을 아래와 같이 할 수 있다.
- 부호 있는 숫자 타입은 `i32`, `i64`가 있고, 부호 없는 숫자 타입은 `u32`, `u64` 등이 있다.

```
let guess: u32 = guess
    .trim()
    .parse()
    .expect("입력한 값이 올바른 숫자가 아닙니다.");
```

- 러스트는 위와 같이 `guess` 변수가 보관하던 값을 새로운 변수의 값으로 가려버린다.
