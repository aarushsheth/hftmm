use crate::brownian::brownian;
use ndarray::{Array1, s};
use plotters::prelude::*;

pub fn simulate_and_plot() -> Result<(), Box<dyn std::error::Error>> {
    let delta = 10.0;
    let t = 10.0;
    let n = 500;
    let dt = t / n as f64;
    let initial_x = 1000.0;

    let mut x = Array1::<f64>::from_elem(n + 1, initial_x);
    brownian(&mut x.slice_mut(s![1..]), dt, delta)?;  // Using the s! macro for slicing

    let root = BitMapBackend::new("plot.png", (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Brownian Motion Path", ("sans-serif", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..t, 980.0..1020.0)?;
    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (0..=n).map(|i| (i as f64 * dt, x[i])),
        &BLUE,
    ))?;

    root.present()?;
    Ok(())
}

fn main() {
    if let Err(e) = simulate_and_plot() {
        eprintln!("Error: {}", e);
    }
}
