/// Converts a temperature in Fahrenheit to
/// its equivalent in Celsius
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

/// Converts a temperature in Celsius to its
/// equivalent in Fahrenheit
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

/// Converts a temperature in Kelvin to its
/// equivalent in celsius
pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

/// Converts a temperature in Celsius to its
/// equivalent in Kelvin
pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn f_to_c() {
        let abs_zero = -459.67;
        assert_relative_eq!(fahrenheit_to_celsius(abs_zero), -273.15);

        let parity = -40.0;
        assert_relative_eq!(fahrenheit_to_celsius(parity), -40.0)
    }

    #[test]
    fn c_to_f() {
        let abs_zero = -273.15;
        assert_relative_eq!(celsius_to_fahrenheit(abs_zero), -459.67);

        let parity = -40.0;
        assert_relative_eq!(celsius_to_fahrenheit(parity), -40.0);
    }

    #[test]
    fn k_to_c() {
        let abs_zero = 0.0;
        assert_relative_eq!(kelvin_to_celsius(abs_zero), -273.15);

        let room_temp = 294.15;
        assert_relative_eq!(kelvin_to_celsius(room_temp), 21.0);
    }

    #[test]
    fn c_to_k() {
        let abs_zero = -273.15;
        assert_relative_eq!(celsius_to_kelvin(abs_zero), 0.0);

        let room_temp = 21.0;
        assert_relative_eq!(celsius_to_kelvin(room_temp), 294.15);
    }
}
