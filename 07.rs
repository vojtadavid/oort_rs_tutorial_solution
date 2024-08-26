// Tutorial: Radar
// Destroy the enemy ships. Use your radar to find them.
// Hint: Press 'g' in-game to show where your radar is looking.
// Hint: Press 'n' to single-step.
// Hint: Use the set_radar_heading() function to keep your radar pointed at a
// target, or to search for a new one.
//
// Join the Discord at https://discord.gg/vYyu9EhkKH for Oort discussion and
// tournament results.
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

    pub fn shoot(&mut self, myTarget : Vec2, myTargetVel : Vec2) {
        let dp = myTarget - position();
        debug!("distance to target: {}", dp.length());
        debug!("time to target: {}", dp.length() / BULLET_SPEED);

        let mut t : i32 = 0;
        let mut diff : f64 = 1000.0;
        let mut tp = vec2(0.0, 0.0);
        while t < 1000 {
            tp = myTarget + myTargetVel*TICK_LENGTH*f64::from(t);
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

    pub fn tick(&mut self) {


        if(angular_velocity()>max_angular_acceleration()){
            turn(0.0);
            return;
        }

        if let Some(contact) = scan() {
            self.shoot(contact.position,contact.velocity);
            set_radar_heading((contact.position - position()).angle());
        } else {
        set_radar_heading(radar_heading() + radar_width());

        }
    }
}
