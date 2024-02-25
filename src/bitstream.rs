#[derive(Default)]
pub struct BitStream {
    pub data: Vec<u8>,
    byte_ptr: usize,
    bit_ptr: usize,
}

impl BitStream {
    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
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
