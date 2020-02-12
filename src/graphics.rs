mod graphics {

    #[derive(Copy, Clone)]
    struct RGBTriple {
        red: u16,
        blue: u16,
        green: u16,
    }
    pub mod ppm_img {
        
        use std::convert::TryInto;
        use std::fs::File;
        use std::io::{self, prelude::*, BufWriter};
        use std::path::Path;
        use super::*;

        pub struct PPMImg {
            height: u32,
            width: u32,
            depth: u16, // max = 2^16
            fg_color: RGBTriple,
            bg_color: RGBTriple,
            data: Vec<RGBTriple>,
        }

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
                        red: 0,
                        green: 0,
                        blue: 0
                    };
                    (width * height).try_into().unwrap()
                ],
            }
        }

        fn create_file(filepath: &str) -> BufWriter<File> {
            let path = Path::new(filepath);
            let display = path.display();
            match File::create(&path) {
                Err(why) => panic!("Could not create {}: {}", display, why),
                Ok(file) => BufWriter::new(file),
            }
        }

        impl PPMImg {
            fn index(&self, x: u32, y: u32) -> usize {
                (x * self.width as u32 + y).try_into().unwrap()
            }

            pub fn plot(&mut self, x: u32, y: u32) -> () {
                let index = self.index(x, y);
                self.data[index] = self.fg_color;
            }

            pub fn write_binary(&self, filepath: &str) -> io::Result<()> {
                let mut file = create_file(filepath);
                writeln!(file, "P6")?;
                writeln!(file, "{} {} {}", self.width, self.height, self.depth);
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
                file.flush();
                Ok(())
            }
        }
    }
}
