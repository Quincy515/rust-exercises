// 定义结构体 Student
struct Student {
    name: &'static str,
    score: i32,
}

fn main() {
    let score = 59;
    let username = "zhangsan";

    // 创建结构体的实例
    let mut student = Student {
        score,          // 将变量 score 的值赋值给结构体字段 score，两者有着相同的名称，可以简写
        name: username, // 实例中字段的顺序可以和结构体声明的顺序不一致
    };

    student.score = 60; // 使用“实例名.字段名”形式更改和访问结构体实例某个字段的值
    println!("name: {}, score: {}", student.name, student.score);

    let student2 = Student {
        name: "lisi",
        ..student // 其余字段的值与 student 对应字段的值相同。使用结构体更新语法
    };

    println!("name: {}, score: {}", student2.name, student2.score);
}
