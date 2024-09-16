use rand_distr::{Distribution, Exp, Normal};
use rand::thread_rng;
use rand::Rng;
use plotters::prelude::*;
use std::f64::consts::E;

// Parameters

const S0: f64 = 100.0; // Initial stock price
const T: f64 = 10.0;   // Terminal time
const DT: f64 = 0.005; // Time step
const GAMMA: f64 = 0.5; // Risk aversion parameter
const SIGMA: f64 = 3.0; // Lower volatility of the mid-price for smoother prices
const K: f64 = 0.1;    // Lower K to make trades more frequent
const A: f64 = 280.0;  // Increased A to boost arrival rates
const LAMBDA: f64 = 0.1; // Base intensity of market orders
const MAX_INVENTORY: i32 = 1000; // Maximum allowable inventory


const NUM_SIMULATIONS: usize = 1000; // Number of simulations to run
const NUM_BINS: usize = 50; // Number of bins for the histogram

// Agent struct
struct Agent {
    inventory: i32,
    cash: f64,
    risk_aversion: f64,
}

impl Agent {
    fn new(risk_aversion: f64) -> Self {
        Agent {
            inventory: 0,
            cash: 0.0,
            risk_aversion,
        }
    }

    fn reservation_price(&self, s: f64, t: f64) -> f64 {
        s - (self.inventory as f64) * self.risk_aversion * SIGMA * SIGMA * (T - t)
    }

    fn optimal_spread(&self, t: f64) -> f64 {
        self.risk_aversion * SIGMA * SIGMA * ((T - t).max(0.01)) + (2.0 / self.risk_aversion) * (LAMBDA.ln())
    }

    fn bid_price(&self, s: f64, t: f64) -> f64 {
        self.reservation_price(s, t) - self.optimal_spread(t) / 2.0
    }

    fn ask_price(&self, s: f64, t: f64) -> f64 {
        self.reservation_price(s, t) + self.optimal_spread(t) / 2.0
    }

    fn execute_buy(&mut self, price: f64) {
        if self.inventory + 1 <= MAX_INVENTORY {
            self.inventory += 1;
            self.cash -= price;
        }
    }

    fn execute_sell(&mut self, price: f64) {
        if self.inventory - 1 >= -MAX_INVENTORY {
            self.inventory -= 1;
            self.cash += price;
        }
    }
}

// Simulate the mid-price as a Brownian motion
fn simulate_mid_price(s0: f64, sigma: f64, dt: f64, steps: usize) -> Vec<f64> {
    let mut prices = vec![s0];
    let normal = Normal::new(0.0, sigma * dt.sqrt()).unwrap();
    let mut rng = thread_rng();

    for _ in 0..steps {
        let ds = normal.sample(&mut rng);
        let s = prices.last().unwrap() + ds;
        prices.push(s);
    }
    prices
}

// Simulate Poisson arrivals for buy and sell orders
fn simulate_poisson_arrival(rate: f64, dt: f64) -> bool {
    let mut rng = thread_rng();
    let prob = (rate * dt).min(1.0);  // Cap the probability at 1.0
    rng.gen_bool(prob)
}


// Run a single simulation and return the final P&L
fn run_single_simulation() -> f64 {
    let steps = (T / DT) as usize;
    let mid_prices = simulate_mid_price(S0, SIGMA, DT, steps);
    let mut agent = Agent::new(GAMMA);

    let mut t = 0.0;
    for i in 0..steps {
        let s = mid_prices[i];
        t += DT;

        let bid = agent.bid_price(s, t);
        let ask = agent.ask_price(s, t);

        let lambda_bid = A * E.powf(-K * (s - bid));
        let lambda_ask = A * E.powf(-K * (ask - s));

        if simulate_poisson_arrival(lambda_bid, DT) {
            agent.execute_buy(bid);
        }
        if simulate_poisson_arrival(lambda_ask, DT) {
            agent.execute_sell(ask);
        }
    }

    // Calculate final P&L (cash + inventory value)
    agent.cash + (agent.inventory as f64) * mid_prices.last().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect P&L values from multiple simulations
    let mut pnl_values = Vec::new();

    for _ in 0..NUM_SIMULATIONS {
        let pnl = run_single_simulation();
        pnl_values.push(pnl);
    }

    // Plot P&L distribution
    plot_pnl_distribution(&pnl_values)?;

    Ok(())
}

// Bin the P&L values into discrete ranges and plot the distribution
fn plot_pnl_distribution(pnl_values: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("pnl_distribution.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_pnl, max_pnl) = pnl_values.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &v| {
        (min.min(v), max.max(v))
    });

    // Create bins for the histogram
    let bin_width = (max_pnl - min_pnl) / NUM_BINS as f64;
    let mut bins = vec![0; NUM_BINS];

    for &pnl in pnl_values {
        let bin_index = ((pnl - min_pnl) / bin_width).floor() as usize;
        let bin_index = bin_index.min(NUM_BINS - 1); // Prevent overflow on the last bin
        bins[bin_index] += 1;
    }

    let max_bin_count = *bins.iter().max().unwrap_or(&1);

    // Set up the chart with discrete bins on the x-axis
    let mut chart = ChartBuilder::on(&root)
        .caption("P&L Distribution", ("sans-serif", 30).into_font())
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_pnl..max_pnl, 0..max_bin_count)?;

    chart.configure_mesh().draw()?;

    // Draw the histogram bars
    chart.draw_series(
        bins.iter().enumerate().map(|(i, &count)| {
            let x0 = min_pnl + i as f64 * bin_width;
            let x1 = x0 + bin_width;
            Rectangle::new([(x0, 0), (x1, count)], RED.filled())
        })
    )?;

    Ok(())
}
