use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="scrape_url")]
struct Opt {
    #[structopt(help="input url")]
    pub url: String,
    #[structopt(help="output file, stdout if not present")]
    pub output: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let url = opt.url;
    let output = &opt.output.unwrap_or("rust.md".to_string());

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
