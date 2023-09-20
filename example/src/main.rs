#![no_std]
#![no_main]

use pros::prelude::*;

struct ExampleRobot;
impl Robot for ExampleRobot {
    fn opcontrol() {
        // Create a new motor plugged into port 2. The motor will brake when not moving.
        let motor = Motor::new(2, pros::motor::BrakeMode::Brake).unwrap();
        // Create a controller, specifically controller 1.
        let controller = Controller::new(pros::controller::ControllerId::Master);

        let mut vision =
            sensors::vision::VisionSensor::new(9, sensors::vision::VisionZeroPoint::Center)
                .unwrap();
        vision.set_led(sensors::vision::LedMode::On(sensors::vision::Rgb::new(
            0, 0, 255,
        )));

        pros::lcd::buttons::register_button_callback(
            left_button_callback,
            pros::lcd::buttons::Button::Left,
        )
        .unwrap();

        // Spawn a new task that will print whether or not the motor is stopped constantly.
        pros::multitasking::TaskBuilder::new({
            // Clone the motor to be used in the task.
            let motor = motor.clone();

            move || loop {
                println!("Motor stopped? {}", motor.get_state().stopped);

                // Sleep the task as to not steal processing time from the OS.
                // This should always be done in any loop, including loops in the main task.
                pros::multitasking::sleep(core::time::Duration::from_millis(20));
            }
        })
        .name("Print Task")
        .build();

        loop {
            // Set the motors output with how far up or down the right joystick is pushed.
            // Set output takes a float from -1 to 1 that is scaled to -12 to 12 volts.
            motor.set_output(controller.state().joysticks.right.y);

            // println!("pid out {}", pid.update(10.0, motor.position().into_degrees() as f32));
            println!(
                "Vision objs {}",
                vision.nth_largest_object(0).unwrap().middle_x
            );

            // Once again, sleep.
            pros::multitasking::sleep(core::time::Duration::from_millis(20));
        }
    }
}
robot!(ExampleRobot);

fn left_button_callback() {
    println!("Left button pressed!");
}
