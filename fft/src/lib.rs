use svg2pts_lib::get_path_from_file;
use num_complex::Complex;
use std::f32::consts::PI;

pub struct FourierCircle {
    pub speed: i32,
    pub radius: f32,
    pub phase: f32,
}

fn load_file(file: &str) -> Vec<Complex<f32>> {
    get_path_from_file(file, 0, 5.).iter().map(|(x, y)| {
	Complex {
	    re: x.clone() as f32,
	    im: -y.clone() as f32
	}
    }).collect()
}

fn calc_coefficient(speed: i32 , points: &Vec<Complex<f32>>) -> Complex<f32> {
    let i = Complex::new(0., 1.);

    points.iter().enumerate().map(|(index, point)| {
	point * Complex::exp(-speed as f32 * 2. * PI * i * (index as f32 / points.len() as f32)) * (1. / points.len() as f32)
    }).sum()
}

pub fn fourier_coefficients(file: &str, range: Box<dyn Iterator<Item=i32>>) -> Vec<FourierCircle> {
    let points = load_file(file);
    
    range.map(|speed| {
	let coefficient = calc_coefficient(speed, &points);
	let radius = coefficient.norm();
	let phase = coefficient.arg();

	FourierCircle {
	    speed,
	    radius,
	    phase
	}
    }).collect()
}
