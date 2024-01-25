<!-- omit in toc -->

> # Chapter 07. 패키지, 크레이트, 모듈로 프로젝트 관리하기
>
> - _패키지_: 크레이트를 빌드, 테스트, 공유할 수 있는 카고의 기능
> - _크레이트_: 라이브러리나 실행 파일을 생성하는 모듈의 트리(tree)
> - _모듈과 use_: 코드의 구조와 범위, 그리고 경로의 접근성을 제어하는 기능
> - _경로(path)_: 구조체, 함수, 혹은 모듈 등의 이름을 결정하는 방식

<details>
<summary>Table of Contents</summary>

- [7-1 패키지와 크레이트](#7-1-패키지와-크레이트)
- [7-2 모듈을 이용한 범위와 접근성 제어](#7-2-모듈을-이용한-범위와-접근성-제어)
- [7-3 경로를 이용해 모듈 트리의 아이템 참조하기](#7-3-경로를-이용해-모듈-트리의-아이템-참조하기)
  - [`pub` 키워드로 경로 공개하기](#pub-키워드로-경로-공개하기)
  - [`super`로 시작하는 상대 경로](#super로-시작하는-상대-경로)
  - [구조체와 열거자 공개하기](#구조체와-열거자-공개하기)
- [7-4 `use` 키워드로 경로를 범위로 가져오기](#7-4-use-키워드로-경로를-범위로-가져오기)
  - [관용적인 경로 사용하기](#관용적인-경로-사용하기)
  - [`as` 키워드로 새로운 이름 부여하기](#as-키워드로-새로운-이름-부여하기)
  - [`pub use` 키워드로 이름을 다시 내보내기](#pub-use-키워드로-이름을-다시-내보내기)
  - [외부 패키지의 사용](#외부-패키지의-사용)
  - [중첩 경로로 `use` 목록을 깔끔하게 유지하기](#중첩-경로로-use-목록을-깔끔하게-유지하기)
  - [글롭 연산자](#글롭-연산자)
- [7-5 모듈을 다른 파일로 분리하기](#7-5-모듈을-다른-파일로-분리하기)

</details>

---

# 7-1 패키지와 크레이트

- 크레이트 루트는 러스트 컴파일러가 컴파일을 시작해서 크레이트의 루트 모듈을 만들어 내는 소스 파일이다.
- `cargo new` 명령을 실행하면, 카고는 `Cargo.toml` 파일을 생성해 패키지를 만들어 낸다.
- `Cargo.toml` 파일을 보면 `src/main.rs` 파일에 대한 언급이 없는 것을 알 수 있다.
- 마찬가지로 카고는 패키지 디렉터리에 `src/lib.rs` 파일이 있으면 이 패키지는 패키지와 같은 이름으로 라이브러리 크레이트를 포함한다고 판단하며, `src/lib.rs` 파일을 크레이트 루트로 인식한다.
- 카고는 라이브러리나 바이너리를 빌드할 때 `rustc` 컴파일러에게 크레이트 루트 파일을 전달한다.

---

# 7-2 모듈을 이용한 범위와 접근성 제어

- 경로는 아이템의 이름을 결정하며, `use` 키워드는 이 경로를 범위 안으로 가져온다.
- `pub` 키워드는 아이템을 외부에 공개한다.
- 또한, `as` 키워드, 외부 패키지, 그리고 `glob` 연산자에 관해서도 설명한다.
- `모듈(module)`은 크레이트의 코드를 그룹화해서 가독성과 재사용성을 향상하는 방법이다.
- 아이템을 외부에 공개할 수 있는지(public), 아니면 외부의 코드가 사용할 수 없는 상세 구현(private)인지를 결정하기도 한다.
- 아이템의 공개 및 비공개 여부를 아이템의 접근성(privacy)라고 한다.
- 레스토랑의 기능을 제공하는 간단한 바이너리 크레이트를 예제로 작성해보자.

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

- 예제 코드를 보면 먼저 `mod` 키워드를 이용해 모듈을 정의한 후 모듈의 이름을 지정한다.
- 모듈의 본문은 중괄호로 감싼다.
- 모듈 안에는 다른 모듈을 정의할 수 있으며 예제에서는 `hosting`과 `serving` 모듈을 정의했다.
- 모듈에는 구조체, 열거자, 상수, 트레이트는 물론 함수를 추가할 수 있다.
- 모듈을 이용하면 관련된 정의들을 하나의 그룹으로 묶어 적절한 이름을 부여할 수 있다.
- 전체 정의를 훑어보지 않아도 그룹 단위로 코드를 살펴보고 자신에게 필요한 정의를 쉽게 찾을 수 있다.
- `src/main.rs`와 `src/lib.rs` 파일을 크레이트 루트라고 부르는데, 그 이유는 두 파일의 콘텐츠는 `crate`라는 이름의 모듈로 구성되며, 이 모듈은 모듈 트리(module tree)라고 부르는 모듈 구조에서 루트 역할을 하기 때문이다.

---

# 7-3 경로를 이용해 모듈 트리의 아이템 참조하기

- 경로는 크게 두 가지 형태이다.
- _절대 경로(absolute path)_: 크레이트 이름이나 `crate` 리터럴을 이용해 크레이트 루트부터 시작하는 경로이다.
- _상대 경로(relative path)_: 현재 모듈로부터 시작하며 `self`, `super` 혹은 현재 모듈의 식별자를 사용한다.
- 아래 예제는 크레이트 루트에 정의한 `eat_at_restaurant`라는 새 함수에서 `add_to_waitlist` 함수를 호출하는 두 가지 방법을 보여준다.

```
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hostring::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

- 이러한 예제를 추가하면, 아래와 같은 오류가 발생한다.

```
   Compiling restaurant v0.1.0 (/Users/toygoon/workspace/rust-lang/Chap07/restaurant)
error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:31:37
   |
31 |     crate::front_of_house::hosting::add_to_waitlist();
   |                                     ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:18:9
   |
18 |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:34:30
   |
34 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:18:9
   |
18 |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors
```

- 러스트에서 접근성이 동작하는 방식은 모든 아이템은 기본적으로 비공개다.
- 부모 모듈의 아이템들은 자식 모듈 안의 비공개 아이템을 사용할 수 없지만, 자식 모듈의 아이템은 부모 모듈의 아이템을 사용할 수 있다.

## `pub` 키워드로 경로 공개하기

- 앞선 예제에 따르면 `hosting` 모듈은 비공개이다.
- 이를 해결하기 위해, `hosting` 모듈에 `pub` 키워드를 추가해야 한다.

```
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    ...
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hostring::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

```

- 하지만, 위의 예제를 컴파일하면 아래와 같은 에러 메시지를 발생한다.

```
   Compiling restaurant v0.1.0 (C:\Users\Toygoon\workspace\rust-lang\Chap07\restaurant)
error[E0433]: failed to resolve: could not find `hostring` in `front_of_house`
  --> src\lib.rs:31:28
   |
31 |     crate::front_of_house::hostring::add_to_waitlist();
   |                            ^^^^^^^^ could not find `hostring` in `front_of_house`

error[E0603]: function `add_to_waitlist` is private
  --> src\lib.rs:34:30
   |
34 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src\lib.rs:18:9
   |
18 |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0433, E0603.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `restaurant` (lib) due to 2 previous errors
```

- 위의 에러 메시지를 참조하면, `add_to_waitlist` 함수가 여전히 비공개이다.
- 아래의 예제처럼 `add_to_waitlist` 함수에 `pub` 키워드를 추가해서 함수를 공개해보자.

```
...
pub fn add_to_waitlist() {}
...
```

- 먼저, 절대 경로는 모듈 트리의 루트인 `crate` 부터 시작한다.
- 그 다음 `front_of_house` 모듈은 크레이트 루트 안에 정의되었다.
- 이는 같은 모듈에 정의된 형제 관계이므로, `eat_at_restaurant` 함수가 `front_of_house` 모듈을 참조할 수 있다.
- `add_to_waitlist` 함수 또한 `pub` 키워드로 인해 공개 함수가 되었으므로, 부모 모듈이 접근할 수 있어 함수 호출이 이루어진다.
- 상대 경로를 살펴보면, 맨 처음의 모듈 이름을 제외한 나머지 경로에는 같은 규칙이 적용된다.
- 상대 경로는 크레이트 루트부터 시작하는 것이 아니라, `front_of_house` 모듈부터 시작한다.
- `eat_at_restaurant` 함수가 정의된 모듈부터 시작하는 상대 경로 또한 동작한다.
- `hosting` 모듈과 `add_to_waitlist` 함수는 `pub` 키워드로 인해 공개되었으므로 이 함수의 호출이 유효하게 된다.

## `super`로 시작하는 상대 경로

- 상대 경로는 `super` 키워드를 이용해 부모 모듈부터 시작할 수도 있다.
- `super` 키워드를 이용하면 나중에 코드를 다른 모듈로 이동해도 수정해야 할 코드를 최소화 할 수 있다.

## 구조체와 열거자 공개하기

- `pub` 키워드는 구조체와 열거자를 공개할 때도 사용할 수 있지만, 만일 구조체를 정의할 때 `pub` 키워드를 사용한다면 구조체가 공개되는 반면 구조체의 필드는 여전히 비공개 상태이다.
- 이때는 필요에 따라 각 필드를 공개하거나 비공개로 유지하면 된다.
- 아래 예시는 구조체를 공개하고, 몇 개의 필드는 공개와 비공개 상태가 같이 존재하는 상태이다.

```
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peach"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // member field `summer` is mutable
    let mut meal = back_of_house::Breakfast::summer("black bread");
    meal.toast = String::from("white bread");
    println!("I'll order {} toast.", meal.toast);

    // but another member field `seasonal_fruit` is not mutable
    // meal.seasonal_fruit = String::from("blueberry");
}
```

- `seasonal_fruit` 필드는 비공개이므로 `eat_at_restaurant` 함수가 접근할 수 없다.
- 또한, 구조체는 비공개 필드를 가지므로 구조체의 인스턴스가 생성할 수 있는 공개용 연관 함수를 제공해야 한다.
- 만일, 구조체에 연관 함수가 없다면 `eat_at_restaurant` 함수는 비공개인 필드를 설정할 수 없으므로 해당 구조체의 인스턴스를 생성할 수 없다.
- 즉, 구조체에 비공개 필드는 생성자를 통해 초기화되어야 하지만, 생성자가 없다면 이를 초기화할 방법이 없으므로 생성자가 없는 경우 인스턴스가 생성되지 않는다는 의미이다.
- 반면, 열거자를 공개하면 모든 열것값 또한 공개된다.
- 아래 예제처럼 `enum` 키워드 앞에 `pub` 키워드만 추가하면 된다.

```
pub enum Appetizer {
    Soup,
    Salad,
}
```

- `Appetizier` 열거자를 공개했으므로 `eat_at_restaurant` 함수는 `Soup`과 `Salad` 열것값을 모두 사용할 수 있다.

---

# 7-4 `use` 키워드로 경로를 범위로 가져오기

- `use` 키워드와 경로를 추가하는 것은 파일 시스템에서 심볼릭 링크를 생성하는 것과 유사하다.
- 아래의 `use` 구문을 크레이트 루트에 추가하면, 마치 `hosting` 모듈을 크레이트 루트에 정의한 것처럼 그 범위에서 유효한 이름이 된다.

```
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- 또한, `use` 키워드에 상대 경로를 지정하는 것은 조금 다르다.
- 이때는 현재 범위의 이름부터 시작하는 대신 `self` 키워드를 이용한 경로를 사용해야 한다.
- 아래는 해당 예시이다.

```
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## 관용적인 경로 사용하기

- 구조체, 열거자, 혹은 기타 다른 아이템을 `use` 구문으로 가져올 때는 전체 경로를 사용하는 것이 관용적이다.
- 아래 예제는 `HashMap` 구조체를 바이너리 크레이트의 범위로 가져오는 방법을 보여준다.

```
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- 아래는 이름은 같지만 다른 부모 모듈에 정의된 두 `Result` 타입을 가져와 사용하는 코드이다.

```
use std::fmt;
use std::io;

fn func1() -> fmt::Result {}

fn func2() -> io::Result<()> {}
```

## `as` 키워드로 새로운 이름 부여하기

- 경로 뒤에 `as` 키워드를 이용해 해당 타입에 새로운 이름을 부여하면 된다.

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn func1() -> fmt::Result {}

fn func2() -> IoResult<()> {}
```

## `pub use` 키워드로 이름을 다시 내보내기

- `use` 키워드를 이용해 범위로 이름을 가져오면 이 이름은 새 범위에서 비공개 이름이 된다.

```
pub use crate::front_of_house::hosting;
```

## 외부 패키지의 사용

- 제2장에서 난수 프로젝트를 진행할 때 난수를 생성하는 `rand`라는 외부 패키지를 사용했었다.
- `Cargo.toml` 파일에 `rand`를 의존성으로 추가하면 카도는 이 패키지를 레포지토리에서 내려받는다.
- 표준 라이브러리(std) 또한 외부 크레이트이다.

## 중첩 경로로 `use` 목록을 깔끔하게 유지하기

- 한 줄에 하나의 아이템을 나열하면 공간을 너무 많이 차지하게 된다.

```
use std::io;
use std::cmp::Ordering;
```

- 중첩된 경로를 이용해 같은 아이템을 한 줄의 코드로 가져올 수 있다.

```
use std::{io, cmp::Ordering};
```

- 또한, 일부 경로를 공유하는 두 개의 `use` 구문은 아래와 같다.

```
use std::io;
use std::io::Write;
```

- 이를 하나로 합치려면 `self` 키워드를 사용하면 된다.

```
use std::io::{self, Write};
```

## 글롭 연산자

- 어떤 경로의 공개 아이템을 모두 현재 범위로 가져오려면 글롭 연산자인 `*`를 사용해 경로를 지정하면 된다.

```
use std::collections::*;
```

---

# 7-5 모듈을 다른 파일로 분리하기

- 앞의 예제는 다음과 같이 모듈 분리가 가능하다.
- 아래는 `front_of_house` 모듈이 `src/lib.rs`에서 사용되는 예시이다.

```
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- `src/front_of_house.rs` 파일에는 아래 예제처럼 `front_of_house` 모듈의 본문을 옮긴다.

```
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

- 마찬가지로 `hosting` 모듈의 콘텐츠를 다른 파일로 옮기면 `src/front_of_house` 파일에는 `hosting` 모듈의 선언 부분만 남게 된다.

```
pub mod hosting;
```

- 이제 `src/front_of_house` 디렉터리를 생성한 후 `src/front_of_house/hosting.rs` 파일에 `hosting` 모듈의 정의를 옮겨오자.

```
pub fn add_to_waitlist() {}
```

- 이렇게 해도 모듈 트리는 같은 형태로 유지되며, `eat_at_restaurant` 함수의 호출은 아무런 문제 없이 실행된다.