/// Convert a speed in Miles per hour
/// to its equivalent in Kilometres
/// per hour
#[inline]
pub fn mph_to_kmh(mph: f64) -> f64 {
    mph * 1.609344
}

/// Convert a speed in Kilometres per
/// hour to its equivalent in Miles
/// per hour
#[inline]
pub fn kmh_to_mph(kmh: f64) -> f64 {
    kmh / 1.609344
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn mph_to_kph() {
        assert_relative_eq!(mph_to_kmh(1.0), 1.609344);
        assert_relative_eq!(mph_to_kmh(69.0), 111.044736);
        assert_relative_eq!(mph_to_kmh(420.0), 675.92448);
    }

    #[test]
    fn kph_to_mph() {
        assert_relative_eq!(kmh_to_mph(1.0), 0.621371192237334);
        assert_relative_eq!(kmh_to_mph(69.0), 42.87461226437604);
        assert_relative_eq!(kmh_to_mph(420.0), 260.97590073968024);
    }
}
