#[derive(Default)]
pub struct BitStreamWriter {
    pub data: Vec<u8>,
    byte_ptr: usize,
    bit_ptr: usize,
}

impl BitStreamWriter {
    pub fn write(&mut self, bit: u8) {
        if self.data.len() <= self.byte_ptr {
            self.data.push(0);
        }
        if bit == 1 {
            self.data[self.byte_ptr] |= 1 << (7 - self.bit_ptr);
        }
        self.bit_ptr += 1;
        if self.bit_ptr == 8 {
            self.bit_ptr = 0;
            self.byte_ptr += 1;
        }
    }
    pub fn total_bits(&self) -> usize {
        8 * self.byte_ptr + self.bit_ptr
    }
}

#[derive(Default)]
pub struct BitStreamReader<'a> {
    pub data: &'a [u8],
    byte_ptr: usize,
    bit_ptr: usize,
}

impl<'a> BitStreamReader<'a> {
    pub fn from_bytes(data: &'a [u8]) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }
    pub fn read(&mut self) -> u8 {
        let bit = (self.data[self.byte_ptr] >> (7 - self.bit_ptr)) & 1;
        self.bit_ptr += 1;
        if self.bit_ptr == 8 {
            self.bit_ptr = 0;
            self.byte_ptr += 1;
        }

        bit
    }
    pub fn total_bytes(&self) -> usize {
        if self.bit_ptr > 0 {
            self.byte_ptr + 1
        } else {
            self.byte_ptr
        }
    }
    pub fn total_bits(&self) -> usize {
        8 * self.byte_ptr + self.bit_ptr
    }
}
