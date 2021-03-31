//! A system of double compound pendulums that can evolve over time.

#[derive(Copy, Clone, Debug, PartialEq)]
/// A set of two linked compound pendulums.
pub struct DoublePendulum {
    // The angle of the first pendulum, so 0 is straight down and pi/2 is to the right.
    pub a1: f64,
    // The angle of the second pendulum compared to the first.
    pub a2: f64,
    // The generalized angular momentum of the first pendulum.
    p1: f64,
    // The generalized angular momentum of the second pendulum.
    p2: f64,
    // The length of both pendulums.
    pub l: f64,
    // The mass of both pendulums. This is effectively the strength of gravity.
    pub m: f64,
}

impl DoublePendulum {
    /// Make a mew pendulum.
    pub(crate) fn new(a1: f64, a2: f64, p1: f64, p2: f64, l: f64, m: f64) -> Self {
        Self {
            a1,
            a2,
            p1,
            p2,
            l,
            m,
        }
    }

    /// Steps the system forward the given time interval as a discrete step.
    pub(crate) fn step(&mut self, dt: f64) {
        // https://www.wikiwand.com/en/Double_pendulum#/Lagrangian
        let ml2 = self.m * self.l.powi(2);
        let da = self.a1 - self.a2;

        let v1 = (6.0 / ml2) * (2.0 * self.p1 - 3.0 * self.p2 * da.cos())
            / (16.0 - 9.0 * da.cos().powi(2));
        let v2 = (6.0 / ml2) * (8.0 * self.p2 - 3.0 * self.p1 * da.cos())
            / (16.0 - 9.0 * da.cos().powi(2));

        let g = 9.81;
        let d_p1 = -0.5 * ml2 * (v1 * v2 * da.sin() + 3.0 * (g / self.l) * self.a1.sin());
        let d_p2 = -0.5 * ml2 * (-v1 * v2 * da.sin() + (g / self.l) * self.a2.sin());

        self.a1 += v1 * dt;
        self.a2 += v2 * dt;
        self.p1 += d_p1 * dt;
        self.p2 += d_p2 * dt;
    }

    /// Steps forward the given total time using the given number of intermediate increments. This
    /// provides greater accuracy at the cost of computation time.
    pub(crate) fn multi_step(&mut self, dt: f64, num_steps: u64) {
        let sub_dt = dt / (num_steps as f64);
        for _ in 0..num_steps {
            self.step(sub_dt);
        }
    }

    /// Gets the location of the join between the two pendulums in SVG coordinates relative to the
    /// origin of the first pendulum.
    pub(crate) fn get_joint_svg(&self) -> (f64, f64) {
        (self.l * self.a1.sin(), self.l * self.a1.cos())
    }
}
