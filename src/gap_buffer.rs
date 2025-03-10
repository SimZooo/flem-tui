use core::fmt;

#[derive(Debug)]
pub struct GapBuffer<T> {
    pub gap_size: usize,
    pub cursor: usize,
    pub buffer: Vec<T>,
    pub gap_value: T,
}

impl<T: Copy + fmt::Display + PartialEq> GapBuffer<T> {
    pub fn new(gap_size: usize, gap_value: T) -> Self {
        Self {
            gap_size,
            cursor: 0,
            gap_value,
            buffer: vec![gap_value; gap_size]
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.gap_size == 1 {
            self.gap_size = 10;
            for i in 0..self.gap_size {
                self.buffer.insert(self.cursor + i, self.gap_value);
            }
        } else {
            self.gap_size -= 1;
        }
        self.buffer[self.cursor] = value;
        self.cursor += 1;
    }

    pub fn delete(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let c = self.buffer.remove(self.cursor);
        }
    }

    pub fn display(&self) {
        for i in 0..self.buffer.len() {
            if i == self.cursor {
                print!("[")
            } if self.buffer[i] == self.gap_value {
                print!("_")
            }
            print!("{}", self.buffer[i]);
            if i == self.cursor + self.gap_size - 1 {
                print!("]")
            }
        }
        println!("")
    }

    pub fn to_string(&self) -> String {
        self.buffer.iter().filter(|&&c| c != self.gap_value).map(|c| c.to_string()).collect()
    }
}
