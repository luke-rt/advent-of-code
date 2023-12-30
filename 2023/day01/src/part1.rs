type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

pub fn part1(input: &str) -> Result<()> {
	let lines = input.split("\n");

	let mut total = 0;
	for line in lines {
		let mut first: u32 = 0;
		let mut is_first = true;
		let mut curr: u32 = 0;
		for ch in line.chars() {
			if let Some(v) = ch.to_digit(10) {
				if is_first {
					first = v;
					is_first = false;
				}
				curr = v;
			}
		}

		total += (first*10) + curr;
	}

	println!("{total}");

    Ok(())
}

