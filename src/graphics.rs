use std::convert::Into;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct RGB {
    pub red: u16,
    pub blue: u16,
    pub green: u16,
}

use std::convert::TryInto;
use std::fs::File;
use std::io::{self, prelude::*, BufWriter};
use std::path::Path;

pub struct PPMImg {
    height: u32,
    width: u32,
    depth: u16, // max = 2^16
    pub fg_color: RGB,
    pub bg_color: RGB,
    data: Vec<RGB>,
}

fn create_file(filepath: &str) -> BufWriter<File> {
    let path = Path::new(filepath);
    let display = path.display();
    match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => BufWriter::new(file),
    }
}

fn polar_to_xy(mag: f64, angle_degrees: f64) -> (f64, f64) {
    let (dy, dx) = angle_degrees.to_radians().sin_cos();
    (dx * mag, dy * mag)
}

// impl constructor and exporter
#[allow(dead_code)]
impl PPMImg {
    /// Createa new PPMImg
    /// Default fg color is white, bg_color is lack
    pub fn new(height: u32, width: u32, depth: u16) -> PPMImg {
        PPMImg {
            height,
            width,
            depth,
            fg_color: RGB {
                red: depth,
                green: depth,
                blue: depth,
            },
            bg_color: RGB {
                red: 0,
                green: 0,
                blue: 0,
            },
            data: vec![
                RGB {
                    red: depth,
                    green: depth,
                    blue: depth,
                };
                (width * height).try_into().unwrap()
            ],
        }
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

// implement point plotting
impl PPMImg {
    pub fn plot(&mut self, x: i32, y: i32) -> () {
        if x < 0
            || y < 0
            || x >= (self.width).try_into().unwrap()
            || y >= (self.height).try_into().unwrap()
        {
            return ();
        }
        // now we know that x and y are positive, we can cast without worry
        let index = self.index(x as u32, y as u32);
        self.data[index] = self.fg_color;
    }
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width as u32 + x).try_into().unwrap()
    }
}

// impl line algorithm
#[allow(dead_code)]
impl PPMImg {
    /// Draw a line from (x0, y0) to (x1, y1)
    /// #### impl note:
    ///    Always add 2A or 2B when updating D. Half of that value will distort line
    pub fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        // swap variables if needed, since we are always going from left to right
        let (x0, y0, x1, y1) = if x0 > x1 {
            (x1, y1, x0, y0)
        } else {
            (x0, y0, x1, y1)
        };

        // force conversion into ints for processing & plotting
        let (x0, y0, x1, y1) = (x0.round() as i32, y0.round() as i32, x1.round() as i32, y1.round() as i32);

        // calculate  values and then truncate
        let (dy, ndx) = (y1 - y0, -(x1 - x0));

        // deal with special cases:
        if ndx == 0 {
            // vertical line
            let (y0, y1) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

            for y in y0..=y1 {
                self.plot(x0, y);
            }

            return ();
        }

        if dy == 0 {
            // horizontal line
            // x vals are already in the right order, so we don't flip
            for x in x0..=x1 {
                self.plot(x, y0);
            }
            return ();
        }

        // find A and B
        // let m  = -dely as f64 / ndelx as f64;

        let (x, mut y) = (x0, y0);

        if (y1 - y0).abs() < (x1 - x0).abs() {
            // octant 1 and 8
            let mut d = 2 * dy + ndx;
            let (y_inc, dy) = if dy > 0 {
                // octant 1
                (1, dy)
            } else {
                // octant 8
                // dy is (-) in octant 8, so flip it to balance out with ndx
                (-1, -dy)
            };

            for x in x0..=x1 {
                self.plot(x, y);
                if d > 0 {
                    y += y_inc;
                    d += 2 * ndx;
                }
                d += 2 * dy;
            }
        } else {
            // octant 2 and 7
            // flipping x and y should work out

            let mut d = 2 * -ndx - dy;

            let (x_inc, mut x, ystart, yend, dy) = 
            if dy > 0 {
                // octant 2
                (1, x, y0, y1, dy)
            } else {
                // octant 7
                // swap -x and y to reflect over y=-x into octant 8
                (-1, x - ndx, y1, y0, -dy)
            };

            for y in ystart..=yend
            {
                self.plot(x, y);
                if d > 0
                {
                    x += x_inc;
                    d -= 2 * dy;
                }
                d -= 2 * ndx;
            }
            
        }
    }

    /// Draw a line from (x0, y0) with a certain magnitude and angle
    /// ## Note
    /// Angle goes counter clockwise from x axis.
    ///
    /// Returns the other endpoint of the line (x1, y1) as a tuple
    pub fn draw_line_degrees(
        &mut self,
        x0: f64,
        y0: f64,
        angle_degrees: f64,
        mag: f64,
    ) -> (f64, f64) {
        let (dx, dy) = polar_to_xy(mag, angle_degrees);
        let (x1, y1) = (x0 + dx, y0 + dy);

        self.draw_line(x0, y0, x1, y1);
        return (x1, y1);
    }
}

pub struct Turtle {
    x: f64,
    y: f64,
    pub angle_deg: f64,
    pub pen_down: bool,
    img: PPMImg,
}

// impl turtle on Img
impl PPMImg {
    /// Creates a turtle for PPMImg
    /// ## Warning
    /// Img will move into a Turtle, so any new bindings to the current instance of PPMImg will be invalid.
    ///
    /// And therefore only one Turtle is allowed at a time for an Img.
    pub fn new_turtle_at(self, x: f64, y: f64) -> Turtle {
        Turtle {
            x,
            y,
            angle_deg: 0.0,
            pen_down: false,
            img: self,
        }
    }
}

#[allow(dead_code)]
impl Turtle {
    pub fn forward(&mut self, steps: i32) {
        let (x0, y0) = (self.x, self.y);
        let (dx, dy) = polar_to_xy(steps.into(), self.angle_deg);
        let (x1, y1) = (x0 as f64 + dx, y0 as f64 + dy);
        if self.pen_down {
            self.img.draw_line(x0 as f64, y0 as f64, x1, y1);
        }
        self.x = x1;
        self.y = y1;
    }

    pub fn turn_rt(&mut self, angle_deg: f64) {
        self.angle_deg = (self.angle_deg + angle_deg) % 360.0;
    }

    pub fn set_color(&mut self, rgb: RGB) {
        self.img.fg_color = rgb;
    }

    pub fn get_color(&self) -> RGB {
        return self.img.fg_color;
    }

    pub fn move_to(&mut self, x: f64, y:f64)
    {
        if self.pen_down
        {
            self.img.draw_line(self.x as f64, self.y as f64, x, y);
        }
        self.x = x;
        self.y = y;
    }

    /// Get the inner PPMImg instance
    ///
    /// This method will move the turtle
    pub fn get_ppm_img(self) -> PPMImg {
        return self.img;
    }
}
