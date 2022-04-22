///结构体的所有权
///当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。
///当这么做时，部分 move 就会发生：
///变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。
///
///在这种情况下，原变量将无法再被使用，
///但是它没有转移所有权的那一部分依然可以使用，也就是之前被引用的那部分。
///
///示例


fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}


