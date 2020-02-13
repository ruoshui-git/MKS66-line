#[allow(dead_code)]
#[derive(Copy, Clone)]
struct RGBTriple {
    red: u16,
    blue: u16,
    green: u16,
}

use std::convert::TryInto;
use std::fs::File;
use std::io::{self, prelude::*, BufWriter};
use std::path::Path;

#[allow(dead_code)]
pub struct PPMImg {
    height: u32,
    width: u32,
    depth: u16, // max = 2^16
    fg_color: RGBTriple,
    bg_color: RGBTriple,
    data: Vec<RGBTriple>,
}

#[allow(dead_code)]
fn create_file(filepath: &str) -> BufWriter<File> {
    let path = Path::new(filepath);
    let display = path.display();
    match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => BufWriter::new(file),
    }
}

#[allow(dead_code)]
impl PPMImg {
    pub fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> () {
        // swap variables if needed
        let (x0, y0, x1, y1) = if x0 > x1 {
            (x1, y1, x0, y0)
        } else {
            (x0, y0, x1, y1)
        };

        let (x0, y0, x1, y1) = (x0 as i32, y0 as i32, x1 as i32, y1 as i32);
        let (mut x, mut y) = (x0, y0);

        // find A and B
        let (dy, ndx) = (y1 - y0, -(x1 - x0));
        // 1st octant
        let mut d = 2 * dy + ndx;
        while x < x1 {
            self.plot(x, y);
            if d > 0
            {
                y = y + 1;
                d = d + ndx;
            }
            x = x + 1;
            d = d + dy;
        }
    }

    pub fn plot(&mut self, x: i32, y: i32) -> () {
        if x < 0 || y < 0 {
            return ();
        }
        // now we know that x and y are positive, we can cast without worry
        let index = self.index(x as u32, y as u32);
        self.data[index] = self.fg_color;
    }

    /// Createa new PPMImg
    /// Default fg color is white, bg_color is lack
    pub fn new(height: u32, width: u32, depth: u16) -> PPMImg {
        PPMImg {
            height,
            width,
            depth,
            fg_color: RGBTriple {
                red: depth,
                green: depth,
                blue: depth,
            },
            bg_color: RGBTriple {
                red: 0,
                green: 0,
                blue: 0,
            },
            data: vec![
                RGBTriple {
                    red: depth,
                    green: depth,
                    blue: depth,
                };
                (width * height).try_into().unwrap()
            ],
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (x * self.width as u32 + y).try_into().unwrap()
    }

    pub fn write_binary(&self, filepath: &str) -> io::Result<()> {
        let mut file = create_file(filepath);
        writeln!(file, "P6")?;
        writeln!(file, "{} {} {}", self.width, self.height, self.depth)?;
        if self.depth < 256 {
            for t in self.data.iter() {
                file.write(&[t.green as u8])?;
                file.write(&[t.green as u8])?;
                file.write(&[t.blue as u8])?;
            }
        } else {
            for t in self.data.iter() {
                file.write_all(&(t.red.to_be_bytes()))?;
                file.write_all(&(t.green.to_be_bytes()))?;
                file.write_all(&(t.blue.to_be_bytes()))?;
            }
        }

        file.flush()?;
        Ok(())
    }
    pub fn write_ascii(&self, filepath: &str) -> io::Result<()> {
        let mut file = create_file(filepath);
        writeln!(file, "P3")?;
        writeln!(file, "{} {} {}", self.width, self.height, self.depth)?;
        for t in self.data.iter() {
            writeln!(file, "{} {} {}", t.red, t.green, t.blue)?;
        }
        file.flush()?;
        Ok(())
    }
}
