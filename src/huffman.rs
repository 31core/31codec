use std::rc::Rc;

#[derive(Clone, Default, Debug)]
struct HuffmanNode {
    byte: u8,
    value: usize,
    left: Option<Rc<HuffmanNode>>,
    right: Option<Rc<HuffmanNode>>,
}

impl HuffmanNode {
    /* statistical the frequencies */
    fn stat_freq(bytes: &[u8]) -> [usize; 256] {
        let mut freq_list = [0; 256];
        for byte in bytes {
            freq_list[*byte as usize] += 1;
        }

        freq_list
    }
    fn build(freq_list: &[usize]) -> Self {
        fn find_min(freq_list: &mut Vec<HuffmanNode>) -> HuffmanNode {
            let mut min_pos = 0;
            for i in 0..freq_list.len() {
                if freq_list[i].value < freq_list[min_pos].value {
                    min_pos = i;
                }
            }
            let min = freq_list[min_pos].clone();
            freq_list.remove(min_pos);
            min
        }

        let mut freq_node_list = Vec::new();

        for (byte, freq) in freq_list.iter().enumerate() {
            let node = HuffmanNode {
                byte: byte as u8,
                value: *freq,
                ..Default::default()
            };
            freq_node_list.push(node);
        }

        while freq_node_list.len() > 1 {
            let mut upper = HuffmanNode::default();
            let min1 = find_min(&mut freq_node_list);
            let min2 = find_min(&mut freq_node_list);

            upper.left = Some(Rc::new(min1));
            upper.right = Some(Rc::new(min2));
            upper.value = upper.left.clone().unwrap().value + upper.right.clone().unwrap().value;
            freq_node_list.push(upper);
        }

        freq_node_list.first().unwrap().clone()
    }
    /** Get Huffman code by byte */
    fn get_huffman_code(&self, byte: u8) -> Vec<u8> {
        fn get_huffman_code(root: &HuffmanNode, byte: u8, bits: &mut Vec<u8>) -> bool {
            if root.is_leaf() {
                root.byte == byte
            } else {
                if get_huffman_code(&root.left.clone().unwrap(), byte, bits) {
                    bits.push(0);
                    return true;
                }
                if get_huffman_code(&root.right.clone().unwrap(), byte, bits) {
                    bits.push(1);
                    return true;
                }
                false
            }
        }

        let mut bits = Vec::new();
        get_huffman_code(self, byte, &mut bits);
        bits.reverse();
        bits
    }
    fn get_dict(&self) -> Vec<Vec<u8>> {
        let mut dict = vec![Vec::new(); 255];
        for (i, byte) in dict.iter_mut().enumerate() {
            *byte = self.get_huffman_code(i as u8);
        }

        dict
    }
    /** Get a byte code by Huffman code */
    fn get_byte(&self, bits: &[u8]) -> (u8, usize) {
        let mut i = 0;
        let mut root = Rc::new(self.clone());
        while !root.is_leaf() {
            if bits[i] == 0 {
                root = root.left.clone().unwrap();
            } else {
                root = root.right.clone().unwrap();
            }
            i += 1;
        }
        (root.byte, i)
    }
    /** load Huffman tree from binary */
    fn load_from_bits(bits: &[u8]) -> (Self, usize) {
        let mut root = Self::default();
        if *bits.first().unwrap() == 0 {
            let (left, i) = Self::load_from_bits(&bits[1..]);
            root.left = Some(Rc::new(left));
            let (right, j) = Self::load_from_bits(&bits[i + 1..]);
            root.right = Some(Rc::new(right));
            (root, i + j + 1)
        } else {
            root.byte = bits[1];
            (root, 2)
        }
    }

    /** dump Huffman tree into binary */
    fn dump_to_bits(&self) -> Vec<u8> {
        let mut bits = Vec::new();
        if self.is_leaf() {
            bits.push(1);
            bits.push(self.byte);
        } else {
            bits.push(0);
            bits.extend(self.left.clone().unwrap().dump_to_bits());
            bits.extend(self.right.clone().unwrap().dump_to_bits());
        }
        bits
    }
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

fn bits_to_byte(bits: &[u8]) -> u8 {
    let mut byte = 0;
    for (i, bit) in bits.iter().enumerate().take(8) {
        byte |= *bit << (7 - i);
    }
    byte
}

fn byte_to_bits(byte: u8) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in 0..8 {
        bits.push((byte >> (7 - i)) & 1);
    }
    bits
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let root = HuffmanNode::build(&HuffmanNode::stat_freq(bytes));

    let mut compressed_data_bits = Vec::new();
    let dict = root.get_dict();
    for byte in bytes {
        compressed_data_bits.extend(&dict[*byte as usize]);
    }

    let compressed_size = compressed_data_bits.len();
    let mut compressed_data = Vec::new();
    if compressed_data_bits.len() < 8 {
        compressed_data_bits.extend(vec![0; 8 - compressed_data_bits.len()]);
    }
    let mut i = 0;
    while i < compressed_data_bits.len() {
        compressed_data.push(bits_to_byte(&compressed_data_bits[i..]));
        i += 8;
    }

    let mut data = Vec::new();
    let huffman_table = root.dump_to_bits();
    data.extend((compressed_size as u32).to_be_bytes());
    data.extend(huffman_table);
    data.extend(compressed_data);
    data
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let size = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    let (huffman_table, table_size) = HuffmanNode::load_from_bits(&bytes[4..]);
    let compressed_data = &bytes[table_size + 4..];

    let mut compressed_data_bits = Vec::new();
    let mut data = Vec::new();
    for byte in compressed_data {
        compressed_data_bits.extend(byte_to_bits(*byte));
    }

    let mut i = 0;
    while i < size {
        let (byte, j) = huffman_table.get_byte(&compressed_data_bits[i..]);
        data.push(byte);
        i += j;
    }

    data
}
