use crate::bitstream::{BitStreamReader, BitStreamWriter};
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
        let mut dict = vec![Vec::new(); 256];
        for (i, byte) in dict.iter_mut().enumerate() {
            *byte = self.get_huffman_code(i as u8);
        }

        dict
    }
    /** Get a byte code by Huffman code */
    fn get_byte(&self, bits: &mut BitStreamReader) -> u8 {
        let mut root = Rc::new(self.clone());
        while !root.is_leaf() {
            if bits.read() == 0 {
                root = root.left.clone().unwrap();
            } else {
                root = root.right.clone().unwrap();
            }
        }
        root.byte
    }
    /** load Huffman tree from binary */
    fn load_from_bits(bits: &mut BitStreamReader, words: &mut Vec<u8>) -> Self {
        let mut root = Self::default();
        if bits.read() == 0 {
            let left = Self::load_from_bits(bits, words);
            root.left = Some(Rc::new(left));
            let right = Self::load_from_bits(bits, words);
            root.right = Some(Rc::new(right));
            root
        } else {
            root.byte = *words.first().unwrap();
            words.remove(0);
            root
        }
    }

    /** dump Huffman tree into binary */
    fn dump_to_bits(&self, bits: &mut BitStreamWriter, words: &mut Vec<u8>) {
        if self.is_leaf() {
            bits.write(1);
            words.push(self.byte);
        } else {
            bits.write(0);
            self.left.clone().unwrap().dump_to_bits(bits, words);
            self.right.clone().unwrap().dump_to_bits(bits, words);
        }
    }
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let root = HuffmanNode::build(&HuffmanNode::stat_freq(bytes));

    let mut compressed_data_bits = BitStreamWriter::default();
    let dict = root.get_dict();
    for byte in bytes {
        for bit in &dict[*byte as usize] {
            compressed_data_bits.write(*bit);
        }
    }

    let mut data = Vec::new();
    let mut huffman_table = BitStreamWriter::default();
    let mut words = Vec::new();
    root.dump_to_bits(&mut huffman_table, &mut words);

    data.extend((compressed_data_bits.total_bits() as u32).to_be_bytes()); // store bits count
    data.extend(words);
    data.extend(huffman_table.data); // store huffman tree
    data.extend(compressed_data_bits.data);
    data
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let size = u32::from_be_bytes(bytes[0..4].try_into().unwrap()) as usize;
    let mut bitts = BitStreamReader::from_bytes(&bytes[4 + 256..]);
    let mut words = bytes[4..4 + 256].to_vec();
    let huffman_table = HuffmanNode::load_from_bits(&mut bitts, &mut words);
    let compressed_data = &bytes[4 + 256 + bitts.total_bytes()..];

    let mut compressed_data_bits = BitStreamReader::from_bytes(compressed_data);
    let mut data = Vec::new();
    while compressed_data_bits.total_bits() < size {
        let byte = huffman_table.get_byte(&mut compressed_data_bits);
        data.push(byte);
    }

    data
}
