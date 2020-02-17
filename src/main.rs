// use std::env::args;

mod graphics;
use graphics::{PPMImg, RGB};
use log::debug;
use env_logger;


fn main()
{

    
    env_logger::init();

    debug!("debug enabled!");

    let mut img = PPMImg::new(1024, 1024, 255);
    img.fg_color = RGB{red: 0, green: 0 , blue: 0};
    img.plot(5, 6);
    img.draw_line(300.0, 1024.0, 724.0, 0.0);
    // img.draw_line(0.0, 0.0, 25.0, 50.0);
    // img.draw_line(10, 450, 480, 30);
    // for i in 1..1024
    // {
    //     // img.draw_line(10, i, 480, 500 - i);
    //     img.draw_line(1000, 1024 - i + 1, 24, i);
    // }.
    // img.draw_line(200, 49.0, 0.0, 0.0);
    
    let mut turtle = img.new_turtle_at(500, 800);
    turtle.angle_deg = 90.0;
    turtle.set_color(RGB{red: 0, blue: 0, green: 0});
    turtle.pen_down = true;

    for n in 19..20
    {
        for _ in 0..n
        {
            turtle.forward(100);
            turtle.turn(360.0 / n as f64);
            // let mut color = turtle.color();
            // color.blue += 50;
            // turtle.set_color(color);
            debug!("angle: {}", turtle.angle_deg);
        }

    }
    // for _ in 0..180
    // {
    //     turtle.forward(5);
    //     turtle.turn(-2.0);
    //     println!("position: {} {}, angle: {}", turtle.x, turtle.y, turtle.angle_deg);
    // }

    let img = turtle.get_ppm_img();

    img.write_ascii("img.ppm").expect("Error writing to file");
}