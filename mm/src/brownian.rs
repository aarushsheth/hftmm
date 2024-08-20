use ndarray::ArrayViewMut1;
use rand_distr::{Normal, Distribution};
use rand::rngs::ThreadRng;
use std::error::Error;
use ndarray::Array1;
use ndarray::s;

pub fn brownian(x0: &mut ArrayViewMut1<f64>, dt: f64, delta: f64) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let normal_dist = Normal::new(0.0, delta * dt.sqrt()).expect("Failed to create normal distribution");

    for val in x0.iter_mut() {
        *val += normal_dist.sample(&mut rng);
    }

    Ok(())
}

fn main() {
    let mut x0 = Array1::<f64>::from_vec(vec![0.0, 10.0, 20.0]); // Make x0 mutable
    let n = 1000; // Number of steps
    let dt = 1.0; // Time step
    let delta = 1.0; // Scale factor for the variance

    let mut x0_slice = x0.slice_mut(s![1..]);

    match brownian(&mut x0_slice, dt, delta) {
        Ok(_) => println!("Simulation successful!"),
        Err(e) => println!("Error during simulation: {}", e),
    }

    println!("Results: {:?}", x0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{Array1, s};

    #[test]
    fn test_zero_variance() {
        let mut x0 = Array1::from_vec(vec![0.0; 11]); // Ensure x0 has space for initial value + 10 steps
        let dt = 1.0;
        let delta = 0.0;  // Zero variance

        brownian(&mut x0.slice_mut(s![1..]), dt, delta).unwrap();
        assert!(x0.slice(s![1..]).iter().all(|&x| (x - x0[0]).abs() < 1e-8), "With zero variance, the path should not deviate.");
    }

    #[test]
    fn test_single_step() {
        let mut x0 = Array1::from_vec(vec![5.0, 0.0]); // Initial value and space for one step
        let dt = 1.0;
        let delta = 1.0;
    
        brownian(&mut x0.slice_mut(s![1..]), dt, delta).unwrap();
        assert_eq!(x0.len(), 2, "There should only be two entries, initial and one step.");
    }

    #[test]
    fn test_independence() {
        let mut x0 = Array1::from_vec(vec![0.0, 10.0, 20.0, 0.0, 0.0, 0.0]); // Initial conditions and space for steps
        let dt = 1.0;
        let delta = 1.0;

        brownian(&mut x0.slice_mut(s![3..]), dt, delta).unwrap();

        assert!(x0[3] != x0[4] && x0[4] != x0[5], "Values should differ indicating potential independence.");
    }
}
