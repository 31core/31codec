pub enum FrameType {
    IFrame,
    PFRame,
}

pub struct Frame {
    pub r#type: FrameType,
    pub frame_data: Vec<u8>,
}

impl Frame {
    pub fn new(bytes: &[u8], frame_type: FrameType) -> Self {
        Self {
            r#type: frame_type,
            frame_data: bytes.to_vec(),
        }
    }
}

#[derive(Default)]
pub struct Video {
    pub frames: Vec<Frame>,
    pub resolution: (usize, usize),
}

impl Video {
    /** dump frames into bytes */
    pub fn dump(&self) -> Vec<u8> {
        let mut offset_area: Vec<u8> = Vec::new();
        let mut data_area: Vec<u8> = Vec::new();
        for frame in &self.frames {
            let size = frame.frame_data.len();
            /* every offest takes 3 bytes to store */
            offset_area.extend(&(size as u32).to_be_bytes()[1..4]);
            data_area.extend(&frame.frame_data);
        }

        let mut data = Vec::new();
        data.extend((self.frames.len() as u32).to_be_bytes());
        data.extend(offset_area);
        data.extend(data_area);
        data
    }
    /** load frames from bytes */
    pub fn load(bytes: &[u8]) -> Self {
        let mut video = Self::default();
        let frame_amount = u32::from_be_bytes({
            let mut bytes_u32 = [0; 4];
            bytes_u32.copy_from_slice(&bytes[0..4]);
            bytes_u32
        }) as usize;

        let offset_area = &bytes[4..];
        let data_area = &bytes[4 + 3 * frame_amount..];
        let mut offset = 0;
        for frame in 0..frame_amount {
            let size = u32::from_be_bytes({
                let mut bytes_u32 = [0; 4];
                bytes_u32[1..].copy_from_slice(&offset_area[(3 * frame)..(3 * frame + 3)]);
                bytes_u32
            }) as usize;
            video.frames.push(Frame::new(
                &data_area[offset..offset + size],
                FrameType::IFrame,
            ));
            offset += size;
        }
        video
    }
    pub fn get_frame(&self, frame: usize) -> &[u8] {
        &self.frames[frame].frame_data
    }
}
