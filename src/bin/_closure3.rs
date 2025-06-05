fn main() {
    let x = 4;
    // 闭包会从当前作用域去寻找x
    let equal_to_x = |z| z == a;
    let y = 13;
    // assert!(equal_to_x(y));
    assert_eq!(equal_to_x(y),true);
}