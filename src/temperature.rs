use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Temperature {
    Celsius(f64),
    Kelvin(f64),
    Fahrenheit(f64),
}

impl Temperature {
    /// Returns `self`'s contained value converted to Celsius
    /// as a [`Temperature::Celsius`], returning a copy of `self` if
    /// it was already in Celsius
    pub fn to_celsius(&self) -> Temperature {
        match self {
            Temperature::Celsius(_) => *self,
            Temperature::Kelvin(k) => Self::Celsius(kelvin_to_celsius(*k)),
            Temperature::Fahrenheit(f) => Self::Celsius(fahrenheit_to_celsius(*f)),
        }
    }

    /// Returns `self`'s contained value converted to Kelvin
    /// as a [`Temperature::Kelvin`], returning a copy of `self` if
    /// it was already in Kelvin
    pub fn to_kelvin(&self) -> Temperature {
        match self {
            Temperature::Kelvin(_) => *self,
            Temperature::Celsius(c) => Self::Kelvin(celsius_to_kelvin(*c)),
            Temperature::Fahrenheit(f) => Self::Kelvin(fahrenheit_to_kelvin(*f)),
        }
    }

    /// Returns `self`'s contained value
    /// converted to Fahrenheit as a [`Temperature::Fahrenheit`],
    /// returning a copy of `self` if it was already in Fahrenheit
    pub fn to_fahrenheit(&self) -> Temperature {
        match self {
            Temperature::Fahrenheit(_) => *self,
            Temperature::Celsius(c) => Self::Fahrenheit(celsius_to_fahrenheit(*c)),
            Temperature::Kelvin(k) => Self::Fahrenheit(kelvin_to_fahrenheit(*k)),
        }
    }
}

/// Convenience macro to create a [`Temperature::Celsius`]
#[macro_export]
macro_rules! celsius {
    ($c:expr) => {
        Temperature::Celsius($c)
    };
}

/// Convenience macro to create a [`Temperature::Kelvin`]
#[macro_export]
macro_rules! kelvin {
    ($k:expr) => {
        Temperature::Kelvin($k)
    };
}

/// Convenience macro to create a [`Temperature::Fahrenheit`]
#[macro_export]
macro_rules! fahrenheit {
    ($f:expr) => {
        Temperature::Fahrenheit($f)
    };
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Temperature::Celsius(c) => write!(f, "{}℃", c),
            Temperature::Kelvin(k) => write!(f, "{}K", k),
            Temperature::Fahrenheit(fa) => write!(f, "{}℉", fa),
        }
    }
}

#[inline]
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

#[inline]
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

#[inline]
pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

#[inline]
pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

#[inline]
pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + 459.67) * 1.8
}

#[inline]
pub fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 1.8 + 32.0
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

    #[test]
    fn f_to_k() {
        let abs_zero = -459.67;
        assert_relative_eq!(fahrenheit_to_kelvin(abs_zero), 0.0);
    }

    #[test]
    fn k_to_f() {
        let abs_zero = 0.0;
        assert_relative_eq!(kelvin_to_fahrenheit(abs_zero), -459.67);
    }

    #[test]
    fn celsius_macro() {
        assert_eq!(celsius!(69.0), Temperature::Celsius(69.0));
    }

    #[test]
    fn kelvin_macro() {
        assert_eq!(kelvin!(69.0), Temperature::Kelvin(69.0));
    }

    #[test]
    fn fahrenheit_macro() {
        assert_eq!(fahrenheit!(69.0), Temperature::Fahrenheit(69.0));
    }
}
