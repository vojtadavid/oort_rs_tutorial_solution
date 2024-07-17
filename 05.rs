// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// This is where the game becomes challenging! You'll need to lead the target
// by firing towards where the target will be by the time the bullet gets there.
//
// Hint: target() + target_velocity() * t gives the position of the target after t seconds.
//
// You can scale a vector by a number: vec2(a, b) * c == vec2(a * c, b * c)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn rotate(&mut self, a : f64) {
        let t = (f64::signum(a)*angular_velocity() + max_angular_acceleration()*TICK_LENGTH)/max_angular_acceleration();
        let s = (f64::signum(a)*angular_velocity() + max_angular_acceleration()*TICK_LENGTH)*t - 0.5*max_angular_acceleration()*t*t;
        debug!("t {}",t);
        debug!("s {}",s);

        torque(f64::signum(a)*max_angular_acceleration());

        if(f64::abs(s) > f64::abs(a) && f64::abs(angular_velocity())>0.0 ){
            debug!("slow!");
           torque(-f64::signum(a)*max_angular_acceleration());
        }


        if(f64::abs(a)<0.1){
            debug!("stop");
            turn(f64::signum(a) * std::f64::consts::PI/10.0);

            if f64::abs(a) < 0.01 {
                fire(0);
            }
        }

    }

    pub fn tick(&mut self) {
        let dp = target() - position();
        debug!("distance to target: {}", dp.length());
        debug!("time to target: {}", dp.length() / BULLET_SPEED);

        let mut t : i32 = 0;
        let mut diff : f64 = 1000.0;
        let mut tp = vec2(0.0, 0.0);
        while t < 1000 {
            tp = target() + target_velocity()*TICK_LENGTH*f64::from(t);
            let timeTT = (tp-position()).length() / BULLET_SPEED;

            let d = f64::abs(timeTT - f64::from(t)/60.0);
            if(d < diff){
                diff = d;
            } else {
                break;
            }

            t = t+1;
        }

        draw_line(position(), target()+target_velocity()*TICK_LENGTH*f64::from(t), 0x00ff00);

        let a = angle_diff(heading(), (tp - position()).angle());

        self.rotate(a);

    }
}
