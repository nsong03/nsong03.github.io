extern crate minifb;

use minifb::{Key, Window, WindowOptions};

struct DoublePendulum {
    // Lengths of the pendulum arms
    l1: f32,
    l2: f32,
    // Masses of the pendulum bobs
    m1: f32,
    m2: f32,
    // Current angles and angular velocities
    theta1: f32,
    theta2: f32,
    omega1: f32,
    omega2: f32,
}

impl DoublePendulum {
    // Update the state of the double pendulum using Verlet integration
    fn update(&mut self, dt: f32) {
        let g = 9.81;  // acceleration due to gravity

        let theta1 = self.theta1;
        let theta2 = self.theta2;
        let omega1 = self.omega1;
        let omega2 = self.omega2;

        let m1 = self.m1;
        let m2 = self.m2;
        let l1 = self.l1;
        let l2 = self.l2;

        let theta1_acc = (-g * (2. * m1 + m2) * theta1.sin()
            - m2 * g * (theta1 - 2. * theta2).sin()
            - 2. * (theta1 - theta2).sin() * m2
                * (omega2.powi(2) * l2 + omega1.powi(2) * l1 * (theta1 - theta2).cos()))
            / (l1 * (2. * m1 + m2 - m2 * (2. * (theta1 - theta2).cos().powi(2))));
        
        let theta2_acc = (2. * (theta1 - theta2).sin()
            * (omega1.powi(2) * l1 * (2. * m1 + m2) + g * (2. * m1 + m2) * theta1.cos()
                + omega2.powi(2) * l2 * m2 * (theta1 - theta2).cos()))
            / (l2 * (2. * m1 + m2 - m2 * (2. * (theta1 - theta2).cos().powi(2))));

        // Update angles and angular velocities
        self.theta1 += dt * omega1 + 0.5 * dt * dt * theta1_acc;
        self.theta2 += dt * omega2 + 0.5 * dt * dt * theta2_acc;
        self.omega1 += dt * theta1_acc;
        self.omega2 += dt * theta2_acc;
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Double Pendulum - Press Esc to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Initialize the double pendulum
    let mut double_pendulum = DoublePendulum {
        l1: 100.0,
        l2: 100.0,
        m1: 10.0,
        m2: 10.0,
        theta1: std::f32::consts::PI / 2.0,
        theta2: std::f32::consts::PI / 2.0,
        omega1: 0.0,
        omega2: 0.0,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for pixel in buffer.iter_mut() {
            *pixel = 0;
        }

        // Update the double pendulum state
        double_pendulum.update(0.01);

        // Calculate the positions of the pendulum bobs
        let x1 = (double_pendulum.l1 * double_pendulum.theta1.cos()) as i32 + WIDTH as i32 / 2;
        let y1 = (double_pendulum.l1 * double_pendulum.theta1.sin()) as i32 + HEIGHT as i32 / 2;
        let x2 = x1 + (double_pendulum.l2 * double_pendulum.theta2.cos()) as i32;
        let y2 = y1 + (double_pendulum.l2 * double_pendulum.theta2.sin()) as i32;

        // Draw the pendulum bobs as red pixels
        buffer[x1 + y1 * WIDTH] = 0xFF0000FF;
        buffer[x2 + y2 * WIDTH] = 0xFF0000FF;

        // Display the buffer
        window.update_with_buffer(&buffer).unwrap();
    }
}
