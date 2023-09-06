# Chapter 01. 시작하기

<details>
<summary>Table of Contents</summary>

- [1-2 첫 번째 러스트 프로그램 작성하기](#1-2-첫-번째-러스트-프로그램-작성하기)
- [1-3 카고 알아보기](#1-3-카고-알아보기)

</details>

---

# 1-2 첫 번째 러스트 프로그램 작성하기

## 러스트 프로그램의 작성과 실행

- `main.rs`라는 이름으로 새로운 소스 파일을 생성하자.
- 러스트 파일은 모두 `.rs` 확장자를 갖는다.
- 아래 예시는 "test"를 출력하는 프로그램이다.

```
fn main() {
    println!("test");
}
```

- Linux나 macOS 환경에서는 다음 명령어를 이용해 파일을 컴파일하고 실행할 수 있다.

```
> rustc main.rs
> ./main
test
```

## 러스트 프로그램 자세히 살펴보기

```
fn main() {
}
```

- 위 코드는 러스트에서 함수를 정의하는 코드다.
- `main` 함수는 실행 가능한 모든 러스트 프로그램에서 가장 첫 번째로 실행된다.
- 첫 번째 줄은 매개 변수도 없고 리턴 값도 없는 `main`이라는 이름의 함수를 선언한다.
- 함수에 매개 변수가 필요하다면 괄호 안에 매개 변수를 나열하면 된다.

```
> rustc main.rs
```

- 컴파일 과정은 `gcc`나 `clang` 컴파일러와 유사하다.
- 컴파일이 성공적으로 완료되면 러스트는 실행 가능한 바이너리 파일을 생성한다.
- 윈도우 환경에서는, 위 코드를 실행하면 윈도우용 디버깅 정보를 가지고 있는 `.pdb` 확장자를 가진 파일까지 확인할 수 있다.
- 러스트는 미리(ahead-of-time) 컴파일하는 언어이다.

---

# 1-3 카고 알아보기

## 카고를 이용해 프로젝트 생성하기

- 운영체제와 관계없이 다음의 명령어를 실행하면 프로젝트를 생성할 수 있다.

```
> cargo new project
> cd project
```

- 디렉토리에 진입하면, 카고가 `main.rs` 파일이 보관된 `src` 디렉터리와 `Cargo.toml` 파일을 생성해준 것을 알 수 있다.
- 또한, `.gitignore` 파일과 함께 새로운 Git 저장소로 초기화된 것도 알 수 있다.
- `Cargo.toml` 파일을 열면, 아래와 같이 생성되어 있다.

```
[package]
name = "project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- 위 파일은 `TOML(Tom's Obvious, Minimal Language)` 형식으로 작성되어 있다. 이 형식은 카고의 설정 파일 형식이다.
- `[package]` 부분은 패키지의 설정을 관리하기 위한 구문들이 시작됨을 의미하는 섹션의 제목이다.
- `[dependencies]`는 프로젝트의 의존 라이브러리 목록을 관리하는 섹션이 시작하는 부분이다.
- 러스트에서 코드의 패키지는 `crate(크레이트)`라고 부른다.
- 이 프로젝트에서는 별도의 크레이트가 필요하지 않아 생략했다.
- 프로젝트 코드를 `src` 디렉터리로 옮긴 후 `Cargo.toml` 파일을 생성해주면, 카고를 이용하지 않고 프로젝트를 생성할 수 있다.

## 카고 프로젝트의 빌드 및 실행

- 프로젝트를 빌드하려면 프로젝트 디렉터리에서 다음 명령을 실행하면 된다.

```
> cargo build
   Compiling project1-3 v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap01/project1-3)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
```

- 이 명령을 실행하면 현재 디렉터리가 아니라 `target/debug/` 아래에 실행 파일이 생성된다.

```
> ./target/debug/project
test
```

- 아무 문제 없이 빌드와 실행이 완료되면 문자열이 출력된다.
- `cargo build` 명령을 처음 실행하면 카고는 최상위 디렉터리 `Cargo.lock`이라는 파일을 생성한다.
- 이 파일은 프로젝트에 필요한 의존 패키지의 정확한 버전을 추적하기 위한 파일이다.

```
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/project1-3`
test
```

- 카고를 이용하면 위와같이 한 줄의 명령으로 코드를 컴파일하고 결과 파일을 실행할 수 있다.
- 소스 코드를 수정하지 않고 실행하면 위처럼 컴파일을 수행하지 않는다.
- 만일, 소스 코드를 수정하면 카고는 바이너리를 실행하기 전에 프로젝트를 다시 빌드하고, 다음과 같은 결과가 나타난다.

```
> cargo run
   Compiling project1-3 v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap01/project1-3)
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/project1-3`
testtest
```

- 카고는 `cargo check`라는 명령어 또한 지원한다.
- 이 명령은 코드의 컴파일 여부를 신속하게 검사하지만 실행 파일은 생성하지 않는다.
- 실행 파일을 생성하는 과정을 생략하므로, `cargo build` 명령보다 훨씬 빠르게 실행된다.

```
> cargo check
    Checking project1-3 v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap01/project1-3)
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
```

## 릴리즈를 위한 빌드

- 프로젝트를 릴리즈할 준비가 되면 `cargo build --release`를 이용해서 최적화된 컴파일을 실행할 수 있다.
- 이 명령은 `target/debug/`가 아닌 `target/release/` 경로에 실행 파일을 생성한다.
- 더 빨리 자주 컴파일하기 위한 개발용 프로필과, 최정 완성된 프로그램을 사용자에게 제공하기 위해 최대한 빠르게 실행될 수 있도록 컴파일하기 위한 프로필이 별개로 분리된 이유는 바로 이 때문이다.
- 코드의 실행 시간을 벤치마킹 해보려면 `cargo build --release` 명령을 이용해 빌드한 후, `target/release/` 경로의 실행 파일을 이용해 벤치마킹을 실행할 수 있다.

