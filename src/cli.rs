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
    /// Convert a temperature from Kelvin
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
    /// Convert a temperature from Fahrenheit
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
    /// Convert from Centimeters
    #[clap(alias = "cm")]
    Centimeters {
        /// The value to convert
        value: f64,
        /// Convert to inches
        #[clap(short, long)]
        inches: bool,
        /// Convert to Kilometers
        #[clap(short, long)]
        kilometers: bool,
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
    /// Convert from inches
    #[clap(alias = "in")]
    Inches {
        /// The value to convert
        value: f64,
        /// Convert to Centimeters
        centimeters: bool,
    },
    /// Convert from Miles
    #[clap(alias = "mi")]
    Miles {
        /// The value to convert
        value: f64,
        /// Convert to Kilometers
        #[clap(short, long)]
        km: bool,
    },
    /// Convert from Grams
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
    /// Convert from Ounces
    #[clap(alias = "oz")]
    Ounces {
        /// The value to convert
        value: f64,
        /// Convert to Grams
        #[clap(short, long)]
        grams: bool,
    },
    /// Convert from Pounds
    #[clap(alias = "lb")]
    Pounds {
        /// The value to convert
        value: f64,
        /// Convert to Kilograms
        #[clap(short, long)]
        kg: bool,
    },
}
