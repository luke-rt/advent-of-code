use std::array;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

const N: usize = 99;

pub fn part1(input: &str) -> Result<()> {
    let height_grid: [[u8; N]; N] = array::from_fn(|i| {
        array::from_fn(|j| {
            input
                .chars()
                .nth((i * N) + j + i)
                .expect("Input size is known")
                .to_digit(10)
                .expect("Input is known") as u8
        })
    });

    let mut visibility_grid: [[bool; N]; N] =
        array::from_fn(|i| array::from_fn(|j| i == 0 || i == N - 1 || j == 0 || j == N - 1));

	let mut retval = N*4 - 4;

	for x in 1..(N - 1) {
        for y in 1..(N - 1) {
            if check_adj(x, y, 0, 1, &mut visibility_grid, &height_grid)
                || check_adj(x, y, 0, -1, &mut visibility_grid, &height_grid)
                || check_adj(x, y, 1, 0, &mut visibility_grid, &height_grid)
                || check_adj(x, y, -1, 0, &mut visibility_grid, &height_grid)
            {
                visibility_grid[x][y] = true;
				retval += 1;
            }
        }
    }

	println!("{retval}");

    Ok(())
}

fn check_adj(
    x1: usize,
    y1: usize,
    i: i8,
    j: i8,
    v_grid: &mut [[bool; N]; N],
    h_grid: &[[u8; N]; N],
) -> bool {
    let x2 = (x1 as i8 + i) as usize;
    let y2 = (y1 as i8 + j) as usize;
    if h_grid[x1][y1] > h_grid[x2][y2] {
		if x2 == 0 || x2 == N-1 || y2 == 0 || y2 == N-1 {
			return true;
		}
        return check_adj(x1, y1, i + (1*i.signum()), j + (1*j.signum()), v_grid, h_grid);
    }
    return false;
}
