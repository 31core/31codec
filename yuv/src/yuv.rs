#[derive(Default, Debug, Clone)]
pub struct YUV420 {
    pub y: [u8; 4],
    pub cb: u8,
    pub cr: u8,
}

pub mod utils {
    pub fn yuv_to_rgb(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
        let y = y as f32 / 255.;
        let u = (u as f32 - 128.) / 255.;
        let v = (v as f32 - 128.) / 255.;

        let r = (y + 1.402 * v).clamp(0., 1.) * 255.;
        let g = (y - 0.344136 * u - 0.714136 * v).clamp(0., 1.) * 255.;
        let b = (y + 1.772 * u).clamp(0., 1.) * 255.;

        (r as u8, g as u8, b as u8)
    }
}

pub trait YUVFrame {
    fn get_pixel(&self, x: usize, y: usize) -> (u8, u8, u8) {
        (
            self.get_pixel_y(x, y),
            self.get_pixel_u(x, y),
            self.get_pixel_v(x, y),
        )
    }
    fn get_pixel_y(&self, x: usize, y: usize) -> u8;
    fn get_pixel_u(&self, x: usize, y: usize) -> u8;
    fn get_pixel_v(&self, x: usize, y: usize) -> u8;
    fn set_pixel(&mut self, x: usize, y: usize, yuv: (u8, u8, u8)) {
        self.set_pixel_y(x, y, yuv.0);
        self.set_pixel_u(x, y, yuv.1);
        self.set_pixel_v(x, y, yuv.2);
    }
    fn set_pixel_y(&mut self, x: usize, y: usize, y_: u8);
    fn set_pixel_u(&mut self, x: usize, y: usize, cb: u8);
    fn set_pixel_v(&mut self, x: usize, y: usize, cr: u8);
    fn get_resolution(&self) -> (usize, usize);
}

#[derive(Clone)]
pub struct YUV420Frame {
    width: usize,
    height: usize,
    pub points: Vec<YUV420>,
}

impl YUV420Frame {
    /** Get a group of 4 yuv pixels */
    fn get_pixel_group(&self, x: usize, y: usize) -> YUV420 {
        self.points[(y / 2) * (self.width / 2) + x / 2].clone()
    }
}

impl YUVFrame for YUV420Frame {
    fn get_pixel(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let group = self.get_pixel_group(x, y);

        let y = if x % 2 == 0 && y % 2 == 0 {
            group.y[0]
        } else if x % 2 == 1 && y % 2 == 0 {
            group.y[1]
        } else if x % 2 == 0 && y % 2 == 1 {
            group.y[2]
        } else {
            group.y[3]
        };

        (y, group.cb, group.cr)
    }
    fn get_pixel_y(&self, x: usize, y: usize) -> u8 {
        let group = self.get_pixel_group(x, y);
        if x % 2 == 0 && y % 2 == 0 {
            group.y[0]
        } else if x % 2 == 1 && y % 2 == 0 {
            group.y[1]
        } else if x % 2 == 0 && y % 2 == 1 {
            group.y[2]
        } else {
            group.y[3]
        }
    }
    fn get_pixel_u(&self, x: usize, y: usize) -> u8 {
        self.get_pixel_group(x, y).cb
    }
    fn get_pixel_v(&self, x: usize, y: usize) -> u8 {
        self.get_pixel_group(x, y).cr
    }
    fn set_pixel_y(&mut self, i: usize, j: usize, y: u8) {
        if i % 2 == 0 && j % 2 == 0 {
            self.points[(j / 2) * (self.width / 2) + i / 2].y[0] = y;
        } else if i % 2 == 1 && j % 2 == 0 {
            self.points[(j / 2) * (self.width / 2) + i / 2].y[1] = y;
        } else if i % 2 == 0 && j % 2 == 1 {
            self.points[(j / 2) * (self.width / 2) + i / 2].y[2] = y;
        } else {
            self.points[(j / 2) * (self.width / 2) + i / 2].y[3] = y;
        };
    }
    fn set_pixel_u(&mut self, i: usize, j: usize, cb: u8) {
        self.points[(j / 2) * (self.width / 2) + i / 2].cb = cb;
    }
    fn set_pixel_v(&mut self, i: usize, j: usize, cr: u8) {
        self.points[(j / 2) * (self.width / 2) + i / 2].cb = cr;
    }
    fn get_resolution(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

/**
 * struct which impletmented this trait can load from bytes and dump to bytes.
*/
pub trait Bitstream {
    fn load(bytes: &[u8], width: usize, height: usize) -> Self;
    fn dump(&self) -> Vec<u8>;
}

impl Bitstream for YUV420Frame {
    fn load(bytes: &[u8], width: usize, height: usize) -> Self {
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
    fn dump(&self) -> Vec<u8> {
        let w = self.width / 2;
        let h = self.height / 2;
        let mut bytes = vec![0; 4 * h * w];
        let mut cb_area = vec![0; h * w];
        let mut cr_area = vec![0; h * w];

        for y in 0..h {
            for x in 0..w {
                let group = self.get_pixel_group(2 * x, 2 * y);
                bytes[4 * y * w + 2 * x] = group.y[0];
                bytes[4 * y * w + 2 * x + 1] = group.y[1];
                bytes[4 * y * w + 2 * w + 2 * x] = group.y[2];
                bytes[4 * y * w + 2 * w + 2 * x + 1] = group.y[3];

                cb_area[y * w + x] = group.cb;
                cr_area[y * w + x] = group.cr;
            }
        }

        bytes.extend(cb_area);
        bytes.extend(cr_area);
        bytes
    }
}
