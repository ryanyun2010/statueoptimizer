use crate::Vec3;
pub struct StatueParams {
    pub h1: f32,
    pub h2: f32,
    pub s: f32,
    pub w: f32,
    pub r: f32,
    pub density: f32,
    pub rho: f32
}

impl StatueParams {
    fn base_volume(&self) -> f32 {
        self.w * self.s * self.h1
    }
    fn cylinder_volume(&self) -> f32 {
        std::f32::consts::PI * self.r.powi(2) * self.h2
    }
    fn center_of_mass(&self) -> Vec3 {
        let base_volume = self.base_volume();
        let cylinder_volume = self.cylinder_volume();
        let total_volume = base_volume + cylinder_volume;
        let base_com = self.h1 / 2.0;
        let cylinder_com = self.h1 + (self.h2 / 2.0);
        Vec3::new(0.,0.,(base_com * base_volume + cylinder_com * cylinder_volume) / total_volume)
    }
    fn pivot_point(&self) -> Vec3 {
        Vec3::new(self.w, -self.s, 0.)
    }
    fn application_point(&self) -> Vec3 {
        Vec3::new(0., self.r, self.h1 + self.h2)
    }
    fn cm_lever_arm(&self) -> Vec3 {
        self.center_of_mass() - self.pivot_point()
    }
    fn force_of_gravity(&self) -> Vec3 {
        let total_volume = self.base_volume() + self.cylinder_volume();
        let gravity_mag = total_volume * self.density * 9.81;
        Vec3::new(0., 0., -gravity_mag)
    }
    fn gravity_torque(&self) -> Vec3 {
        self.cm_lever_arm().cross(&self.force_of_gravity())
    }
    fn ap_lever_arm(&self) -> Vec3 {
        self.application_point() - self.pivot_point()
    }
    fn rope_force(&self, theta: f32, phi: f32) -> Vec3 {
        Vec3::new(
            self.rho * theta.sin() * phi.cos(),
            self.rho * theta.sin() * phi.sin(),
            self.rho * theta.cos()
        )
    }
    fn rope_torque(&self, theta: f32, phi: f32) -> Vec3 {
        self.ap_lever_arm().cross(&self.rope_force(theta, phi))
    }
    pub fn total_torque(&self, theta: f32, phi: f32) -> Vec3 {
        self.gravity_torque() + self.rope_torque(theta, phi)
    }
}
