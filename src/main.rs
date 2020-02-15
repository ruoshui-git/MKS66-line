mod graphics;
use graphics::{PPMImg, RGBTriple};

fn main()
{
    let mut img = PPMImg::new(50, 50, 255);
    img.fg_color = RGBTriple{red: 0, green: 0 , blue: 0};
    img.plot(5, 6);
    // img.draw_line(0.0, 0.0, 50.0, 25.0);
    // img.draw_line(0.0, 0.0, 25.0, 50.0);
    img.draw_line(10.0, 49.0, 49.0, 30.0);
    img.draw_line(20.0, 49.0, 0.0, 0.0);
    img.write_ascii("img.ppm").expect("Error writing to file");
}