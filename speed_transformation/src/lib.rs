pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    let meters = km_h * 1000.0;
    let seconds =  60.0 * 60.0;
    meters / seconds
}