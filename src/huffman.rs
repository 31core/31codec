use std::rc::Rc;

#[derive(Clone, Default, Debug)]
struct HuffmanNode {
    byte: u8,
    value: usize,
    left: Option<Rc<HuffmanNode>>,
    right: Option<Rc<HuffmanNode>>,
}

impl HuffmanNode {
    fn build(freq_list: &[usize]) -> Self {
        fn find_min(freq_list: &mut Vec<HuffmanNode>) -> HuffmanNode {
            let mut min = 0;
            for i in 0..freq_list.len() {
                if freq_list[i].value < freq_list[min].value {
                    min = i;
                }
            }
            let min2 = freq_list[min].clone();
            freq_list.remove(min);
            min2
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
        bits
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
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/** dump Huffman tree into binary */
fn dump_to_bits(root: &HuffmanNode) -> Vec<u8> {
    let mut bits = Vec::new();
    if root.is_leaf() {
        bits.push(1);
        bits.push(root.byte);
    } else {
        bits.push(0);
        bits.extend(dump_to_bits(&root.left.clone().unwrap()));
        bits.extend(dump_to_bits(&root.right.clone().unwrap()));
    }
    bits
}

/** load Huffman tree from binary */
fn load_from_bits(bits: &[u8]) -> (HuffmanNode, usize) {
    let mut root = HuffmanNode::default();
    if *bits.first().unwrap() == 0 {
        let (left, i) = load_from_bits(&bits[1..]);
        root.left = Some(Rc::new(left));
        let (right, j) = load_from_bits(&bits[i + 1..]);
        root.right = Some(Rc::new(right));
        (root, i + j + 1)
    } else {
        root.byte = bits[1];
        (root, 2)
    }
}

fn bits_to_byte(bits: &[u8]) -> u8 {
    let mut byte = 0;
    for (i, bit) in bits.iter().enumerate().take(8) {
        byte |= *bit << (8 - i);
    }
    byte
}

fn byte_to_bits(byte: u8) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in 0..8 {
        bits.push((byte >> (8 - i)) & 1);
    }
    bits
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    /* statistical the frequencies */
    let mut freq_list = [0; 256];
    for byte in bytes {
        freq_list[*byte as usize] += 1;
    }

    let root = HuffmanNode::build(&freq_list);

    let mut compressed_data_bits = Vec::new();
    for byte in bytes {
        let mut bits = root.get_huffman_code(*byte);
        bits.reverse();
        compressed_data_bits.extend(bits);
    }
    let compressed_size = compressed_data_bits.len();

    let mut compressed_data = Vec::new();
    if compressed_data_bits.len() < 8 {
        compressed_data_bits.extend(vec![0; 8 - compressed_data_bits.len()]);
    }
    let mut i = 0;
    while i < compressed_data_bits.len() {
        compressed_data.push(bits_to_byte(&compressed_data_bits));
        i += 8;
    }

    let mut data = Vec::new();
    let huffman_table = dump_to_bits(&root);
    data.extend((compressed_size as u32).to_be_bytes());
    data.extend(huffman_table);
    data.extend(compressed_data);
    data
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let size = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let (huffman_table, table_size) = load_from_bits(&bytes[4..]);
    let compressed_data = &bytes[table_size + 4..];

    let mut compressed_data_bits = Vec::new();
    let mut data = Vec::new();
    for byte in compressed_data {
        compressed_data_bits.extend(byte_to_bits(*byte));
    }

    let mut i = 0;
    while i <= size as usize {
        let (byte, j) = huffman_table.get_byte(&compressed_data_bits[i..]);
        data.push(byte);
        i += j;
    }

    data
}
