mod days;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let day: String = std::env::args()
        .nth(1)
        .expect("No day given. Possible options are: 01-25.");
    let day_slice: &str = day.as_str();

    match day_slice {
        "01" => days::day01::run(include_str!("days/inputs/01.txt")),
        "02" => days::day02::run(include_str!("days/inputs/02.txt")),
        "03" => days::day03::run(include_str!("days/inputs/03.txt")),
        "04" => days::day04::run(include_str!("days/inputs/04.txt")),
        "05" => days::day05::run(include_str!("days/inputs/05.txt")),
        "06" => days::day06::run(include_str!("days/inputs/06.txt")),
        "07" => days::day07::run(include_str!("days/inputs/07.txt")),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };

    Ok(())
}
