fn main() {
  let x = 1024;
  let y = &x;
  println!("x: {}", x);
  println!("y: {:?}", y);
  if *y == 1024 {
    println!("y is 1024");
  }
  assert_eq!(x, *y);
}