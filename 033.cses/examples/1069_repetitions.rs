fn repetitions(str: Vec<char>) -> i32 {
    let (mut i, mut j) = (0, 1);
    let mut max = 1;
    while i < str.len() && j < str.len() {
        if str[i] == str[j] {
            max = max.max(j - i + 1);
            j += 1;
        } else {
            i = j;
            j += 1;
        }
    }
    max as i32
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let str = input.trim();
    if str.len() == 1 {
        println!("1");
        return;
    }
    let result = repetitions(str.chars().collect::<Vec<char>>());
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            repetitions("ATTCGGGA".to_string().chars().collect::<Vec<char>>()),
            3
        );
    }
}