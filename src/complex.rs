use pyo3::prelude::*;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

#[pyclass]
#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[pymethods]
impl Complex {
    #[staticmethod]
    pub fn zero() -> Complex {
        Complex { re: 0.0, im: 0.0 }
    }

    #[new]
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
}

impl Complex {
    pub fn mag_sqrt(self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, other: f64) -> Complex {
        Complex {
            re: self.re * other,
            im: self.re * other,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: (self.re * other.re) - (self.im * other.im),
            im: (self.re * other.im) + (self.im * other.re),
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.re, self.im)
    }
}
