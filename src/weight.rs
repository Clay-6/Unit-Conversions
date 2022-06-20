/// Convert a value in Pounds to its
/// equivalent in Kilograms
pub fn lbs_to_kg(lbs: f64) -> f64 {
    lbs * 0.45359237
}

/// Convert a value in Kilograms to
/// its equivalent in Pounds
pub fn kg_to_lbs(kg: f64) -> f64 {
    kg / 0.45359237
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn pounds_to_kg() {
        assert_relative_eq!(lbs_to_kg(0.220), 0.0997903214);
        assert_relative_eq!(lbs_to_kg(22.04623), 10.000001715265102);
        assert_relative_eq!(lbs_to_kg(220.4623), 100.00001715265101);
    }

    #[test]
    fn kg_to_pounds() {
        assert_relative_eq!(kg_to_lbs(0.1), 0.22046226218487758);
        assert_relative_eq!(kg_to_lbs(10.0), 22.046226218487757);
        assert_relative_eq!(kg_to_lbs(100.0), 220.46226218487757);
    }
}
