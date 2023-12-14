<!-- omit in toc -->

> # Chapter 05. 구조체를 활용한 관련 데이터의 구조화
>
> - 구조체는 서로 관련이 있는 여러 값을 하나로 모으고, 이름을 지정해 접근할 수 있는 타입이다.

<details>
<summary>Table of Contents</summary>

- [5-1 구조체 정의와 인스턴스 생성](#5-1-구조체-정의와-인스턴스-생성)
  - [같은 이름의 필드와 변수를 편리하게 활용하기](#같은-이름의-필드와-변수를-편리하게-활용하기)
  - [기존의 인스턴스로부터 새 인스턴스 생성하기](#기존의-인스턴스로부터-새-인스턴스-생성하기)
  - [이름 없는 필드를 가진 튜플 구조체로 다른 타입 생성하기](#이름-없는-필드를-가진-튜플-구조체로-다른-타입-생성하기)
  - [필드가 없는 유사 유닛 구조체](#필드가-없는-유사-유닛-구조체)
- [5-2 구조체를 사용하는 예제 프로그램](#5-2-구조체를-사용하는-예제-프로그램)
  - [튜플을 이용한 리팩토링](#튜플을-이용한-리팩토링)
  - [구조체를 이용한 리팩토링: 더 많은 의미 반영하기](#구조체를-이용한-리팩토링-더-많은-의미-반영하기)
  - [트레이트를 상속해서 유용한 기능 추가하기](#트레이트를-상속해서-유용한-기능-추가하기)
- [5.3 메서드 문법](#53-메서드-문법)
  - [메서드 정의하기](#메서드-정의하기)

</details>

---

# 5-1 구조체 정의와 인스턴스 생성

- 구조체를 정의하려면 `struct` 키워드 다음에 구조체에 부여할 이름을 지정해주면 된다.
- 구조체의 이름은 그룹화된 데이터를 잘 표현할 수 있어야 한다.
- 이 데이터들을 '필드(field)'라고 한다.
- 다음은 사용자 계정 정보를 저장하는 구조체를 정의한 코드이다.

```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
```

- 구조체를 정의한 후 이를 사용하려면 해당 구조체의 이름과 중괄호를 이용해 '키:값'의 쌍을 나열하면 된다.

```
let user1 = User {
    email: String::from("user@example.com"),
    uesrname: String::from("username"),
    active: true,
    sign_in_count: 1,
}
```

- 구조체에서 원하는 값을 읽으려면 마침표를 사용하면 된다.
- 이를 이용하여 필드의 값을 변경할 수도 있다.

```
user1.email = String::from("newemail@example.com");
```

- 아래는 지정한 메일과 사용자 이름으로 `User` 인스턴스를 생성해 리턴하는 `build_user` 함수의 코드이다.
- `active` 필드는 `true`로 초기화되며, `sign_in_count` 필드는 1로 초기화된다.

```
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

## 같은 이름의 필드와 변수를 편리하게 활용하기

- 필드 초기화 단축 문법을 이용하면 함수의 매개변수 이름과 구조체의 필드 이름이 같은 경우, 이를 생략할 수 있다.

```
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## 기존의 인스턴스로부터 새 인스턴스 생성하기

- 구조체 갱신 문법을 이용하면 편리하다.

```
let user2 = User {
    email: String::from("another@example.com"),
    ...user1
};
```

## 이름 없는 필드를 가진 튜플 구조체로 다른 타입 생성하기

- 튜플과 유사하게 생긴 구조체를 선언할 수도 있다.
- 이런 구조체를 튜플 구조체라고 한다.
- 튜플 구조체를 정의하려면 `struct` 키워드와 구조체의 이름, 그리고 튜플 안에서 사용할 타입들을 차례대로 나열하면 된다.

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## 필드가 없는 유사 유닛 구조체

- 러스트는 필드가 하나도 없는 구조체를 선언할 수도 있다.
- 이런 구조체를 유사 유닛 구조체라고 한다.

---

# 5-2 구조체를 사용하는 예제 프로그램

- 이 프로그램은 픽셀 단위와 너비와 높이를 이용해 사각형의 면적을 구한다.

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("Size: {}", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- `area` 함수는 한 사각형의 면적을 구하지만, 두 개의 매개변수를 선언하고 있다.
- 두 개의 매개변수는 서로 관련이 있지만, 관계를 표현하고 있지 않다.

## 튜플을 이용한 리팩토링

- 아래는 튜플을 이용해 다시 작성한 코드이다.

```
fn main() {
    let rect1 = (30, 50);

    println!("area: {}", area(rect1));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
```

## 구조체를 이용한 리팩토링: 더 많은 의미 반영하기

- 이 예제에서는 `Rectangle`이라는 이름의 구조체를 선언한다.
- `area` 함수는 이제 `rectangle`이라는 하나의 매개변수만을 사용하며, `Rectangle` 구조체의 불변 인스턴스에 대한 대여다.

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area: {}", area(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

## 트레이트를 상속해서 유용한 기능 추가하기

- 해당 문장을 추가하면 `println!` 등에서 출력이 가능하다.

```
#[derive(Debug)]
```

---

# 5.3 메서드 문법

- 메서드는 함수와 유사하다.
- 함수와 마찬가지로 `fn` 키워드를 이용해 정의하며 이름, 매개변수, 리턴 타입을 정의할 수 있다.

## 메서드 정의하기

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

- `Rectangle` 타입의 컨텍스트 안에 함수를 정의하려면 `impl` 블록을 이용하면 된다.
- 그런 다음 `area` 함수를 `impl` 중괄호 안으로 이동하고, 첫 번째 매개변수의 이름과 함수 본문 안에서의 참조를 모두 `self`로 바꾼다.
