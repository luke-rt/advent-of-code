mod part1;
mod part2;

type Result<T> = ::std::result::Result<T, Box <dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let input = include_str!("../day00.txt").trim();

    part1::part1(&input)?;
    part2::part2(&input)?;
    Ok(())
}

