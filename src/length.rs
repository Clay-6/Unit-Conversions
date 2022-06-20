/// Convert a value in Miles to its
/// equivalent in Kilometers
#[inline]
pub fn miles_to_km(miles: f64) -> f64 {
    miles * 1.609344
}

/// Convert a value in Kilometers to its
/// equivalent in Miles
#[inline]
pub fn km_to_miles(km: f64) -> f64 {
    km * 0.6213712
}

/// Convert a value in Inches to its
/// equivalent in Centimeters
#[inline]
pub fn inches_to_cm(inches: f64) -> f64 {
    inches * 2.54
}

/// Convert a value in Centimeters to its
/// equivalent in Inches
#[inline]
pub fn cm_to_inches(cm: f64) -> f64 {
    cm / 2.54
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn km_to_mi() {
        assert_relative_eq!(km_to_miles(1.0), 0.6213712);
        assert_relative_eq!(km_to_miles(69.0), 42.8746128);
        assert_relative_eq!(km_to_miles(420.0), 260.975904);
    }

    #[test]
    fn mi_to_km() {
        assert_relative_eq!(miles_to_km(1.0), 1.609344);
        assert_relative_eq!(miles_to_km(69.0), 111.044736);
        assert_relative_eq!(miles_to_km(420.0), 675.92448);
    }

    #[test]
    fn in_to_cm() {
        assert_relative_eq!(inches_to_cm(1.0), 2.54);
        assert_relative_eq!(inches_to_cm(10.0), 25.40);
        assert_relative_eq!(inches_to_cm(100.0), 254.0);
    }

    #[test]
    fn cm_to_in() {
        assert_relative_eq!(cm_to_inches(1.0), 0.39370078740157477);
        assert_relative_eq!(cm_to_inches(10.0), 3.9370078740157477);
        assert_relative_eq!(cm_to_inches(100.0), 39.370078740157477);
    }
}
