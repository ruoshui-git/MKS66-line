mod graphics;
use graphics::{PPMImg, RGB};
use env_logger;

fn main()
{
    env_logger::init();

    let mut img = PPMImg::new(1024, 1024, 255);
    img.fg_color = RGB{red: 0, green: 0 , blue: 0};
    img.plot(5, 6);
    // img.draw_line(0.0, 0.0, 50.0, 25.0);
    // img.draw_line(0.0, 0.0, 25.0, 50.0);
    // img.draw_line(10, 450, 480, 30);
    for i in 1..480
    {
        // img.draw_line(10, i, 480, 500 - i);
        img.draw_line(1000, 480 - i + 1, 24, i);
    }
    // img.draw_line(20.0, 49.0, 0.0, 0.0);
    img.fg_color = RGB { red: 255, green: 0, blue: 0 };
    // img.draw_line(1000, 239, 24, 239);
    img.write_ascii("img.ppm").expect("Error writing to file");
}