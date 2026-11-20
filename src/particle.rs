// This is depricated

#[derive(Debug)]
pub struct  Particle {
    x: f64,
    y: f64,
    z: f64,

    px: f64,
    py: f64,
    pz: f64,

    mass: f64,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            px: 0.0,
            py: 0.0,
            pz: 0.0,
            mass: 1.0,
        }
    }
}
/*
impl  Particle {
    pub fn set_position(&mut self, x: f64, y: f64, z: f64, mass: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.mass = mass;
    }

    pub fn set_momentum(&mut self, px: f64, py: f64, pz: f64, mass: f64) {
        self.mass = mass;
        self.px = px;
        self.py = py;
        self.pz = pz;
    }


    pub fn update(&mut self, x: f64, y: f64, z: f64, px: f64, py: f64, pz: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.px = px;
        self.py = py;
        self.pz = pz;
    }

}*/

impl Particle {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            ..Default::default()
        }
    }

    pub fn update(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        px: f64,
        py: f64,
        pz: f64,
    ) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.px = px;
        self.py = py;
        self.pz = pz;
    }
}