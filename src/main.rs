use rand::Rng;
use burn::tensor::Tensor;
use burn::module::Module;
use burn::optim::{Optimizer, Sgd, SgdConfig};
use burn::nn::loss::MseLoss;
use textplots::{Chart, Plot, Shape};

// Generate Synthetic Data
fn generate_data(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let mut x_data = Vec::with_capacity(n);
    let mut y_data = Vec::with_capacity(n);

    for _ in 0..n {
        let x: f64 = rng.gen_range(0.0..10.0);
        let noise: f64 = rng.gen_range(-1.0..1.0);
        let y = 2.0 * x + 1.0 + noise;
        x_data.push(x);
        y_data.push(y);
    }

    (x_data, y_data)
}

// Define the Model
#[derive(Module, Debug)]
struct LinearRegression {
    weight: Tensor<f64>,
    bias: Tensor<f64>,
}

impl LinearRegression {
    fn new() -> Self {
        Self {
            weight: Tensor::random([1], -1.0, 1.0),
            bias: Tensor::random([1], -1.0, 1.0),
        }
    }

    fn forward(&self, x: &Tensor<f64>) -> Tensor<f64> {
        x * &self.weight + &self.bias
    }

    fn loss(&self, predictions: &Tensor<f64>, targets: &Tensor<f64>) -> Tensor<f64> {
        MseLoss::new().forward(predictions, targets, burn::nn::loss::Reduction::Mean)
    }
}

// Train the Model
fn train_model(model: &mut LinearRegression, x_data: &[f64], y_data: &[f64], epochs: usize, lr: f64) {
    let optimizer_config = SgdConfig::new().learning_rate(lr);
    let mut optimizer = Sgd::new(optimizer_config);

    for epoch in 0..epochs {
        let x_tensor = Tensor::from_slice(x_data);
        let y_tensor = Tensor::from_slice(y_data);

        let predictions = model.forward(&x_tensor);
        let loss = model.loss(&predictions, &y_tensor);

        optimizer.step(&loss);

        println!("Epoch {}: Loss = {:?}", epoch, loss);
    }
}

// Evaluate the Model
fn evaluate_model(model: &LinearRegression, x_data: &[f64], y_data: &[f64]) {
    let x_tensor = Tensor::from_slice(x_data);
    let predictions = model.forward(&x_tensor);

    let mut data = vec![];
    for i in 0..x_data.len() {
        data.push((x_data[i], predictions[i].to_f64().unwrap()));
    }

    Chart::new(180, 60, 0.0, 10.0)
        .lineplot(&Shape::Lines(&data))
        .display();
}

fn main() {
    let (x_data, y_data) = generate_data(100);

    let mut model = LinearRegression::new();
    train_model(&mut model, &x_data, &y_data, 1000, 0.01);

    evaluate_model(&model, &x_data, &y_data);
}