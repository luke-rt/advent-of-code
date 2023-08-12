use std::array;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

const N: usize = 99;

#[derive(Debug, Copy, Clone)]
struct Tree {
	up: usize,
	down: usize,
	left: usize,
	right: usize,
}

impl Tree {
	fn new() -> Tree {
		Tree {
			up: 0,
			down: 0,
			left: 0,
			right: 0,
		}
	}
}

enum Dir {
	Up,
	Down,
	Left,
	Right,
}

pub fn part2(input: &str) -> Result<()> {
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

    let mut view_grid = [[Tree::new(); N]; N];

	// up visibility [-1][0]
	for row in 1..N { // skips first row bc up should be 0
		for col in 0..N {
			view_grid[row][col].up = check_adj(row, col, -1, 0, Dir::Up, &view_grid, &height_grid);
		}
	}

	// down visibility [1][0]
	for row in (0..N-1).rev() {
		for col in 0..N {
			view_grid[row][col].down = check_adj(row, col, 1, 0, Dir::Down, &view_grid, &height_grid);
		}
	}

	// left visibility [0][-1]
	for row in 0..N {
		for col in 1..N {
			view_grid[row][col].left = check_adj(row, col, 0, -1, Dir::Left, &view_grid, &height_grid);
		}
	}

	// right visibility [0][1]
	for row in 0..N {
		for col in (0..N-1).rev() {
			view_grid[row][col].right = check_adj(row, col, 0, 1, Dir::Right, &view_grid, &height_grid);
		}
	}

	let mut max = 0;
	for arr in view_grid.iter() {
		for tree in arr {
			let tmp = tree.up * tree.down * tree.left * tree.right;
			if tmp > max { max = tmp }
		}
	}

	println!("{max}");

    Ok(())
}

fn check_adj(row: usize, col: usize, i: isize, j: isize, dir: Dir, v_grid: &[[Tree; N]; N], h_grid: &[[u8; N]; N]) -> usize {
	let x = (row as isize + i) as usize;
	let y = (col as isize + j) as usize;

	if x > N-1 || y > N-1 {
		// includes x < 0 bc 0usize - 1 is usize::MAX which is greater than N-1
		return 0;
	}
	if h_grid[row][col] <= h_grid[x][y] {
		return 1;
	}


	let v_grid_dir = match dir {
		Dir::Up => v_grid[x][y].up,
		Dir::Down => v_grid[x][y].down,
		Dir::Left => v_grid[x][y].left,
		Dir::Right => v_grid[x][y].right,
	};

	if v_grid_dir == 0 {
		return 1
	}

	let mut retval = 0;
	retval += v_grid_dir;
	retval += check_adj(row, col, i + (retval as isize * i.signum()), j + (retval as isize * j.signum()), dir, &v_grid, &h_grid);

	return retval;
}

