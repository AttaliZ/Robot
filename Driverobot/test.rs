use rppal::gpio::{Gpio, OutputPin, InputPin};
use rppal::pwm::{Pwm, Channel};
use std::thread::sleep;
use std::time::Duration;

// กำหนดขา GPIO สำหรับมอเตอร์
const PIN_B1: u8 = 16; // GPIO16 -> IN1 (ซ้าย)
const PIN_B2: u8 = 5;  // GPIO5  -> IN2
const PIN_A1: u8 = 4;  // GPIO4  -> IN3 (ขวา)
const PIN_A2: u8 = 0;  // GPIO0  -> IN4

// กำหนดขา GPIO สำหรับเซ็นเซอร์ HC-SR04
const TRIG: u8 = 23;   // GPIO23 -> 
const ECHO: u8 = 24;   // GPIO24 -> 

// กำหนดความถี่ PWM 
const PWM_FREQ: f64 = 1000.0; 

struct Robot {
    a1: OutputPin,
    a2: OutputPin,
    b1: OutputPin,
    b2: OutputPin,
    pwm_left: Pwm,
    pwm_right: Pwm,
    trig: OutputPin,
    echo: InputPin,
}

impl Robot {
    fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            a1: gpio.get(PIN_A1).unwrap().into_output(),
            a2: gpio.get(PIN_A2).unwrap().into_output(),
            b1: gpio.get(PIN_B1).unwrap().into_output(),
            b2: gpio.get(PIN_B2).unwrap().into_output(),
            pwm_left: Pwm::with_frequency(Channel::Pwm0, PWM_FREQ, 0.5, false).unwrap(),
            pwm_right: Pwm::with_frequency(Channel::Pwm1, PWM_FREQ, 0.5, false).unwrap(),
            trig: gpio.get(TRIG).unwrap().into_output(),
            echo: gpio.get(ECHO).unwrap().into_input(),
        }
    }

    fn set_speed(&mut self, duty: f64) {
        self.pwm_left.set_duty_cycle(duty).unwrap();
        self.pwm_right.set_duty_cycle(duty).unwrap();
        self.pwm_left.enable().unwrap();
        self.pwm_right.enable().unwrap();
    }

    fn stop(&mut self) {
        self.a1.set_low();
        self.a2.set_low();
        self.b1.set_low();
        self.b2.set_low();
    }

    fn forward(&mut self, speed: f64) {
        self.set_speed(speed);
        self.a1.set_high();
        self.a2.set_low();
        self.b1.set_high();
        self.b2.set_low();
    }

    fn backward(&mut self, speed: f64) {
        self.set_speed(speed);
        self.a1.set_low();
        self.a2.set_high();
        self.b1.set_low();
        self.b2.set_high();
    }

    fn left(&mut self, speed: f64) {
        self.set_speed(speed);
        self.a1.set_high();
        self.a2.set_low();
        self.b1.set_low();
        self.b2.set_high();
    }

    fn right(&mut self, speed: f64) {
        self.set_speed(speed);
        self.a1.set_low();
        self.a2.set_high();
        self.b1.set_high();
        self.b2.set_low();
    }

    fn measure_distance(&mut self) -> f64 {
        self.trig.set_low();
        sleep(Duration::from_micros(2));
        self.trig.set_high();
        sleep(Duration::from_micros(10));
        self.trig.set_low();

        while self.echo.is_low() {}
        let start = std::time::Instant::now();
        while self.echo.is_high() {}
        let duration = start.elapsed().as_secs_f64();

        let distance = duration * 34300.0 / 2.0;
        distance
    }
}

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
