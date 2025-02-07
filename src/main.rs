// Reference and Borrow 2 . 25.02.07
// 참조와 대여 2

/*
    참조된 값을 '불러오기' 만 할 뿐, 소유권을 가져오는 것이 아니기 때문에
    스코프가 끝난 이후에도 값이 버려지지 않는다.
    앰퍼센드(&) 참조자는 변수이와 같이 기본적으로 "immutable(불변)" 이다.
    변수에 가변성을 추가하는 요소인 'mut' 을 함께 사용함으로써 가변성을 부여하듯
    참조자도 mut을 사용하여 가변성을 부여해주어야 한다.
*/

fn main() {
    /*
        // 대여한 값을 수정하기
        let s = String::from("hello");

        change(&s);
    }

    fn change(some_string: &String) {
        some_string.push_str(", world");
        // 이 코드는 동작하지 않는다. 왜냐하면,
        // 변수가 기본적으로 불변성을 지니듯, 참조자 또한 참조하는 것을 수정할 수 없다.
        */

    /* 여기서부터 정상적으로 실행이 가능한 코드
    위의 코드와 무엇이 다른지 확인해야 이해함 */
    let mut s = String::from("Hello");
    // 변수를 mut을 이용하여 가변형 변수 s를 생성함.
    change(&mut s);
    // 가변 참조자 s를 change에 넘김.
    println!("{s}");
}

fn change(some_string: &mut String) {
    // 가변 참조된 String을 넘겨줌.
    some_string.push_str(", world");

    /* 가변 참조를 이용하여 빌린 값을 변경할 수 있다.

    */
}

// 출력 결과
// Hello, World.
