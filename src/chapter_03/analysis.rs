//! Analysis and Visualization Tools for Asymptotic Functions
//!
//! This module provides tools for analyzing and visualizing the growth
//! of functions, demonstrating Rust's capabilities for mathematical analysis.

use super::asymptotic::AsymptoticFunction;

/// Growth rate comparison result
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthComparison {
    Faster,
    Same,
    Slower,
}

/// Compare the asymptotic growth rates of two functions
pub fn compare_growth<F, G>(f: &F, g: &G, samples: usize) -> GrowthComparison
where
    F: AsymptoticFunction,
    G: AsymptoticFunction,
{
    let mut f_growing_faster = 0;
    let mut g_growing_faster = 0;

    // Sample at exponentially increasing points
    let mut n = 1.0;

    for _ in 0..samples {
        let f_val = f.evaluate(n);
        let g_val = g.evaluate(n);

        if f_val > g_val * 1.1 {
            f_growing_faster += 1;
        } else if g_val > f_val * 1.1 {
            g_growing_faster += 1;
        }

        n *= 2.0;
    }

    let threshold = samples / 3;

    if f_growing_faster > threshold && f_growing_faster > g_growing_faster {
        GrowthComparison::Faster
    } else if g_growing_faster > threshold && g_growing_faster > f_growing_faster {
        GrowthComparison::Slower
    } else {
        GrowthComparison::Same
    }
}

/// Generate a text-based visualization of function growth
pub fn visualize_growth<F>(f: &F, n_start: f64, n_end: f64, width: usize, height: usize) -> String
where
    F: AsymptoticFunction,
{
    let mut result = String::new();
    result.push_str(&format!("Growth of {}\n", f.name()));
    result.push_str(&format!("n from {} to {}\n\n", n_start, n_end));

    // Find min and max values
    let mut min_val = f64::INFINITY;
    let mut max_val = f64::NEG_INFINITY;

    let num_samples = width * 2;
    for i in 0..=num_samples {
        let n = n_start + (n_end - n_start) * (i as f64) / (num_samples as f64);
        let val = f.evaluate(n);
        if val.is_finite() {
            let val_f64: f64 = val;
            min_val = min_val.min(val_f64);
            max_val = max_val.max(val_f64);
        }
    }

    if !min_val.is_finite() || !max_val.is_finite() {
        return format!(
            "Cannot visualize {} - function values out of range",
            f.name()
        );
    }

    // Create grid
    let mut grid = vec![vec![' '; width]; height];

    // Draw function
    for i in 0..width {
        let n = n_start + (n_end - n_start) * (i as f64) / (width as f64);
        let val = f.evaluate(n);

        if val.is_finite() {
            let row =
                height - 1 - ((val - min_val) / (max_val - min_val) * (height - 1) as f64) as usize;
            if row < height {
                grid[row][i] = '*';
            }
        }
    }

    // Draw axes
    let zero_row = height - 1 - ((-min_val) / (max_val - min_val) * (height - 1) as f64) as usize;
    if zero_row < height {
        for i in 0..width {
            if grid[zero_row][i] == ' ' {
                grid[zero_row][i] = '-';
            }
        }
    }

    // Convert to string
    for row in grid.iter().rev() {
        result.push_str(&format!("{:8.2e} |", max_val));
        result.push_str(&row.iter().collect::<String>());
        result.push('\n');
        max_val -= (max_val - min_val) / height as f64;
    }

    result.push_str(&format!("{:8} |", ""));
    result.push_str(&"-".repeat(width));
    result.push('\n');
    result.push_str(&format!("{:8} |", ""));
    result.push_str(&format!("{:>width$}", format!("{}", n_end), width = width));
    result.push('\n');

    result
}

/// Analyze function growth characteristics
pub fn analyze_function<F>(f: &F) -> String
where
    F: AsymptoticFunction,
{
    let mut analysis = String::new();
    analysis.push_str(&format!("Analysis of: {}\n\n", f.name()));

    // Sample values at different scales
    let scales = vec![1.0, 10.0, 100.0, 1000.0, 10000.0];

    analysis.push_str("Values at different scales:\n");
    for scale in scales {
        let val = f.evaluate(scale);
        analysis.push_str(&format!("  f({:>6}) = {:>15.6e}\n", scale, val));
    }

    // Check growth rate
    let growth_rate = estimate_growth_rate(f);
    analysis.push_str(&format!("\nEstimated growth rate: {:?}\n", growth_rate));

    analysis
}

/// Estimate the growth rate category of a function
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthRateCategory {
    Constant,
    Logarithmic,
    Polylogarithmic,
    Polynomial { degree: f64 },
    Exponential,
    Factorial,
    Unknown,
}

fn estimate_growth_rate<F>(f: &F) -> GrowthRateCategory
where
    F: AsymptoticFunction,
{
    let scales = [100.0, 1000.0, 10000.0];
    let mut ratios = Vec::new();

    for i in 1..scales.len() {
        let ratio = f.evaluate(scales[i]) / f.evaluate(scales[i - 1]);
        ratios.push(ratio);
    }

    // Analyze ratio patterns
    let avg_ratio = ratios.iter().sum::<f64>() / ratios.len() as f64;

    if avg_ratio < 1.1 {
        GrowthRateCategory::Constant
    } else if avg_ratio < 1.5 {
        GrowthRateCategory::Logarithmic
    } else if avg_ratio < 10.0 {
        // Could be polynomial - estimate degree
        let degree = avg_ratio.log10() / 3.0_f64.log10(); // log₁₀(ratio) / log₁₀(10) ≈ degree
        GrowthRateCategory::Polynomial { degree }
    } else if avg_ratio > 1000.0 {
        GrowthRateCategory::Exponential
    } else {
        GrowthRateCategory::Unknown
    }
}

/// Compare multiple functions side by side
/// Uses FunctionWrapper for dynamic dispatch
pub fn compare_functions(
    functions: &[&super::functions::FunctionWrapper],
    n_start: f64,
    n_end: f64,
    num_samples: usize,
) -> String {
    let mut result = String::new();
    result.push_str("Function Comparison\n");
    result.push_str(&"=".repeat(80));
    result.push('\n');
    result.push_str(&format!("{:<20} ", "Function"));

    for i in 0..num_samples {
        let n = n_start + (n_end - n_start) * (i as f64) / ((num_samples - 1) as f64);
        result.push_str(&format!("{:>12.0e}", n));
    }
    result.push('\n');
    result.push_str(&"-".repeat(80));
    result.push('\n');

    for func in functions {
        result.push_str(&format!("{:<20} ", func.name()));
        for i in 0..num_samples {
            let n = n_start + (n_end - n_start) * (i as f64) / ((num_samples - 1) as f64);
            let val = func.evaluate(n);
            if val.is_finite() && val < 1e15 {
                result.push_str(&format!("{:>12.2e}", val));
            } else {
                result.push_str(&format!("{:>12}", "∞"));
            }
        }
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chapter_03::Polynomial;

    #[test]
    fn test_compare_growth() {
        let n_squared = Polynomial::new(2.0);
        let n_cubed = Polynomial::new(3.0);

        let comparison = compare_growth(&n_squared, &n_cubed, 50);
        assert_eq!(comparison, GrowthComparison::Slower);
    }

    #[test]
    fn test_analyze_function() {
        let n_squared = Polynomial::new(2.0);
        let analysis = analyze_function(&n_squared);
        assert!(analysis.contains("n²"));
    }
}
