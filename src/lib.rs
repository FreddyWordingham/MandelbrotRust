pub mod complex;

use crate::complex::Complex;
use image::RgbImage;
use ndarray::{arr1, s, Array2, Array3};
use palette::{Gradient, LinSrgb, Pixel};
use pyo3::prelude::*;

#[pyfunction]
fn mandelbrot_single(c: Complex, max_iter: i32) -> i32 {
    let mut z = Complex::zero();
    for n in 0..max_iter {
        z = z * z + c.clone();
        if z.mag_sqrt() > 4.0 {
            return n;
        }
    }
    return max_iter;
}

fn mandelbrot_range(
    centre: Complex,
    scale: f64,
    res: [usize; 2],
    max_iter: i32,
    data: &mut Array2<i32>,
) {
    let aspect_ratio = res[0] as f64 / res[1] as f64;
    let start = centre + Complex::new(scale * -0.5, scale / aspect_ratio * -0.5);
    let delta = scale / (res[0] - 1).max(1) as f64;

    for yi in 0..res[1] {
        let y = start.im + (delta * yi as f64);
        for xi in 0..res[0] {
            let x = start.re + (delta * xi as f64);
            let c = Complex::new(x, y);
            data[(xi, yi)] = mandelbrot_single(c, max_iter);
        }
    }
}

fn data_to_rgb(data: &Array2<i32>, max_iter: i32, rgb: &mut Array3<u8>) {
    let max = max_iter as f32;
    let cmap = Gradient::new(vec![
        LinSrgb::new(0.00, 0.05, 0.20),
        LinSrgb::new(0.70, 0.10, 0.20),
        LinSrgb::new(0.95, 0.90, 0.30),
    ]);

    let (width, height) = data.dim();
    for yi in 0..height {
        for xi in 0..width {
            let col = cmap.get(data[(xi, yi)] as f32 / max);
            let u8s: [u8; 3] = col.into_format().into_raw();
            rgb.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
        }
    }
}

fn rgb_to_image(arr: Array3<u8>) -> RgbImage {
    let (width, height, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

#[pyfunction]
fn mandelbrot_video(
    centre: Complex,
    mut scale: f64,
    rate: f64,
    res: [usize; 2],
    frames: usize,
    max_iter: i32,
) {
    assert!(scale > 0.0);
    assert!(rate > 0.0);
    assert!(res[0] > 0);
    assert!(res[1] > 0);
    assert!(frames > 0);
    assert!(max_iter > 0);

    let mut data = Array2::<i32>::zeros(res);
    let mut rgb = Array3::<u8>::zeros((res[0], res[1], 3));

    for n in 0..frames {
        println!(
            "Frame {} of {} \t ({}x)",
            n + 1,
            frames,
            (1.0 / scale).log10() as i32
        );
        mandelbrot_range(centre, scale, res, max_iter, &mut data);
        data_to_rgb(&data, max_iter, &mut rgb);
        rgb_to_image(rgb.clone())
            .save(format!("output/img_{:04}.png", n))
            .expect("Failed to save image.");

        scale *= rate;
    }
}

#[pymodule]
fn mandelbrot(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Complex>()?;
    m.add_function(wrap_pyfunction!(mandelbrot_single, m)?)?;
    m.add_function(wrap_pyfunction!(mandelbrot_video, m)?)?;
    Ok(())
}
