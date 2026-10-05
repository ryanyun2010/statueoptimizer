mod vec3;
mod statue_sim;
use crate::vec3::Vec3;
use crate::statue_sim::StatueParams;



fn main(){
    let statue_params = StatueParams {
        h1: 0.035,
        h2: 0.225,
        s: 0.016,
        w: 0.009,
        r: 0.050,
        density: 1247.04,
        rho: 12.0
    };
    let mut best_torque = f32::MIN;
    for theta in (0..=180 * 100).step_by(1) {
        for phi in (-(90 * 100)..=100 * 100).step_by(1){
            let theta_rad = (theta as f32/100.0).to_radians();
            let phi_rad = (phi as f32/100.0).to_radians();
            let torque = statue_params.total_torque(theta_rad, phi_rad).to_tuple();
            if torque.0 < 0.0 || torque.1 < 0.0 {
                continue;
            }
            if torque.2.abs() > best_torque {
                println!("New best torque: {:?}, Theta: {}, Phi: {}", torque, theta as f32 /100.0, phi as f32/100.0);
                best_torque = torque.2.abs();
            }
        }
    }

}
