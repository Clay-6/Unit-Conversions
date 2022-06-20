/// Convert a value in Pounds to its
/// equivalent in Kilograms
#[inline]
pub fn pounds_to_kg(lbs: f64) -> f64 {
    lbs * 0.45359237
}

/// Convert a value in Kilograms to
/// its equivalent in Pounds
#[inline]
pub fn kg_to_pounds(kg: f64) -> f64 {
    kg / 0.45359237
}

/// Convert a value in Ounces to its
/// equivalent in Grams
#[inline]
pub fn ounces_to_grams(oz: f64) -> f64 {
    oz * 28.34952
}

/// Convert a value in Grams to its
/// equivalent in Ounces
#[inline]
pub fn grams_to_ounces(g: f64) -> f64 {
    g * 0.03527396195
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn lb_to_kg() {
        assert_relative_eq!(pounds_to_kg(0.220), 0.0997903214);
        assert_relative_eq!(pounds_to_kg(22.04623), 10.000001715265102);
        assert_relative_eq!(pounds_to_kg(220.4623), 100.00001715265101);
    }

    #[test]
    fn kg_to_lb() {
        assert_relative_eq!(kg_to_pounds(0.1), 0.22046226218487758);
        assert_relative_eq!(kg_to_pounds(10.0), 22.046226218487757);
        assert_relative_eq!(kg_to_pounds(100.0), 220.46226218487757);
    }

    #[test]
    fn oz_to_g() {
        assert_relative_eq!(ounces_to_grams(1.0), 28.34952);
        assert_relative_eq!(ounces_to_grams(10.0), 283.4952);
        assert_relative_eq!(ounces_to_grams(100.0), 2834.952);
    }

    #[test]
    fn g_to_oz() {
        assert_relative_eq!(grams_to_ounces(1.0), 0.03527396195);
        assert_relative_eq!(grams_to_ounces(10.0), 0.3527396195);
        assert_relative_eq!(grams_to_ounces(100.0), 3.527396195);
    }
}
