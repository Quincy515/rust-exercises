fn return_fn(a: i32) -> impl Fn(i32) -> i32 {
  move |b| a + b
}

#[test]
fn return_fn_test() {
  assert_eq!(return_fn(5)(6), 11);
}

fn show_student_info(cls_name: String, name:String) {
  println!("class:{}, student name:{}", cls_name, name);
}

fn show_student_info_v2(cls_name: String) -> impl Fn(String) -> () {
  move |name| println!("class:{}, student name: {}", cls_name, name)
}
#[test]
fn show_student_info_test() {
  show_student_info(String::from("一班"), String::from("Jerry"));
  show_student_info(String::from("一班"), String::from("Allen"));
  show_student_info(String::from("一班"), String::from("Custer"));

  let show_student_info_clas1 = show_student_info_v2(String::from("一班"));
  show_student_info_clas1(String::from("Jerry"));
  show_student_info_clas1(String::from("Allen"));
  show_student_info_clas1(String::from("Custer"));
}