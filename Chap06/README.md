<!-- omit in toc -->

> # Chapter 06. 열거자와 패턴 매칭
>
> - 열거자는 사용 가능한 값만 나열한 타입을 정의할 때 사용한다.
> - 러스트에서 특히 유용하게 사용하는 `Option` 열거자에 대해 알아본다.
> - `Option` 열거자는 어떤 값을 갖거나 전혀 갖지 않는 타입이다.
> - `match` 표현식을 이용한 패턴 매칭에 관해 설명한다.
> - 마지막으로는 `if let` 구문을 이용해 열거자를 더욱 편리하고 간단하게 다루는 방법에 대해 알아본다.

<details>
<summary>Table of Contents</summary>

- [6-1 열거자 정의하기](#6-1-열거자-정의하기)
  - [열거자의 값](#열거자의-값)
  - [`Option` 열거자를 `Null` 값 대신 사용할 때의 장점](#option-열거자를-null-값-대신-사용할-때의-장점)

</details>

---

# 6-1 열거자 정의하기

- 예를 들어, IP 주소를 다루는 상황을 가정해보자.
- 모든 IP 주소는 버전 4나 버전 6 형식의 주소지만 동시에 두 형식을 지원할 수는 없다.
- 이런 상황에서는 IP 주소의 형식을 나타내는 `IpAddrKind` 열거자를 정의하고 각 형식을 표현하기 위한 V4와 V6 값을 정의하면 된다.
- 이 값들을 열거자의 `열것값(variants)`이라고 한다.

```
enum IpAddrKind {
    V4,
    V6
}
```

## 열거자의 값

- `IpAddrKind` 열거자의 각 값을 표현하는 인스턴스는 다음과 같이 생성한다.

```
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- 열거자를 매개변수로 갖는 함수를 정의할 수도 있다.

```
fn route(ip_type: IpAddrKind) { }
```

- 그러면 이 함수는 열거자의 값을 이용해 다음과 같이 호출할 수 있다.

```
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

- 열거자를 사용하면 여러 가지 장점을 얻을 수 있다.
- 아래 예제는 열거자의 값과 관련 값을 하나로 처리할 수 있는 예제이다.

```
enum IpAddrKind {
  V4,
  V6,
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1"),
};
```

- 아래는 `V4` 형식의 주소에 네 개의 `u8` 값을 저장하는 구조체 예시이다.

```
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

- 아래는 개별 값을 각기 다른 타입으로 정의한 `Message` 열거자이다.

```
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
```

- `impl` 블록을 이용해 구조체에 메서드를 정의할 수 있는 것과 마찬가지로 열거자에도 메서드를 정의할 수 있다.

```
impl Message {
  fn call(&self) {
    // 여기에 메서드 본문을 작성한다.
  }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## `Option` 열거자를 `Null` 값 대신 사용할 때의 장점

- `Option` 타입은 매우 다양한 곳에서 활용할 수 있다.
- 열거자가 어떤 값이 존재하거나 존재하지 않는, 아주 범용적인 시나리오에 적합하도록 디자인되었기 때문이다.
- `Option<T>` 열거자는 매우 유용하며 심지어 프렐류드에 포함되어 있다.
- `Option::` 접두어 없이도 `Some`이나 `None` 값을 직접 사용할 수 있다.
- 다음 예제는 숫자 타입과 문자열 타입을 저장하는 `Option` 열거자를 사용하는 방법을 보여준다.

```
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

- 단, 아래의 예제처럼 `i8` 타입의 값에 `Option<i8>` 타입의 값을 더하는 다음의 코드는 컴파일되지 않는다.

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

- 이는 이 둘이 서로 다른 타입이기 때문이다.
- `i8` 타입의 값을 가지고 있다면 컴파일러는 항상 이 값이 유효한 값이라고 가정한다.
- 따라서 이 값을 사용하기 전에 널 검사 같은 것을 할 필요가 없다.
- 컴파일러는 `Option<i8>` 타입을 사용할 때만 이 변수에 값을 얻을 가능성이 있기 때문에 값을 사용하기에 앞서 값이 없는 경우를 처리하려고 한다.
- 다시 말하면, `T` 타입에 대한 작업을 실행하기 전에 `Option<T>` 타입을 `T` 타입을 변환해야 한다.
- 통상 `Option<T>` 값을 사용하려면 열거자에 나열된 개별 값들을 처리할 코드를 작성해야 한다.
- `match` 표현식은 이런 코드를 쉽게 작성할 수 있는 '흐름 제어 연산자'이다.
