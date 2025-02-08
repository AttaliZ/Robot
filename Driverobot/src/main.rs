mod robot;

use robot::Robot;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut bot = Robot::new();
    let speed = 0.7; // 70% ความเร็ว

    loop {
        let distance = bot.measure_distance();
        println!("ระยะทาง: {:.2} cm", distance);

        if distance < 20.0 {
            println!("พบสิ่งกีดขวางถอยหลัง");
            bot.backward(speed);
            sleep(Duration::from_secs(1));
            bot.left(speed);
            sleep(Duration::from_secs(1));
        } else {
            bot.forward(speed);
        }

        sleep(Duration::from_millis(100));
    }
}
