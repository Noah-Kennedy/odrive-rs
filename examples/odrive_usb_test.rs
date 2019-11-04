use std::env::args;
use std::io::{BufRead, BufReader, stdin};
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

use serialport::SerialPortSettings;

use odrive_rs::ascii::{ODrive};
use odrive_rs::enumerations::AxisState;

fn main() {
    // Get CLI args
    let args: Vec<String> = args().collect();

    // Create serial port settings
    let mut settings = SerialPortSettings::default();

    // ODrive uses 115200 baud
    settings.baud_rate = 115_200;

    // Create serial port
    let serial = serialport::posix::TTYPort::open(Path::new(&args[1]), &settings).expect("Failed to open port");

    // Create odrive connection
    let mut odrive = ODrive::new(serial);

    // STDIN reader
    let mut input_reader = BufReader::new(stdin());

    println!("ODrive USB Test");
    println!("Setting parameters...");

    // In this example we set the same parameters to both motors.
    // You can of course set them different if you want.
    // See the documentation or play around in odrivetool to see the available parameters
    for axis in 0..2 {
        writeln!(odrive, "w axis{}.controller.config.vel_limit 22000.0", axis).unwrap();
        writeln!(odrive, "w axis{}.motor.config.current_lim 11.0", axis).unwrap();
    }

    println!("Ready!");
    println!("Send the character '0' or '1' to calibrate respective motor (you must do this before you can command movement)");
    println!("Send the character 's' to execute test move");
    println!("Send the character 'b' to read bus voltage");
    println!("Send the character 'p' to read motor positions in a 10s loop");
    println!("Type !exit to exit.");

    let mut line = String::with_capacity(4);
    loop {
        input_reader.read_line(&mut line).unwrap();
        let trimmed = line.trim();
        
        if trimmed == "!exit" {
            break
        } else {
            if let Some(first) = trimmed.chars().nth(0) {
                match first {
                    // Run calibration sequence
                    '0' | '1' => {
                        let motor_num = first as u32 - '0' as u32;

                        println!("Axis {}: Requesting state {:?}", first, AxisState::MotorCalibration);
                        odrive.run_state(motor_num as u8, AxisState::MotorCalibration, true).unwrap();

                        println!("Axis {}: Requesting state {:?}", first, AxisState::EncoderOffsetCalibration);
                        odrive.run_state(motor_num as u8, AxisState::EncoderOffsetCalibration, true).unwrap();

                        println!("Axis {}: Requesting state {:?}", first, AxisState::ClosedLoopControl);
                        odrive.run_state(motor_num as u8, AxisState::ClosedLoopControl, false).unwrap();
                    }
                    // Sinusoidal test move
                    's' => {
                        println!("Executing test move");
                        let mut ph: f32 = 0.0;
                        while ph < 6.283_185_5 {
                            let pos_m0 = 20000.0 * ph.cos();
                            let pos_m1 = 20000.0 * ph.sin();

                            odrive.set_position(0, pos_m0, None, None).unwrap();
                            odrive.set_position(1, pos_m1, None, None).unwrap();

                            ph += 0.01;
                            sleep(Duration::from_millis(5));
                        }
                    }
                    // Read bus voltage
                    'b' => {
                        writeln!(odrive, "r vbus_voltage");
                        println!("Vbus voltage: {}", odrive.read_float().unwrap());
                    }
                    // print motor positions in a 10s loop
                    'p' => {
                        let start = Instant::now();
                        while start.elapsed().as_millis() < 10_000 {
                            for axis in 0..2 {
                                writeln!(odrive, "r axis{}.encoder.pos_estimate", axis);
                                print!("{}\t", odrive.read_float().unwrap_or_default());
                            }
                            println!();
                        }
                    }
                    _ => println!("Invalid user input!")
                }
            }
            
            // clear line buffer
            line.clear()
        }
    }
}