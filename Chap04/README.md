# Chapter 04. 소유권

- 소유권은 러스트의 독특한 기능 중 하나로, `Garbage Collector`에 의존하지 않고도 메모리 안정성을 보장하려는 러스트만의 해법이다.
- 이번 장에서는 소유권과 더불이 `borrowing`, `slice`, 메모리 관리 방법을 살펴본다.

<details>
<summary>Table of Contents</summary>

- [4-1 소유권이란?](#4-1-소유권이란?)
- [4-2 참조와 대여](#4-2-참조와-대여)

</details>

---

# 4-1 소유권이란?

- 소유권은 러스트의 핵심 기능이다.
- Java는 가비지 콜렉터를 사용해 사용하지 않는 메모리를 지속적으로 찾아 자동으로 해제한다.
- C 계열 언어는 프로그래머가 명시적으로 메모리를 할당하고 해제해 주어야 한다.
- 러스트는 제3의 방법을 사용한다.
- 메모리는 컴파일러가 컴파일 시점에 검사하는 다양한 규칙으로 이루어진 소유권 시스템에 의해 관리된다.
- 따라서 소유권과 관련된 기능은 프로그램의 실행 성능에 아무런 영향을 미치지 않는다.

### 스택 메모리와 힙 메모리

- 러스트 같은 시스템 프로그래밍 언어 환경에서는 값이 스택 메모리에 저장되었는지 힙 메모리에 저장되었는지에 따라 언어의 동작이나 의사결정에 큰 영향을 미친다.
- 스택에 데이터를 푸시하는 것이 힙에 할당하는 것보다 빠른 이유는 OS가 새 데이터를 저장할 공간을 찾을 필요가 없기 때문이다.
- 새 데이터는 항상 스택의 가장 위에 추가된다.
- 힙에 공간을 할당하는 것은 상대적으로 더 많은 작업이 필요하다.
- OS가 데이터를 저장할 충분히 큰 공간을 찾은 후 다음 할당 작업을 위한 예약 작업을 수행해야 하는 이유다.
- 코드에서 함수를 호출할 때 함수에는 여러 값이 전달되며, 이 값들은 함수의 로컬 변수에 할당되어 스택에 저장된다.
- 함수의 실행이 완료되면 이 값들은 스택에서 제거된다.

## 소유권 규칙

- 러스트가 다루는 각각의 값은 소유자(owner)라고 부르는 변수를 가지고 있다.
- 특정 시점에 값의 소유자는 단 하나뿐이다.
- 소유자가 범위를 벗어나면 그 값은 제거된다.

## 변수의 범위

- 변수의 범위(scope)에 대해 확인해보자.
- 다음과 같이 변수를 하나 선언했다고 생각해보자.

```
let s = "hello";
```

- 변수 `s`는 프로그램 안에 하드코딩한 문자열값인 문자열 리터럴을 참조한다.
- 변수 `s`는 선언된 지점부터 현재 범위를 벗어나기 전까지 유효하다.
- 변수는 범위 안으로 들어오면 유효하고, 범위를 벗어나기 전까지 유효하다.

## String 타입

- 문자열 리터럴은 텍스트를 다루어야 하는 모든 경우에 적잡한 방법은 아니다.
- 그 이유 중 하나는 문자열이 `immutable` 특성을 가지기 때문이다.
- 러스트는 또 다른 타입인 `String` 타입을 제공한다.
- 이 타입은 힙에 할당되므로 컴파일 시점에 알 수 없는 크기의 문자열을 저장할 수 있다.
- 다음과 같이 `from` 함수를 이용하면 문자열 리터럴을 이용해 `String` 인스턴스를 생성할 수 있다.

```
let s = String::from("hello");
```

- 이렇게 생성한 문자열은 변경이 가능하다.

```
let mut s = String::from("hello");
s.push_str(", world");

println!("{}", s);
```

## 메모리와 할당

- String literal은 컴파일 시점에 문자열의 내용을 이미 알고 있으므로 텍스트를 최종 실행할 수 있는 형태로 직접 하드코딩 할 수 있다.
- 그러므로 문자열 리터럴은 빠르고 효율적이다.
- 하지만 이런 장점은 문자열 리터럴이 불변이라는 사실에서 비롯된다.
- 안타깝게도 컴파일 시점에 그 길이를 미리 알 수 없거나 프로그램의 실행 중에 길이가 변경되는 문자열은 그 문자열이 사용할 메모리를 바이너리 형태로 미리 변환할 수가 없다.
- 가변 문자열을 지원하는 `String` 타입은 길이를 조절할 수 있는 텍스트이므로 컴파일 시점에 알 수 없는 내용을 저장하기 위해 힙 메모리에 일정 부분의 메모리를 할당해야 한다.
- 따라서 두 가지 절차를 거친다.

> - 해당 메모리는 반드시 런타임 시점에 운영체제에 요청해야 한다.
> - String 타입의 사용이 완료된 후에는 이 메모리를 운영체제에 다시 돌려줄 방법이 필요하다. (free)

- 첫 번째 절차는 개발자가 처리해야 한다.
- 두 번째 절차는 `GC`가 있는 언어의 경우 개발자가 이를 직접 처리할 필요는 없지만, 그렇지 않은 언어는 메모리를 돌려주는 작업도 개발자가 직접 처리해야 한다.
- 러스트는 메모리의 할당과 해제를 다른 방식으로 수행한다.
- 변수에 할당된 메모리는 변수를 소유한 범위(scope)를 벗어나는 순간 자동으로 해제한다.
- 위의 코드를 통해 범위에 대해 다시 한 번 확인해보자.

```
{
    let s = String::from("hello"); // 변수 s는 이 시점부터 유효하다.

    // 변수 s를 이용해 필요한 동작을 수행한다.
}   // 여기서 범위를 벗어나게 되므로 변수 s는 이제 유효하지 않다.
```

- 이 예제를 보면 변수 s가 사용하는 메모리를 운영체제로 돌려주기에 매우 적합한 지점이 있다.
- 변수가 범위를 벗어나면 러스트는 `drop`이라는 이름의 특별한 함수를 호출한다.
- `drop` 함수는 `String` 타입을 구현한 개발자가 메모리를 해제하는 코드를 작성해 둔 함수다. [(`ManuallyDrop`을 이용한 예시)](https://doc.rust-lang.org/std/string/struct.String.html#representation), [(`ManuallyDrop` 내부 소스 코드)](https://doc.rust-lang.org/stable/src/core/mem/manually_drop.rs.html#50)

### 변수와 데이터가 상호작용하는 방식 : 이동 (Move)

- `String` 타입을 사용하는 코드를 살펴보자.

```
let s1 = String:from("hello");
let s2 = s1;
```

- 두 번째 줄의 코드는 변수 `s1` 값의 복사본을 만들어 변수 `s2` 에 대입할 것으로 예상되지만, 실제로 그렇게 동작하지 않는다.
- `String` 타입은 문자열 콘텐츠를 저장하고 있는 메모리에 대한 포인터, 길이, 용량 등 세 부분으로 구성된다.

  > A String is made up of three components: a pointer to some bytes, a length, and a capacity. The pointer points to an internal buffer String uses to store its data. The length is the number of bytes currently stored in the buffer, and the capacity is the size of the buffer in bytes. As such, the length will always be less than or equal to the capacity.
  >
  > Documentation에서 요소를 직접 확인해볼 수 있다. [(#)](https://doc.rust-lang.org/std/string/struct.String.html#representation)

- 변수 `s1`에 `s2`에 대입하면 `String` 타입의 데이터가 복사된다.
- 즉, 포인터가 가리키는 힙 메모리의 실제 데이터가 아니라 문자열에 대한 포인터와 길이, 용량이 스택에 복사된다.
- 실제로 문자열의 복사는 이루어지지 않는다.
- 문자열이 같이 복사된다면 저장된 데이터가 큰 경우 런타임 성능이 크게 떨어질 것이다.
- 하지만 실제로는 포인터를 복사하는데, 변수 `s2`와 `s1`이 범위를 벗어나면 두 변수가 모두 같은 메모리를 해제하려고 한다.
- 이 문제는 '이중 해제 에러(double free error)'라고 하며, 메모리 안전성 버그 중 하나다.
- 메모리를 두 번 해제하는 것은 메모리의 불순화(corruption)를 일으킨다.
- 러스트는 할당된 메모리를 복사하는 대신 변수 `s1`이 더 이상 유효하지 않다고 판단하기 때문에 변수 `s1`이 범위를 벗어날 때 메모리를 해제할 필요가 없다.
- 러스트는 첫 번째 변수를 무효화해버리므로 이 동작은 얕은 복사라고 하지 않고 이동(move)이라고 한다.
- 이 예제의 경우, 변수 `s1`이 변수 `s2`로 '이동했다'고 표현한다.
- 러스트는 자동으로 데이터에 대해 '깊은' 복사를 수행하지 않는다.
- 그래서 런타임 성능 관점에서 볼 때, 모든 자동 복사 작업은 매우 가벼운 작업이다.

### 변수와 데이터가 상호작용하는 방식 : 복제 (Clone)

- 만일 스택 데이터가 아니라 힙 메모리에 저장된 `String` 데이터를 복사하기를 원한다면 `clone` 공통 메서드를 사용하면 된다.
- `clone` 메서드는 다음과 같이 사용할 수 있다.

```
let s1 = String::from("hello");
let s2 = s1.clone();
```

### 스택 전용 데이터 : 복사 (Copy)

- 아래는 정상적으로 동작하는 코드이다.

```
let x = 5;
let y = x;
```

- 정수형 같은 타입은 컴파일 시점에 이미 그 크기를 알 수 있으며, 온전히 스택에 저장되기 때문에 실제 값을 복사해도 전혀 부담되지 않는다.
- 온전히 스택에 저장되기 때문에 실제 값을 복사해도 전혀 부담되지 않는다.
- 러스트는 스택에 저장되는 정수형 같은 타입에 적용할 수 있는 `Copy` 트레이트라는 특별한 특성을 제공한다.
- 만일 어떤 타입에 `Copy` 트레이트가 적용되어 있다면 이전 변수를 새 변수에 할당해도 무효화되지 않는다.
- `Drop` 트레이트가 적용되어 있으면 `Copy` 트레이트를 적용할 수 없다.
- 다음은 `Copy` 트레이트가 적용된 타입의 일부를 나열한 것이다.

> - `u32`와 같은 모든 정수형 타입
> - `true`와 `false` 값만을 가지는 불리언 타입, `bool`
> - 문자 타입, `char`
> - `f64`와 같은 모든 부동 소수점 타입
> - `Copy` 트레이트가 적용된 타입을 포함하는 튜플

## 소유권과 함수

```
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
```

- 만일, `takes_ownership` 함수를 호출한 다음에 변수 `s`를 사용하려고 하면 러스트는 컴파일 시점에 에러를 리턴한다.

## 리턴값과 범위

- 변수의 소유권은 매번 같은 패턴을 따른다.
- 즉, 값을 변수에 할당하면 소유권이 옮겨진다.
- 힙 메모리에 저장된 변수의 데이터는 소유권이 다른 변수로 옮겨지지 않았다면 범위를 벗어날 때 `drop` 함수에 의해 제거된다.

```
// gives_ownership 함수의 리턴값이 변수 s1로 옮겨진다.
let s1 = gives_ownership();
// 변수 s2가 범위 내에 생성된다.
let s2 = String::from("hello");
// 변수 s2가 takes_and_give_back 함수로 옮겨간 후 리턴값은 변수 s3로 옮겨진다.
let s3 = takes_and_gives_back(s2);

...

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
```

---

# 4.2 참조와 대여

- 아래의 코드는 다음과 같은 과정을 따른다.

1. `calculate_length` 함수를 호출한다.
2. 이동할 문자열 변수를 다시 호출자 함수에 리턴해서 받아온다.
3. `calculate_length` 함수를 호출한 후에도 `String` 변수를 계속 사용할 수 있다.

- 이렇게 값의 소유권을 가져오는 대신, 매개변수로 전달된 객체의 참조를 이용하도록 `calculate_length` 함수를 작성하였다.

```
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("Length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- `calculate_length` 함수의 선언부에서는 `String` 대신 `&String`을 사용하여 함수를 호출할 때는 `&s1`과 같이 값을 전달하고 있다.
- `&` 사용하면 소유권을 가져오지 않고도 값을 참조할 수 있다.
- `&s1` 문법을 이용하면 변수 `s1`의 값은 읽을 수 있지만 소유권은 가져오지 않는 참조를 생성할 수 있다.
- 참조는 소유권을 갖지 않기 때문에 참조가 가리키는 값은 참조가 범위를 벗어나더라도 `drop` 함수가 호출되지 않는다.
- 함수의 시그너처에도 `&`를 이용해 매개변수 `s`의 타입이 참조임을 표현한다.
- 이처럼 함수 매개변수로 참조를 전달하는 것을 대여(borrowing)라고 한다.
- 단, 아래와 같이 대여한 변수를 수정하려고 하면 실제로 동작하지 않는다.
- 변수가 기본적으로 불변인 것처럼, 참조도 기본적으로 불변이다.

```
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str("aaaa");
}
```

## 가변 참조

- 아래와 같이 코드를 조금 수정하면 이 에러를 수정할 수 있다.

```
fn change(some_string: &mut String) {
    some_string.push_str("aaaa");
}
```

- 우선 변수 `s`에 `mut` 키워드를 추가한다.
- 그런 다음 `&mut s`와 같이 가변 참조를 생성한 후 `some_string: &mut String`과 같이 가변 참조를 전달받으면 된다.
- 하지만 가변 참조는 오직 한 개만 존재해야 한다.
- 이러한 제약 덕분에 가변 참조를 제어하면서 사용할 수 있다.
- 또한 러스트는 race condition과 유사한 data races를 컴파일 시점에 방지할 수 있다.

> - 둘 혹은 그 이상의 포인터가 동시에 같은 데이터를 읽거나 쓰기 위해 접근할 때
> - 최소한 하나의 포인터가 데이터를 쓰기 위해 사용될 때
> - 데이터에 대한 접근을 동기화할 수 있는 매커니즘이 없을 때

- 데이터 경합은 예측할 수 없는 결과를 유발하며, 런타임에 디버깅하기 힘들다.

## 죽은 참조

- 포인터를 사용하는 언어는 죽은 포인터로 인해 에러가 발생하기 쉽다.
- 러스트는 죽은 참조가 발생하지 않도록 컴파일러가 보장해준다.
- 즉, 어떤 데이터에 대한 참조를 생성하면 컴파일러가 해당 데이터에 대한 참조를 실행하기에 앞서 데이터가 범위를 벗어나지 않았는지 확인해 준다.
- 다음 코드는 죽은 참조가 발생했을 때 컴파일러가 에러를 발생시키는 예시이다.

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

```
error[E0106]: missing lifetime specifier
 --> src\main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `orphan` (bin "orphan") due to previous error
```

- 이 에러 메시지는 수명에 대해 언급하고 있다.
- 수명은 추후 살펴볼 내용이고, 실제 오류는 대여할 값이 존재하지 않는다는 의미이다.
- 실제로 C언어는 죽은 포인터를 참조해도 컴파일러가 오류를 알려주지 않는다.

```
char* s = (char*)malloc(sizeof(5));
sprintf(s, "asdf", 5);
printf("%s\n", s);

free(s);
printf("%d\n", s);
```

- 위의 러스트 코드에서는 변수 `s`가 함수 `dangle` 함수 내에서 생성되었기 때문에 `dangle` 함수의 실행이 종료되는 시점에 변수 `s`의 메모리가 해제된다.
- 즉, 이 참조가 가리키는 메모리에는 유효하지 않은 `String` 타입의 값이 보관되어 있다.
- 이 문제를 해결하려면 다음과 같이 `String` 타입을 직접 리턴하는 방법이 있다.

