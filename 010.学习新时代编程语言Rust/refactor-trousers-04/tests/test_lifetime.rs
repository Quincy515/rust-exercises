fn get_max<'a>(num1: &'a i32, num2: &'static i32) -> &'a i32 {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}

#[test]
fn test_lifetime() {
    let num1 = 5;
    let mut max = &0;
    {
        const num2: i32 = 6;
        max = get_max(&num1, &num2);
    }
    assert_eq!(&6, max);
}
