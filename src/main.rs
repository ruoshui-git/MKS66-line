mod graphics;
use graphics::PPMImg;

fn main()
{
    let img = PPMImg::new(512, 512, 225);
    img.plot(x: u32, y: u32)
}