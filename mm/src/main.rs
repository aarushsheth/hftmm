use ndarray::Array1;
use ndarray::s;
use plotters::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::f64::consts::E;
use std::f64::{MIN, MAX};


use mm::brownian::brownian;

fn main() {
    let n_sim: usize = 100;
    let mut pnl_sim = Vec::with_capacity(n_sim);
    let mut max_q_held = f64::MIN;
    let mut min_q_held = f64::MAX;

    let mut last_q = 0.0;
    let mut last_s = 0.0;
    let mut last_x = 0.0;

    let mut final_s = Array1::<f64>::zeros(0);
    let mut final_r = Array1::<f64>::zeros(0);
    let mut final_ra = Array1::<f64>::zeros(0);
    let mut final_rb = Array1::<f64>::zeros(0);
    let mut final_x = Array1::<f64>::zeros(0);
    let mut final_q = Array1::<f64>::zeros(0);

    for _ in 0..n_sim {
        let (s, x, q, r, ra, rb) = run_simulation();
        pnl_sim.push(x[x.len() - 1] + q[q.len() - 1] * s[s.len() - 1]);
        last_q = *q.last().unwrap();
        last_s = *s.last().unwrap();
        last_x = *x.last().unwrap();
        max_q_held = max_q_held.max(*q.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
        min_q_held = min_q_held.min(*q.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());

        final_s = s;
        final_r = r;
        final_ra = ra;
        final_rb = rb;
        final_x = x;
        final_q = q;
    }

    
    println!("Last simulation results:");
    println!("Final inventory hold: {}", last_q);
    println!("Last price: {}", last_s);
    println!("Cash: {}", last_x);
    println!("Final wealth: {}", pnl_sim[pnl_sim.len() - 1]);
    println!("Max q held: {}", max_q_held);
    println!("Min q held: {}", min_q_held);

    plot_results(&final_s, &final_r, &final_ra, &final_rb, &final_x, &final_q);
}

fn run_simulation() -> (Array1<f64>, Array1<f64>, Array1<f64>, Array1<f64>, Array1<f64>, Array1<f64>) {
    let sigma = 2.0;
    let t = 1.0;
    let n = 200;
    let dt = t / n as f64;

    let mut s = Array1::from_elem(n + 1, 100.0);
    brownian(&mut s.slice_mut(s![1..]), dt, sigma).unwrap();

    let mut x = Array1::from_elem(n + 1, 0.0);
    let mut q = Array1::from_elem(n + 1, 0.0);
    let mut r = Array1::from_elem(n + 1, 0.0);
    let mut ra = Array1::from_elem(n + 1, 0.0);
    let mut rb = Array1::from_elem(n + 1, 0.0);

    let limit_horizon = true;
    let gamma = 0.1;
    let k = 0.1;
    let m = s[0] / 200.0;
    let a = 1.0 / dt / E.powf(k * m / 2.0);

    for i in 0..n {
        if limit_horizon {
            let r_spread = 2.0 / gamma * (E.powf(gamma / k) - 1.0).ln();
            r[i] = s[i] - q[i] * gamma * sigma.powi(2) * (t - dt * i as f64);
            ra[i] = r[i] + r_spread / 2.0;
            rb[i] = r[i] - r_spread / 2.0;
        }

        let delta_a = ra[i] - s[i];
        let delta_b = s[i] - rb[i];
        let lambda_a = a * (-k * (delta_a / 300.0));
        let lambda_b = a * (-k * (delta_b / 300.0)).exp();

        let ya = rand::thread_rng().gen_range(0.0..1.0);
        let yb = rand::thread_rng().gen_range(0.0..1.0);

        let dna = if ya < 1.0 - (-lambda_a * dt).exp() { 1 } else { 0 };
        let dnb = if yb < 1.0 - (-lambda_b * dt).exp() { 1 } else { 0 };

        q[i + 1] = q[i] - dna as f64 + dnb as f64;
        x[i + 1] = x[i] + ra[i] * dna as f64 - rb[i] * dnb as f64;
        println!("Step {}: s = {:.2}, x = {:.2}, q = {:.2}, r = {:.2}, ra = {:.2}, rb = {:.2}, lambda_a = {:.2}, lambda_b = {:.2}, dna = {}, dnb = {}", 
            i + 1, s[i + 1], x[i + 1], q[i + 1], r[i], ra[i], rb[i], lambda_a, lambda_b, dna, dnb);
    }


    (s, x, q, r, ra, rb)
}


fn plot_results(s: &Array1<f64>, r: &Array1<f64>, ra: &Array1<f64>, rb: &Array1<f64>, x: &Array1<f64>, q: &Array1<f64>) {
    let t: Vec<f64> = (0..s.len()).map(|i| i as f64 / s.len() as f64).collect::<Vec<_>>();

    let root = BitMapBackend::new("plot.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let s_min = s.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let s_max = s.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let margin_factor = 0.1; // 10% margin on both sides
    let y_min = s_min - (s_max - s_min) * margin_factor;
    let y_max = s_max + (s_max - s_min) * margin_factor;

    let mut chart = ChartBuilder::on(&root)
        .caption("Avellaneda-Stoikov Market Making", ("sans-serif", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..t[t.len() - 1], y_min..y_max)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(
        t.iter().zip(s.iter()).map(|(&t, &s)| (t, s)),
        &BLACK,
    )).unwrap()
    .label("Mid-market price")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart.draw_series(LineSeries::new(
        t.iter().zip(r.iter()).map(|(&t, &r)| (t, r)),
        &BLUE,
    )).unwrap()
    .label("Reservation price")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.draw_series(LineSeries::new(
        t.iter().zip(ra.iter()).map(|(&t, &ra)| (t, ra)),
        &RED,
    )).unwrap()
    .label("Price asked")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(
        t.iter().zip(rb.iter()).map(|(&t, &rb)| (t, rb)),
        &GREEN,
    )).unwrap()
    .label("Price bid")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart.configure_series_labels()
    .position(SeriesLabelPosition::UpperLeft)
    .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}
