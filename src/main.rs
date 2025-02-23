use rand::Rng;
use std::f64;
fn generate_synthetic_data(num_points: usize) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    for _ in 0..num_points {
        // Generate random x value in a range (e.g., -10 to 10)
        let x: f64 = rng.gen_range(-10.0..10.0);

        // Calculate y = 2x + 1
        let y_true = 2.0 * x + 1.0;

        // Add noise to y value (e.g., Gaussian noise)
        let noise: f64 = rng.gen_range(-2.0..2.0); // Random noise between -2 and 2
        let y_noisy = y_true + noise;

        // Store the (x, y) pair
        data.push((x, y_noisy));
    }

    data
}

fn main() {
    let num_points = 100; // Number of points to generate
    let synthetic_data = generate_synthetic_data(num_points);

    // Print out the first 10 data points as a sample
    for (x, y) in synthetic_data.iter().take(10) {
        println!("x: {:.2}, y: {:.2}", x, y);
    }
}
