// The hamiltonian for a N particle system
// H(p, q) = Σp_i^2/(2m_i) + Σ_(i<jV(q_i, q_j)
mod particlesystem;
use particlesystem::ParticleSystem;

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    println!("Hello, world!");

    let energy_csv = File::create("Energy_Latice.csv").unwrap();
    let mut writer = BufWriter::new(energy_csv);

    let q_csv = File::create("Q_Latice.csv").unwrap();
    let mut writer_q = BufWriter::new(q_csv);

    let mut lattice: ParticleSystem = ParticleSystem::from_initial_state(
        vec![-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0],
        vec![-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], // These should remain 0 if we want stable orbits
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    );

    writeln!(writer, "t\tenergy").unwrap(); // header
    let header = std::iter::once("t".to_string())
        .chain(
            ["qx", "qy", "qz", "px", "py", "pz"]
                .iter()
                .flat_map(|prefix| {
                    (1..=lattice.get_particle_count()).map(move |i| format!("{prefix}{i}"))
                }),
        )
        .collect::<Vec<_>>()
        .join("\t");
    writeln!(writer_q, "{header}").unwrap();

    let dt: f64 = 1e-6;
    let mut t: f64 = 0.0;
    const T: f64 = 5.0; // This is the max sim time
    // A for loop
    let mut i: u8 = 0;
    while t < T {
        t += dt;
        lattice.update(dt);

        // Downsampling
        i = (i + 1) % 250;
        if (i == 0) {
            print!("{}%\r", (100.0 * t / T).floor());
            writeln!(writer, "{}\t{}", t, lattice.energy()).unwrap();

            let (qx, qy, qz) = lattice.get_q();
            let coordinates: String = qx
                .iter()
                .chain(qy.iter())
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("\t");

            writeln!(writer_q, "{}\t{}", t, coordinates).unwrap();
        }
    }

    println!("Done!")
}
