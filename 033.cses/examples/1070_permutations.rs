fn solved(n: i32) {
    if n == 1 {
        println!("1");
        return;
    }
    if n == 2 || n == 3 {
        println!("NO SOLUTION");
        return;
    }
    if n % 2 == 0 {
        (2..=n).step_by(2).for_each(|x| print!("{} ", x));
        (1..n).step_by(2).for_each(|x| print!("{} ", x));
    } else {
        (1..n).rev().step_by(2).for_each(|x| print!("{} ", x));
        (1..=n).rev().step_by(2).for_each(|x| print!("{} ", x));
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    solved(n.trim().parse::<i32>().unwrap());
}
