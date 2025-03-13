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

    pub fn right(&mut self) {
        let n = self.buffer.len() as i32 - self.gap_size as i32;
        if (self.cursor as i32) < n {
            let mut i = self.cursor + self.gap_size;
            while i > self.cursor {
                let temp = self.buffer[i - 1];
                self.buffer[i - 1] = self.buffer[i];
                self.buffer[i] = temp;
                i -= 1;
            }
            self.cursor += 1;
        }
    }

    pub fn left(&mut self) {
        if self.cursor > 0 {
            for i in self.cursor..(self.cursor + self.gap_size) {
                let temp = self.buffer[i];
                self.buffer[i] = self.buffer[i - 1];
                self.buffer[i - 1] = temp;
            }
            self.cursor -= 1;
        }
    }

    pub fn len(&self) -> usize {
        return self.buffer.len() - self.gap_size
    }


    pub fn display(&self) {
        for i in 0..self.buffer.len() {
            if i == self.cursor {
                print!("[")
            } 
            if self.buffer[i] == self.gap_value {
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
        //self.buffer.iter().filter(|&&c| c != self.gap_value).map(|c| c.to_string()).collect()
        self.buffer.iter().map(|c| c.to_string()).collect()
    }
}

impl From<Vec<u8>> for GapBuffer<char> {
    fn from(value: Vec<u8>) -> Self {
        let content = value.iter().map(|c| *c as char).collect::<Vec<char>>();
        GapBuffer {
            buffer: content,
            gap_size: 10,
            gap_value: '\0',
            cursor: 0,
        }
    }
}
