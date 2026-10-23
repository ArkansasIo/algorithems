pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() { None } else { Some(values.iter().sum::<f64>() / values.len() as f64) }
}

pub fn variance(values: &[f64]) -> Option<f64> {
    let m = mean(values)?;
    Some(values.iter().map(|x| (x - m).powi(2)).sum::<f64>() / values.len() as f64)
}

pub fn standard_deviation(values: &[f64]) -> Option<f64> { variance(values).map(f64::sqrt) }

pub fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() { return None; }
    values.sort_by(f64::total_cmp);
    let n = values.len();
    Some(if n % 2 == 0 { (values[n/2 - 1] + values[n/2]) / 2.0 } else { values[n/2] })
}
