mod days;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let day: String = std::env::args()
        .nth(1)
        .expect("No day given. Possible options are: 01-25.");
    let day_slice: &str = day.as_str();

    match day_slice {
        "01" => days::day1::run(include_str!("days/inputs/01.txt")),
        "02" => days::day2::run(include_str!("days/inputs/02.txt")),
        "03" => days::day3::run(include_str!("days/inputs/03.txt")),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };

    Ok(())
}
