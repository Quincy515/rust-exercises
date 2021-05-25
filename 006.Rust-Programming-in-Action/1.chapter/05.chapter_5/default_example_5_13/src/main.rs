#[derive(Default, Debug)]
struct MyStruct {
    foo: i32,
    bar: f32,
}
fn main() {
    let options1: MyStruct = Default::default(); // 为 MyStruct 的所有字段提供默认值
    let options2 = MyStruct {
        foo: 7,
        ..Default::default()
    }; // 自定义 MyStruct 的一部分字段，而其他字段使用 ..Default::default() 设置为默认值

    println!("options1: {:?}", options1);
    println!("options2: {:?}", options2);
}
