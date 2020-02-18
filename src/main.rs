mod graphics;
use graphics::{PPMImg, RGB};

fn main() {

    let mut img = PPMImg::new(1024, 1024, 255);

    let mut color = RGB {
        red: 0,
        green: 50,
        blue: 255,
    };
    
    for i in (0..1024).step_by(64)
    {
        img.draw_line(0.0, i as f64, 1023.0, 1023.0 - i as f64);
        color.blue -= 10;
        color.red += 10;
        img.fg_color = color;
        img.draw_line(i as f64, 0.0, 1023.0 - i as f64, 1023.0);
    }

    img.draw_line(0.0, 1023.0, 1023.0, 0.0);

    let mut turtle = img.new_turtle_at(512.0, 512.0);
    turtle.angle_deg = 180.0;
    turtle.set_color(RGB {
        red: 255,
        blue: 0,
        green: 0,
    });
    turtle.pen_down = true;

    let factor = 300.0;

    for a in 1..5000 {
        turtle.forward(
            (a as f64 / factor / ((a as f64 % (std::f64::consts::PI * 2.0)).sin() + 1.5).round())
                as i32,
        );
        turtle.turn_rt(1.0);
    }

    turtle.pen_down = false;
    turtle.move_to(512.0, 512.0);
    turtle.pen_down = true;
    turtle.set_color(RGB{red: 0, blue: 255, green: 255});

    for a in 1..5000 {
        turtle.forward(
            (a as f64 / factor / ((a as f64 % (std::f64::consts::PI * 2.0)).cos() + 1.5).round())
                as i32,
        );
        turtle.turn_rt(1.0);
    }

    let img = turtle.get_ppm_img();

    img.write_ascii("img.ppm").expect("Error writing to file");
}



// if false
// {
//     for n in 3..5
//     {
//         for _ in 0..n
//         {
//             if n == 2 { break; }
//             turtle.forward(100);
//             turtle.turn_rt(360.0 / n as f64);
//             let mut color = turtle.get_color();
//             color.red += 50;
//             turtle.set_color(color);
//             debug!("angle: {}", turtle.angle_deg);
//         }
//     }
// }