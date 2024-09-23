use motor_controller::motor::{setup_motors, DualPwm, PwmDir};
use motor_controller::udp_communication;
use rppal::gpio::{Gpio, InputPin, Level};
use std::thread;
use std::thread::sleep;

const FREQUENCY: f64 = 1000.;
const START_DUTY_CYCLE: f64 = 0.0;
use std::time::Duration;

fn main() {
    let pwm_dir_motors = vec![
        PwmDir::new( 17,18, START_DUTY_CYCLE, FREQUENCY),
        PwmDir::new( 27,22, START_DUTY_CYCLE, FREQUENCY),
        PwmDir::new( 23,24, START_DUTY_CYCLE, FREQUENCY),
        PwmDir::new( 25,32, START_DUTY_CYCLE, FREQUENCY),
        PwmDir::new( 16,26, START_DUTY_CYCLE, FREQUENCY),
        PwmDir::new( 13,2 , START_DUTY_CYCLE, FREQUENCY),

        PwmDir::new( 3,4 , START_DUTY_CYCLE, FREQUENCY), // 6

    ];

    let motors = setup_motors(pwm_dir_motors);

    let mut udp_communication = motors.clone();
    thread::spawn(move || loop {
        udp_communication::recv_pwm_udp(&mut udp_communication, "60000");
    });

    let gpio = Gpio::new().unwrap();
    let limit_switch_pin = [
        gpio.get(8).unwrap().into_input_pullup(),
        gpio.get(9).unwrap().into_input_pullup(),
    ];

    let mut state = 0;
    let mut state1 = 0;
    loop {
        if limit_switch_pin[0].is_low() {
            // スイッチが押されているか確認
            *motors[6].lock().unwrap() = 0.;
            sleep(Duration::from_millis(10));
            *motors[6].lock().unwrap() = 1.;
            state = 1;
            state1 = 0;
            println!("0")
        }

        if limit_switch_pin[1].is_low() {
            state1 = 1;
            println!("2");
        }
        if limit_switch_pin[1].is_high() && state1 >= 1 && state >= 1 {
            println!("{} {}", state1, state);
            *motors[6].lock().unwrap() = 0.;

            state1 = 0;
            state = 0;
        }

        // 状態の更新を待機
        sleep(Duration::from_millis(1));
    }
}
