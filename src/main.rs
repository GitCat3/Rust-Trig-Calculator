use std::{f64::{consts::PI, NAN}, io};

fn calc_sin(x: f64) -> f64 {
    let terms = 30;
    let mut sum = 0.0;
    let mut term = x;
    let mut n = 1;

    for _ in 0..terms {
        sum += term;
        n += 2;
        term *= -x * x / ((n - 1) as f64 * n as f64);
    }
    sum
}

fn parse_pi_expression(expr: &str) -> Option<f64> {
    let expr = expr.replace(" ", "").replace("π", "pi");
    let (numerator, denominator) = if let Some((num, denom)) = expr.split_once('/') {
        (num, denom)
    } else {
        (expr.as_str(), "1")
    };
    let pi_index = numerator.find("pi")?;
    let coeff_str = &numerator[..pi_index];
    let coeff = if coeff_str.is_empty() {
        1.0
    } else if coeff_str == "-" {
        -1.0
    } else {
        coeff_str.parse::<f64>().ok()?
    };
    let denom = denominator.parse::<f64>().ok()?;
    Some(coeff * std::f64::consts::PI / denom)
}

fn use_funi_trig(input: f64) -> f64{
    let mut angle = input.rem_euclid(2.0 * PI);
    if angle > PI {
        angle -= 2.0 * PI;
    }
    else if angle < -PI {
        angle += 2.0 * PI;
    }
    if angle > PI/2.0 {
        calc_sin(PI-angle)
    }
    else if angle < -PI/2.0 {
        -calc_sin(-PI - angle)
    }
    else {
        calc_sin(angle)
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter value to calculate:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = parse_pi_expression(input.trim_end()).unwrap();
    let outputsin = use_funi_trig(input);
    let outputcos = use_funi_trig(input + (PI/2.0));
    let outputtan = if outputcos == 0.0 { NAN } else { outputsin / outputcos };
    println!("sin: {:.5}", outputsin);
    println!("cosine: {:.5}", outputcos);
    println!("tan: {:.5}", outputtan);
    println!("cosec: {:.5}", if outputsin == 0.0 {NAN} else {1.0/outputsin});
    println!("sec: {:.5}", if outputcos == 0.0 {NAN} else {1.0/outputcos});
    println!("cotan: {:.5}", if outputtan == 0.0 {NAN} else {1.0/outputtan});
}
