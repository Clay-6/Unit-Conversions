use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The conversion source
    #[clap(subcommand)]
    pub source: Source,
}

#[derive(Debug, Subcommand)]
pub enum Source {
    /// Convert from Celsius.
    /// Alias is `c`
    #[clap(alias = "c")]
    Celsius {
        /// The value to convert
        value: f64,
        /// Convert to Fahrenheit
        #[clap(short, long)]
        fahrenheit: bool,
        /// Convert to Kelvin
        #[clap(short, long)]
        kelvin: bool,
    },
    /// Convert from Kelvin.
    /// Alias is `k`
    #[clap(alias = "k")]
    Kelvin {
        /// The value to convert
        value: f64,
        /// Convert to Celsius
        #[clap(short, long)]
        celsius: bool,
        /// Convert to Fahrenheit
        #[clap(short, long)]
        fahrenheit: bool,
    },
    /// Convert from Fahrenheit.
    /// Alias is `f`
    #[clap(alias = "f")]
    Fahrenheit {
        /// The value to convert
        value: f64,
        /// Convert to Celsius
        #[clap(short, long)]
        celsius: bool,
        /// Convert to Kelvin
        #[clap(short, long)]
        kelvin: bool,
    },
    /// Convert from Centimetres.
    /// Alias is `cm`
    #[clap(alias = "cm")]
    Centimetres {
        /// The value to convert
        value: f64,
        /// Convert to inches
        #[clap(short, long)]
        inches: bool,
        /// Convert to metres
        #[clap(short, long)]
        metres: bool,
        /// Convert to Kilometres
        #[clap(short, long)]
        kilometers: bool,
    },
    /// Convert from Metres.
    /// Alias is `m`
    #[clap(alias = "m")]
    Metres {
        /// The value to convert
        value: f64,
        /// Convert to feet
        #[clap(short, long)]
        feet: bool,
        /// Convert to Centimetres
        #[clap(short, long)]
        centimetres: bool,
        /// Convert to Kilometres
        #[clap(short, long)]
        kilometres: bool,
    },
    /// Convert from Kilometres.
    /// Alias is `km`
    #[clap(alias = "km")]
    Kilometres {
        /// The value to convert
        value: f64,
        /// Convert to miles
        #[clap(short, long)]
        miles: bool,
        /// Convert to Centimetres
        #[clap(short, long)]
        centimetres: bool,
        /// Convert to metres
        #[clap(short, long)]
        metres: bool,
    },
    /// Convert from inches.
    /// Alias is `in`
    #[clap(alias = "in")]
    Inches {
        /// The value to convert
        value: f64,
        /// Convert to Centimeters
        centimeters: bool,
    },
    /// Convert from feet.
    /// Alias is `ft`
    #[clap(alias = "ft")]
    Feet {
        /// The value to convert
        value: f64,
        /// Convert to Metres
        #[clap(short, long)]
        metres: bool,
    },
    /// Convert from Miles.
    /// Alias is `mi`
    #[clap(alias = "mi")]
    Miles {
        /// The value to convert
        value: f64,
        /// Convert to Kilometers
        #[clap(short, long)]
        km: bool,
    },
    /// Convert from Grams.
    /// Alias is `g`
    #[clap(alias = "g")]
    Grams {
        /// The value to convert
        value: f64,
        /// Convert to ounces
        #[clap(short, long)]
        ounces: bool,
        /// Convert to Kilograms
        #[clap(short, long)]
        kilograms: bool,
    },
    /// Convert from Kilograms.
    /// Alias is `kg`
    #[clap(alias = "kg")]
    Kilograms {
        /// The value to convert
        value: f64,
        /// Convert to Pounds
        #[clap(short, long)]
        lbs: bool,
        /// Convert to grams
        #[clap(short, long)]
        grams: bool,
    },
    /// Convert from Ounces.
    /// Alias is `oz`
    #[clap(alias = "oz")]
    Ounces {
        /// The value to convert
        value: f64,
        /// Convert to Grams
        #[clap(short, long)]
        grams: bool,
    },
    /// Convert from Pounds.
    /// Alias is `lb`
    #[clap(alias = "lb")]
    Pounds {
        /// The value to convert
        value: f64,
        /// Convert to Kilograms
        #[clap(short, long)]
        kg: bool,
    },
    /// Convert from Miles per hour.
    /// Alias is `mph`
    #[clap(alias = "mph")]
    MilesPerHour {
        value: f64,
        /// Convert to Kilometres per hour
        #[clap(short, long)]
        kmh: bool,
    },
    /// Convert from Kilometres per hour.
    /// Alias is 'kmh`
    #[clap(alias = "kmh")]
    KilometersPerHour {
        value: f64,
        /// Convert to Miles per hour
        #[clap(short, long)]
        mph: bool
    },
}
