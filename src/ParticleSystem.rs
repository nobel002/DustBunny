pub struct ParticleSystem {
    qx: Vec<f64>,
    qy: Vec<f64>,
    qz: Vec<f64>,

    px: Vec<f64>,
    py: Vec<f64>,
    pz: Vec<f64>,

    m: Vec<f64>,
}

#[allow(non_snake_case)]
impl ParticleSystem {
    pub fn from_initial_state(
        qx: Vec<f64>,
        qy: Vec<f64>,
        qz: Vec<f64>,
        px: Vec<f64>,
        py: Vec<f64>,
        pz: Vec<f64>,
        m: Vec<f64>,
    ) -> Self {
        let n = qx.len();
        assert_eq!(qy.len(), n);
        assert_eq!(qz.len(), n);
        assert_eq!(px.len(), n);
        assert_eq!(py.len(), n);
        assert_eq!(pz.len(), n);
        assert_eq!(m.len(), n);

        Self {
            qx,
            qy,
            qz,
            px,
            py,
            pz,
            m,
        }
    }

    // Settings for the potential
    // These ones should create a 'hydrogen like atom' with an r0 of 1 unit
    const K: f64 = -40.0;
    const G: f64 = 20.;

    // We'll set this one in stone
    //const N:i32 = 2;

    fn dH_dp(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        // Update according to the Hamilton

        let n = self.qx.len();

        let mut vx = vec![0.0; n];
        let mut vy = vec![0.0; n];
        let mut vz = vec![0.0; n];

        for i in 0..n {
            let inv_m = 1.0 / self.m[i];
            vx[i] = self.px[i] * inv_m;
            vy[i] = self.py[i] * inv_m;
            vz[i] = self.pz[i] * inv_m;
        }

        (vx, vy, vz)
    }

    fn dH_dq(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        // Update according to the Hamilton
        let n = self.qx.len();

        let mut fx = vec![0.0; n];
        let mut fy = vec![0.0; n];
        let mut fz = vec![0.0; n];

        for i in 0..n {
            let (qxi, qyi, qzi) = (self.qx[i], self.qy[i], self.qz[i]);

            for j in (i + 1)..n {
                let dx: f64 = qxi - self.qx[j];
                let dy: f64 = qyi - self.qy[j];
                let dz: f64 = qzi - self.qz[j];

                let r2: f64 = dx * dx + dy * dy + dz * dz;
                let f: f64 = -Self::K / (r2 * r2.sqrt()) - Self::G * 2.0 / (r2 * r2);

                fx[i] += f * dx;
                fy[i] += f * dy;
                fz[i] += f * dz;

                fx[j] -= f * dx;
                fy[j] -= f * dy;
                fz[j] -= f * dz;
            }
        }

        (fx, fy, fz)
    }

    pub fn energy(&self) -> f64 {
        let mut e: f64 = 0.0;

        for i in 0..self.m.len() {
            e += 1.0 / (2.0 * self.m[i])
                * (self.px[i] * self.px[i] + self.py[i] * self.py[i] + self.pz[i] * self.pz[i])
        }

        for i in 0..self.m.len() {
            let (dxi, dyi, dzi) = (self.qx[i], self.qy[i], self.qz[i]);

            for j in 0..i {
                let dx = dxi - self.qx[j];
                let dy = dyi - self.qy[j];
                let dz = dzi - self.qz[j];

                let dr = (dx * dx + dy * dy + dz * dz).sqrt();

                e += Self::K / (dr) + Self::G / (dr * dr);
            }
        }

        e
    }

    pub fn update(&mut self, dt: f64) {
        // Evaluate dH/dp at (qn, pn)
        let (dH_dpx, dH_dpy, dH_dpz) = self.dH_dp();

        for i in 0..self.qx.len() {
            self.qx[i] += dt * dH_dpx[i];
            self.qy[i] += dt * dH_dpy[i];
            self.qz[i] += dt * dH_dpz[i];
        }

        // Evaluate dH/dp at (q(n+1), pn)
        let (dH_dqx, dH_dqy, dH_dqz) = self.dH_dq();

        for i in 0..self.px.len() {
            self.px[i] -= dt * dH_dqx[i];
            self.py[i] -= dt * dH_dqy[i];
            self.pz[i] -= dt * dH_dqz[i];
        }
    }

    pub fn get_q(&self) -> (&Vec<f64>, &Vec<f64>, &Vec<f64>) {
        (&self.qx, &self.qy, &self.qz)
    }
    
    pub fn get_particle_count(&self) -> usize {
        return self.m.len();
    }
}
