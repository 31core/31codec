pub mod dct;
pub mod format;
pub mod frames;
pub mod huffman;
pub mod mat;

mod bitstream;

use yuv::{Bitstream, YUV420Frame, YUVFrame};

pub fn encode_frame<T>(src: &T) -> Vec<u8>
where
    T: YUVFrame + Bitstream + Clone,
{
    let mut src = src.clone();
    dct::decode_frame(&mut src);
    huffman::encode(&src.dump())
}

pub fn decode_frame<T>(src: &[u8], width: usize, height: usize) -> YUV420Frame
where
    T: YUVFrame + Bitstream + Clone,
{
    let data = huffman::decode(src);
    YUV420Frame::load(&huffman::decode(&data), width, height)
}
