mod graphics;
use graphics::PPMImg;

fn main()
{
    let mut img = PPMImg::new(512, 512, 255);
    img.plot(5, 6);
    img.draw_line(0.0, 100.0, 100.0, 50.0);

    img.write_binary("img.ppm").expect("Error writing to file");
}