fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October ",
        "November",
        "December",
    ];

    let first_month = months[0];
    println!("first_month: {first_month}");
    let halfyear = &months[..6];
    println!("halfyear: {:?}", halfyear);

    let mut monthsv = Vec::new();
    for month in months {
        monthsv.push(month);
    }

    println!("monthsv: {:?}", monthsv);
}
