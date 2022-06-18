use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
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
}
