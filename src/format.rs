const MAGIC_HEADER: [u8; 4] = [0x31, 0x0c, 0x00, b'p'];
const VERSION: u8 = 1;

const PIX_FMT_YUV420P: u8 = 1;

#[derive(Default)]
/**
 * Conatiner format of 31codec encoded pictures.
 *
 * # Data structure
 * |Start|End|Field|
 * |-----|---|-----|
 * |0    |4  |Magic header|
 * |4    |5  |Version|
 * |5    |6  |YUV type|
 * |6    |8  |Width|
 * |8    |10 |Height|
 * |10   |   |Encoded data|
 *
 * ## YUV type
 * |Name|Value|
 * |----|-----|
 * |YUV420P|1 |
*/
pub struct PictureFormat {
    pub pix_fmt: u8,
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

impl PictureFormat {
    pub fn load(bytes: &[u8]) -> Self {
        let pix_fmt = bytes[5];
        let width = u16::from_be_bytes(bytes[6..8].try_into().unwrap());
        let height = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
        Self {
            pix_fmt,
            width,
            height,
            data: bytes[10..].to_vec(),
        }
    }
    pub fn dump(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MAGIC_HEADER);
        bytes.push(VERSION);
        bytes.push(PIX_FMT_YUV420P);
        bytes.extend(self.width.to_be_bytes());
        bytes.extend(self.height.to_be_bytes());
        bytes.extend(&self.data);

        bytes
    }
}
