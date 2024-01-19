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
