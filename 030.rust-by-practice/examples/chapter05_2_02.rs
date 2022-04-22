fn main() {
    let x = 5;
    let y = &x;

    // modify this line only
    assert_eq!(5, *y);
}
