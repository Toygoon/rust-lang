<!-- omit in toc -->

> # Chapter 05. 구조체를 활용한 관련 데이터의 구조화
>
> - 구조체는 서로 관련이 있는 여러 값을 하나로 모으고, 이름을 지정해 접근할 수 있는 타입이다.

<details>
<summary>Table of Contents</summary>

- [5-1 구조체 정의와 인스턴스 생성](#5-1-구조체-정의와-인스턴스-생성)

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
