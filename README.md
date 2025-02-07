[Rust] Reference and Borrow 2
===

## 참조와 대여 2

참조는 참조된 값을 '불러오기' 만 할 뿐이며, 소유권을 가져오는 것이 아니기 때문에

스코프가 끝난 뒤에도 값이 버려지지 않는다.

앰퍼센드(&) 참조자는 기본적으로 'immutable(불변)' 이기 때문에 변수에 가변성을 추가하는 요소인

"mut" 을 사용해서 가변성을 부여하듯, 동일하게 참조에서도 '&mut' 으로 가변성을 부여해 줄 수 있다.

```rust
fn main() {
  let mut s = String::from("Hello"); // 가변성을 가진 s 변수를 생함성
  
  change(&mut s); // 가변성을 가진 s 를 change로 전달

  println!("{s}"); // change의 결과를 출력함
}

fn change(some_string: &mut String) { // 가변 참조된 String을 넘겨줌
  some_string.push_str(", world");
}
```

**결과 출력**
```
Hello, world
```
