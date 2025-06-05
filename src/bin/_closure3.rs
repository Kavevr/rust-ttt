fn main() {
    let x = 4;
    // let b = 10;
    // 闭包会从当前作用域去寻找xa
    let equal_to_x = |z| z == x;
    let y = 13;
    // assert!(equal_to_x(y));
    assert_eq!(equal_to_x(y),true);
}