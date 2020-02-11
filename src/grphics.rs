mod graphics
{
    use std::fs::File;
    use std::io::{self, prelude::*, BufWriter};
    use std::path::Path;


    #[derive(Copy, Clone)]
    struct RGBTriple
    {
        red: f64,
        blue: f64,
        green: f64,
    };

    pub struct PPMImg
    {
        height: u32,
        width: u32,
        depth: u16, // max = 2^16
        fg_color: RGBTriple,
        bg_color: RGBTriple,
        data: Vec<RGBTriple>,
    };

    impl PPMImg
    {
        
        fn index(&self, x: u32, y:u32) -> u32
        {
            x * self.width + y
        }

        pub fn new(height, width, depth)-> Self
        {
            PPMImg
            {
                height,
                width,
                depth,
                fg_color = RGBTriple
                {
                    red = depth,
                    green = depth,
                    blue = depth,
                }
                bg_color = RGBTriple
                {
                    red = 0,
                    green = 0,
                    blue = 0,
                }
                data = vec![PPMImg{ 0, 0, 0 }; width * height],
            }
        }

        pub fn plot(&self, x:u32, y:u32) -> ()
        {
            self.data[self.index(x, y)] = self.fg_color;
        }

        fn create_file(filepath: &str) -> BufWriter<File>
        {
            let path = Path::new(str);
            let display = path.display();
            let mut file = match File::create(&path)
            {
                Err(why) => panic!("Could not create {}: {}", display, why)
                Ok(file) => BufWriter::new(file),
            }
        }

        pub fn write_binary(&self, filepath: &str) -> io::Result
        {
            let mut file = create_file(filepath);
            writeln!(file, "P6")?;
            writeln!(file, "{} {} {}", self.width, self.height, self.depth);
            if self.depth < 256
            {
                for t in self.data.iter()
                {
                    file.write(t.red as u8);
                    file.write(t.green as u8);
                    file.write(t.blue as u8);
                }
            }
            else
            {

            }
        }
    }
}
