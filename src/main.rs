mod days;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let day: String = std::env::args()
        .nth(1)
        .expect("No day given. Possible options are: 01-25.");
    let day_slice: &str = day.as_str();

    match day_slice {
        "01" => days::day1::run(),
        "02" => days::day2::run(),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };

    Ok(())
}
