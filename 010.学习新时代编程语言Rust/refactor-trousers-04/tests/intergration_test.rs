use rust_learn::coat::Shirt;
use rust_learn::mother_change_len;

#[test]
fn mother_change_len_itest() {
    let mut shirt = Shirt::new(10);
    mother_change_len(&mut shirt, 8);
    assert!(shirt.get_len() == 8, "shirt's len is not equals to 8");
}
