use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

struct Todo {
    // 使用 Rust 内置的 HashMap 来保存 key - val 键值对
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // 打开 db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // 序列化 json 为 HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    fn new_txt() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        //    let map: HashMap<String, bool> = content
        //        .lines()
        //        .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //        .map(|v| (v[0], v[1]))
        //        .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //        .collect();
        let mut map = HashMap::new();
        for entries in content.lines() {
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        Ok(Todo { map })
    }
    fn insert(&mut self, key: String) {
        // 在我们的 map 中新增一个新的元素
        // 我们默认将其状态值设置为 true
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // 打开 db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // 通过 Serde 写文件
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    fn save_txt(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify anaction");
    let item = std::env::args().nth(2).expect("Please specify an item");
    let file = std::env::args()
        .nth(3)
        .expect("Please specify an save file format, txt or json");
    println!("{:?}, {:?}, {:?}", action, item, file);

    if file == "json" {
        let mut todo = Todo::new().expect("Initialisation of db failed");
        if action == "add" {
            todo.insert(item);
            match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            }
        } else if action == "complete" {
            match todo.complete(&item) {
                None => println!("'{}' is not present in the list", item),
                Some(_) => match todo.save() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occurred: {}", why),
                },
            }
        }
    } else if file == "txt" {
        let mut todo = Todo::new_txt().expect("Initialisation of db failed");
        if action == "add" {
            todo.insert(item);
            match todo.save_txt() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            }
        } else if action == "complete" {
            match todo.complete(&item) {
                None => println!("'{}' is not present in the list", item),
                Some(_) => match todo.save_txt() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occurred: {}", why),
                },
            }
        }
    }
}
