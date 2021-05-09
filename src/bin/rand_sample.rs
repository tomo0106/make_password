use rand::distributions::{Distribution, Uniform};

fn main() {
    let range = Uniform::new(-1.0f64, 1.0);
    let mut rng = rand::thread_rng();

    let total = 1_000_000;
    let mut in_circle = 0;

    for _ in 0..total {
        let a = range.sample(&mut rng);
        let b = range.sample(&mut rng);
        if a * a + b * b <= 1.0 {
            in_circle += 1;
        }
    }

    // prints something close to 3.14159...
    println!(
        "π is approximately {}",
        4. * (in_circle as f64) / (total as f64)
    );
}
