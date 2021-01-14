use std::{thread, time};

const WIDTH: usize = 40;
const HEIGHT: usize = 40;

#[derive(Debug)]
struct Space {
	w: usize,
	h: usize,
	s: [[u8; WIDTH]; HEIGHT],
}

impl Space {
	fn display(&self) {
		print!("{}[2J", 27 as char);
		for y in 0..WIDTH {
			for x in 0..HEIGHT {
				let pixel: char;
				match self.s[x][y] {
					1 => pixel = 's',
					2 => pixel = 'w',
					_ => pixel = ' ',
				}
				print!("{}", pixel);
			}
			print!("\n");
		}
	}

	// to be implemented
	fn sim(& mut self) {
		let width = self.w;
		let height = self.h;
		loop {
			for x in (0..(width-1)).rev() {
				for y in (0..(height-1)).rev() {
					if x != 0 && x+1 != width && y+1 != height {
						/* Sim rules for Sand */
						if 1 == self.s[x][y] {
							if self.s[x][y+1] != 1 {
								self.s[x][y] = self.s[x][y+1];
								self.s[x][y+1] = 1;
							} else if self.s[x-1][y+1] != 1 {
								self.s[x][y] = self.s[x-1][y+1];
								self.s[x-1][y+1] = 1;
							} else if self.s[x+1][y+1] != 1 {
								self.s[x][y] = self.s[x+1][y+1];
								self.s[x+1][y+1] = 1;
							}
						}
						/* Sim rules for Water */
						if 2 == self.s[x][y] {
							if self.s[x][y+1] == 0 {
								self.s[x][y] = self.s[x][y+1];
								self.s[x][y+1] = 2;
							} else if self.s[x-1][y+1] == 0 {
								self.s[x][y] = self.s[x-1][y+1];
								self.s[x-1][y+1] = 2;
							} else if self.s[x+1][y+1] == 0 {
								self.s[x][y] = self.s[x+1][y+1];
								self.s[x+1][y+1] = 2;
							} else if self.s[x-1][y] == 0 {
								self.s[x][y] = self.s[x-1][y];
								self.s[x-1][y] = 2;
							} else if self.s[x+1][y] == 0 {
								self.s[x][y] = self.s[x+1][y];
								self.s[x+1][y] = 2;
							}
						}
					}
				}
			}
			/* Pouring sand @ 1/3 width */
			if self.s[width/3][1] != 1 {
				self.s[width/3][0] = 1;
			}
			/* Dripping water @ 2/3 width */
			if self.s[2*width/3][1] != 2 {
				self.s[2*width/3][0] = 2;
			}
			self.display();

			thread::sleep(time::Duration::from_millis(16));  /* for approx 60 "frames" on terminal */
		}
	}
}

fn main() {
	let mut space: Space = Space {
		w: WIDTH,
		h: HEIGHT,
		s: [[0; WIDTH]; HEIGHT],
	};
	space.sim();
}
