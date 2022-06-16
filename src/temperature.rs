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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_to_c() {
        let abs_zero = -459.67;
        assert_eq!(fahrenheit_to_celsius(abs_zero), -273.15);

        let parity = -40.0;
        assert_eq!(fahrenheit_to_celsius(parity), -40.0)
    }

    #[test]
    fn c_to_f() {
        let abs_zero = -273.15;
        assert_eq!(celsius_to_fahrenheit(abs_zero), -459.67);

        let parity = -40.0;
        assert_eq!(celsius_to_fahrenheit(parity), -40.0);
    }
}
