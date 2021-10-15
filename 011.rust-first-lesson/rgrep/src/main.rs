mod mest;

use std::error::Error;

use clap::{AppSettings, Clap};
use colored::Colorize;
// use globset::Glob;
use regex::Regex;
use tokio::fs;

#[derive(Clap)]
#[clap(version = "1.0", author = "Custer<custer@email.cn>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    find: String,
    #[clap(multiple_values = true)]
    paths: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 解析参数
    let opts: Opts = Opts::parse();
    let find = opts.find.as_str();
    let paths = opts.paths;

    // 2. 循环读取匹配到的文件
    for path in paths {
        println!("{:?}", path);
        let contents = fs::read_to_string(path).await?;

        // 3. 匹配字符串
        for (row, line) in contents.lines().enumerate() {
            if let Some(re) = Regex::new(find).unwrap().find(line) {
                let start = re.start();
                let end = re.end();
                println!(
                    "{}:{} {}{}{}",
                    row + 1,
                    start + 1,
                    &line[..start],
                    &line[start..end].red().bold(),
                    &line[end..]
                );
            }
        }
    }
    Ok(())
}
