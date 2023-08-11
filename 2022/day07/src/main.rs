/**
* Couldn't get this one to work :(
* It worked on the test data but refused to work on the actual input
* I'll revisit when less burned out
**/
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box <dyn ::std::error::Error>>;

struct Directory {
	file_sizes: u128,
	subdirectories: Vec<String>,
}

fn main() -> Result<()> {
    let input = include_str!("../test.txt");

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn get_total_size(directories: &HashMap<&str, Directory>, dir_name: &str) -> u128 {
	if let Some(dir) = directories.get(dir_name) {
		let mut total_size = dir.file_sizes;
		// println!("{dir_name}: {total_size}");

		for subdir_name in dir.subdirectories.iter() {
		/* 	print!(" - "); */
			total_size += get_total_size(directories, subdir_name);
		}

		return total_size;
	}
	panic!("Shouldn't happen");
}

fn part1(input: &str) -> Result<()> {
	let mut directories: HashMap<&str, Directory> = HashMap::new();
	let mut lines = input.split("\n").peekable();

	while let Some(line) = lines.next() {
		match line {
			"$ cd .." => {}
			"" => {}
			_ if &line[0..3] == "$ c" => {
				let dir_name = &line[5..];

				directories.entry(dir_name).or_insert(
					Directory {
						file_sizes: 0,
						subdirectories: Vec::new(),
					}
				);

				lines.next(); // $ ls
				while let Some(item) = lines.next_if(|l| l.get(0..1).is_some() && l.get(0..1) != Some("$")) {
					if &item[0..1] == "d" { // directory
						directories
							.entry(dir_name)
							.and_modify(|dir| dir.subdirectories.push(item[4..].to_string()));
						directories.insert(
							&item[4..],
							Directory {
								file_sizes: 0,
								subdirectories: Vec::new(),
							}
						);
					}
					else { // file
						directories
							.entry(dir_name)
							.and_modify(|dir: &mut Directory|
								dir.file_sizes += item.split(" ").next().unwrap().parse::<u128>().unwrap());
					}
				}
			}
			_ => {
			}
		}
	}

	let mut retval: u128 = 0;
	for (k, v) in directories.iter() {
		let s = get_total_size(&directories, k);
		println!("{s}");

		if s < 100_000 { retval += s }
	}

	println!("{retval}");

	Ok(())
}

fn part2(input: &str) -> Result<()> {

	Ok(())
}
