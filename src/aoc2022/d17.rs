use std::io::Read;
struct Rock {
    shape: u32,
}
#[derive(Debug, Clone)]
struct Field {
    blocks: Vec<u8>,
    wind: Vec<bool>,
    wind_pos: usize,
}

impl Field {
    fn new(wind: Vec<bool>) -> Self {
        Self {
            blocks: vec![0xff],
            wind,
            wind_pos: 0,
        }
    }
    fn drop(&mut self, mut shape: u32, max_pos: i32) {
        let mut height = self.blocks.len() + 3;
        let mut pos = 2;
        loop {
            if self.wind[self.wind_pos] {
                if pos > 0 {
                    let new_shape = shape << 1;
                    if self.can_move(new_shape, height) {
                        shape = new_shape;
                        pos -= 1;
                    }
                }
            } else {
                if pos < max_pos {
                    let new_shape = shape >> 1;
                    if self.can_move(new_shape, height) {
                        shape = new_shape;
                        pos += 1;
                    }
                }
            }
            self.wind_pos = (self.wind_pos + 1) % self.wind.len();
            if !self.can_move(shape, height - 1) {
                self.place(shape, height);
                return;
            }
            height -= 1;
        }
    }
    fn can_move(&self, shape: u32, height: usize) -> bool {
        let mut sx = shape;
        let mut r = 0;

        while sx != 0 {
            if let Some(b) = self.blocks.get(height + r) {
                let overlap = b & (sx & 0x7f) as u8;
                if overlap != 0 {
                    return false;
                }
            }
            sx >>= 7;
            r += 1;
        }
        true
    }
    fn place(&mut self, shape: u32, height: usize) {
        let mut sx = shape;
        let mut r = 0;
        while sx != 0 {
            if let Some(b) = self.blocks.get_mut(height + r) {
                *b |= sx as u8 & 0x7f;
            } else {
                self.blocks.push(sx as u8 & 0x7f);
            }
            sx >>= 7;
            r += 1;
        }
    }
    fn draw(&self) {
        for i in (0..self.blocks.len()).rev() {
            let v = self.blocks[i];
            for bit in (0..7).rev() {
                if (1 << bit) & v == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}
pub fn f(file: std::fs::File) -> crate::AocResult {
    let rocks = [
        0b0011110,
        0b0001000_0011100_0001000,
        0b0000100_0000100_0011100,
        0b0010000_0010000_0010000_0010000,
        0b0011000_0011000,
    ];
    let max_pos = [3, 4, 4, 6, 5];
    let wind = file
        .bytes()
        .filter_map(|b| b.ok())
        .filter_map(|b| {
            if b == b'<' {
                Some(true)
            } else if b == b'>' {
                Some(false)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut field = Field::new(wind);
    for i in 0..2022 {
        let rock = i % rocks.len();
        field.drop(rocks[rock], max_pos[rock]);
    }
    let res1 = field.blocks.len() - 1;
    res1.into()
}
