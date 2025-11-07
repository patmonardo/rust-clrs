//! Mathematical Functions for Asymptotic Analysis
//!
//! This module implements common mathematical functions as types that
//! implement AsymptoticFunction, demonstrating Rust's trait system for
//! mathematical abstractions.

use super::asymptotic::AsymptoticFunction;
use std::fmt;

/// Polynomial function: n^k
#[derive(Debug, Clone, Copy)]
pub struct Polynomial {
    pub degree: f64,
}

impl Polynomial {
    pub fn new(degree: f64) -> Self {
        Polynomial { degree }
    }
}

impl AsymptoticFunction for Polynomial {
    fn evaluate(&self, n: f64) -> f64 {
        n.powf(self.degree)
    }

    fn name(&self) -> String {
        if self.degree == 0.0 {
            "1".to_string()
        } else if self.degree == 1.0 {
            "n".to_string()
        } else if self.degree == 2.0 {
            "n²".to_string()
        } else if self.degree == 3.0 {
            "n³".to_string()
        } else {
            format!("n^{}", self.degree)
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Logarithmic function: log_b(n)
#[derive(Debug, Clone, Copy)]
pub struct Logarithm {
    pub base: f64,
}

impl Logarithm {
    pub fn new(base: f64) -> Self {
        Logarithm { base }
    }

    pub fn base_2() -> Self {
        Logarithm { base: 2.0 }
    }

    pub fn base_e() -> Self {
        Logarithm {
            base: std::f64::consts::E,
        }
    }
}

impl AsymptoticFunction for Logarithm {
    fn evaluate(&self, n: f64) -> f64 {
        n.ln() / self.base.ln()
    }

    fn name(&self) -> String {
        if self.base == 2.0 {
            "lg n".to_string()
        } else if self.base == std::f64::consts::E {
            "ln n".to_string()
        } else {
            format!("log_{}(n)", self.base)
        }
    }
}

impl fmt::Display for Logarithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Exponential function: b^n
#[derive(Debug, Clone, Copy)]
pub struct Exponential {
    pub base: f64,
}

impl Exponential {
    pub fn new(base: f64) -> Self {
        Exponential { base }
    }

    pub fn base_2() -> Self {
        Exponential { base: 2.0 }
    }
}

impl AsymptoticFunction for Exponential {
    fn evaluate(&self, n: f64) -> f64 {
        self.base.powf(n)
    }

    fn name(&self) -> String {
        if self.base == 2.0 {
            "2^n".to_string()
        } else {
            format!("{}^n", self.base)
        }
    }
}

impl fmt::Display for Exponential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Factorial function: n!
#[derive(Debug, Clone, Copy)]
pub struct Factorial;

impl AsymptoticFunction for Factorial {
    fn evaluate(&self, n: f64) -> f64 {
        if n < 0.0 {
            0.0
        } else if n < 2.0 {
            1.0
        } else {
            // Use Stirling's approximation for large n: n! ≈ √(2πn)(n/e)^n
            let n = n;
            let pi = std::f64::consts::PI;
            let e = std::f64::consts::E;

            if n > 20.0 {
                // Stirling's approximation
                (2.0 * pi * n).sqrt() * (n / e).powf(n)
            } else {
                // Exact for small n
                let mut result = 1.0;
                for i in 1..=n as u64 {
                    result *= i as f64;
                }
                result
            }
        }
    }

    fn name(&self) -> String {
        "n!".to_string()
    }
}

impl fmt::Display for Factorial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Sum of two functions: f(n) + g(n)
///
/// Note: Using concrete types for better performance.
/// For dynamic dispatch, wrap functions in FunctionWrapper.
#[derive(Debug, Clone)]
pub struct Sum {
    pub f: Box<FunctionWrapper>,
    pub g: Box<FunctionWrapper>,
}

impl Sum {
    pub fn new(f: Box<FunctionWrapper>, g: Box<FunctionWrapper>) -> Self {
        Sum { f, g }
    }
}

impl AsymptoticFunction for Sum {
    fn evaluate(&self, n: f64) -> f64 {
        self.f.evaluate(n) + self.g.evaluate(n)
    }

    fn name(&self) -> String {
        format!("({} + {})", self.f.name(), self.g.name())
    }
}

impl fmt::Display for Sum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Product of two functions: f(n) · g(n)
#[derive(Debug, Clone)]
pub struct Product {
    pub f: Box<FunctionWrapper>,
    pub g: Box<FunctionWrapper>,
}

impl Product {
    pub fn new(f: Box<FunctionWrapper>, g: Box<FunctionWrapper>) -> Self {
        Product { f, g }
    }
}

impl AsymptoticFunction for Product {
    fn evaluate(&self, n: f64) -> f64 {
        self.f.evaluate(n) * self.g.evaluate(n)
    }

    fn name(&self) -> String {
        format!("({} · {})", self.f.name(), self.g.name())
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Composition of two functions: f(g(n))
#[derive(Debug, Clone)]
pub struct Composition {
    pub outer: Box<FunctionWrapper>,
    pub inner: Box<FunctionWrapper>,
}

impl Composition {
    pub fn new(outer: Box<FunctionWrapper>, inner: Box<FunctionWrapper>) -> Self {
        Composition { outer, inner }
    }
}

impl AsymptoticFunction for Composition {
    fn evaluate(&self, n: f64) -> f64 {
        let inner_val = self.inner.evaluate(n);
        self.outer.evaluate(inner_val)
    }

    fn name(&self) -> String {
        format!("{}({})", self.outer.name(), self.inner.name())
    }
}

impl fmt::Display for Composition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Constant function: c
#[derive(Debug, Clone, Copy)]
pub struct Constant {
    pub value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Self {
        Constant { value }
    }
}

impl AsymptoticFunction for Constant {
    fn evaluate(&self, _n: f64) -> f64 {
        self.value
    }

    fn name(&self) -> String {
        format!("{}", self.value)
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Maximum of two functions: max(f(n), g(n))
#[derive(Debug, Clone)]
pub struct Max {
    pub f: Box<FunctionWrapper>,
    pub g: Box<FunctionWrapper>,
}

impl Max {
    pub fn new(f: Box<FunctionWrapper>, g: Box<FunctionWrapper>) -> Self {
        Max { f, g }
    }
}

impl AsymptoticFunction for Max {
    fn evaluate(&self, n: f64) -> f64 {
        let f_val: f64 = self.f.evaluate(n);
        let g_val: f64 = self.g.evaluate(n);
        if f_val > g_val {
            f_val
        } else {
            g_val
        }
    }

    fn name(&self) -> String {
        format!("max({}, {})", self.f.name(), self.g.name())
    }
}

impl fmt::Display for Max {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Wrapper for dynamic dispatch of functions
/// This allows us to use trait objects when needed
#[derive(Debug, Clone)]
pub enum FunctionWrapper {
    Polynomial(Polynomial),
    Logarithm(Logarithm),
    Exponential(Exponential),
    Factorial,
    Constant(Constant),
}

impl AsymptoticFunction for FunctionWrapper {
    fn evaluate(&self, n: f64) -> f64 {
        match self {
            FunctionWrapper::Polynomial(p) => p.evaluate(n),
            FunctionWrapper::Logarithm(l) => l.evaluate(n),
            FunctionWrapper::Exponential(e) => e.evaluate(n),
            FunctionWrapper::Factorial => Factorial.evaluate(n),
            FunctionWrapper::Constant(c) => c.evaluate(n),
        }
    }

    fn name(&self) -> String {
        match self {
            FunctionWrapper::Polynomial(p) => p.name(),
            FunctionWrapper::Logarithm(l) => l.name(),
            FunctionWrapper::Exponential(e) => e.name(),
            FunctionWrapper::Factorial => Factorial.name(),
            FunctionWrapper::Constant(c) => c.name(),
        }
    }
}

impl fmt::Display for FunctionWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctionWrapper::Polynomial(p) => p.fmt(f),
            FunctionWrapper::Logarithm(l) => l.fmt(f),
            FunctionWrapper::Exponential(e) => e.fmt(f),
            FunctionWrapper::Factorial => Factorial.fmt(f),
            FunctionWrapper::Constant(c) => c.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial() {
        let n_squared = Polynomial::new(2.0);
        assert_eq!(n_squared.evaluate(5.0), 25.0);
        assert_eq!(n_squared.name(), "n²");
    }

    #[test]
    fn test_logarithm() {
        let lg = Logarithm::base_2();
        assert!((lg.evaluate(8.0) - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_exponential() {
        let exp = Exponential::base_2();
        assert_eq!(exp.evaluate(3.0), 8.0);
    }

    #[test]
    fn test_factorial() {
        let fact = Factorial;
        assert_eq!(fact.evaluate(5.0), 120.0);
    }

    #[test]
    fn test_sum() {
        let n_squared = FunctionWrapper::Polynomial(Polynomial::new(2.0));
        let n = FunctionWrapper::Polynomial(Polynomial::new(1.0));
        let sum = Sum::new(Box::new(n_squared), Box::new(n));
        assert_eq!(sum.evaluate(5.0), 30.0);
    }

    #[test]
    fn test_product() {
        let n = FunctionWrapper::Polynomial(Polynomial::new(1.0));
        let lg = FunctionWrapper::Logarithm(Logarithm::base_2());
        let prod = Product::new(Box::new(n), Box::new(lg));
        assert!((prod.evaluate(8.0) - 24.0).abs() < 0.001);
    }
}
