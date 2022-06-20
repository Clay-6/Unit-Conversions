use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The conversion source
    #[clap(subcommand)]
    pub source: Source,
}

#[derive(Debug, Subcommand)]
pub enum Source {
    /// Convert a temperature from Celsius
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
    /// Convert a temperature from Kelvin
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
    /// Convert a temperature from Fahrenheit
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
    /// Convert from Kilometers. Alias is `km`
    #[clap(alias = "km")]
    Kilometers {
        /// The value to convert
        value: f64,
        /// Convert to miles
        #[clap(short, long)]
        miles: bool,
    },
    /// Convert from Miles
    Miles {
        /// The value to convert
        value: f64,
        /// Convert to Kilometers
        #[clap(short, long)]
        km: bool,
    },
    /// Convert from Kilograms
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
    /// Convert from Pounds
    Pounds {
        /// The value to convert
        value: f64,
        /// Convert to Kilograms
        #[clap(short, long)]
        kg: bool,
    },
}
