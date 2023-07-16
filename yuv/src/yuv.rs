#[derive(Default, Debug, Clone)]
pub struct YUV420 {
    pub y: [u8; 4],
    pub cb: u8,
    pub cr: u8,
}

pub fn yuv_to_rgb(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    fn clamp(value: f32) -> f32 {
        if value < 0. {
            0.
        } else if value > 1. {
            1.
        } else {
            value
        }
    }
    let y = y as f32 / 255.;
    let u = (u as f32 - 128.) / 255.;
    let v = (v as f32 - 128.) / 255.;

    let r = clamp(y + 1.402 * v) * 255.;
    let g = clamp(y - 0.344136 * u - 0.714136 * v) * 255.;
    let b = clamp(y + 1.772 * u) * 255.;

    (r as u8, g as u8, b as u8)
}

pub struct YUV420Frame {
    width: usize,
    #[allow(dead_code)]
    height: usize,
    pub points: Vec<YUV420>,
}

impl YUV420Frame {
    pub fn parse_yuv420p(bytes: &[u8], width: usize, height: usize) -> Self {
        let w = width / 2;
        let h = height / 2;
        let mut pixels: Vec<YUV420> = Vec::new();

        let cb_area = &bytes[4 * w * h..];
        let cr_area = &bytes[5 * w * h..];
        for y in 0..h {
            for x in 0..w {
                let mut pixel = YUV420::default();
                pixel.y[0] = bytes[4 * y * w + 2 * x];
                pixel.y[1] = bytes[4 * y * w + 2 * x + 1];
                pixel.y[2] = bytes[4 * y * w + 2 * w + 2 * x];
                pixel.y[3] = bytes[4 * y * w + 2 * w + 2 * x + 1];

                pixel.cb = cb_area[y * w + x];
                pixel.cr = cr_area[y * w + x];
                pixels.push(pixel);
            }
        }
        Self {
            width,
            height,
            points: pixels,
        }
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> YUV420 {
        self.points[(y / 2) * (self.width / 2) + x / 2].clone()
    }
}
