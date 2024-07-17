// Tutorial: Rotation
// Destroy the asteroid. The target is in a random
// location given by the "target()" function.
//
// You can get the angle between a vector and the x-axis: vec2(a, b).angle()
// And compare an angle with your ship's heading: angle_diff(heading(), angle)
//
// If angle_diff returns a positive number you need to turn left, or if negative then right.
// The turn function takes a speed argument, where positive speeds result in turning left
// and negative speeds will turn right.
use oort_api::prelude::*;
use std::f64;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        // Hint: "angle_diff(heading(), (target() - position()).angle())"
        // returns the direction your ship needs to turn to face the target.
        // turn(1.0);
        // fire(0);
        debug!("max angular rotation {}",max_angular_acceleration());
        debug!("angular vel {}",angular_velocity());
        debug!("heading {}",heading());
        let a = angle_diff(heading(), (target() - position()).angle());
        debug!("a {}",a);

        debug!("tick length speed {}",TICK_LENGTH);

        // if(angular_velocity()<0.1 && f64::abs(a)<0.1  ){
        //     fire(0);
        //     return;
        // }



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
            turn(f64::signum(a) * f64::consts::PI/10.0);
            fire(0);
        }

        
        
    }
}
