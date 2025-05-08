use std::{f64::consts::PI, io};

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
    let mut inputabs = input.abs();
    while inputabs>(3.0*PI)/2.0 {
        inputabs -= PI;
    }
    if inputabs<=(3.0*PI)/2.0 && inputabs>=PI {
        inputabs = -calc_sin(inputabs-PI);
    }
    else if input>=PI/2.0 && input<=PI {
        inputabs = calc_sin(PI-inputabs);
    }
    else {
        inputabs = calc_sin(inputabs);
    }
    if input.is_sign_negative() {
        inputabs = -inputabs;
    }
    if inputabs == -0.0 {
        inputabs = 0.0;
    }
    return inputabs;
}

fn main() {
    let mut input = String::new();
    println!("Enter value to sin:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f64 = parse_pi_expression(input.trim_end()).unwrap();
    let mut output = use_funi_trig(input);
    println!("{:.5}", output);
    //<debug>
    println!("{}", (PI/2.0)-3.0*PI);
    //</debug>
    output = use_funi_trig((PI/2.0)-3.0*PI);
    println!("{:.5}", output);
}
