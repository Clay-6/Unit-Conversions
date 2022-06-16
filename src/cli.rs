use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    /// The conversion source
    #[clap(subcommand)]
    pub target: Source,
}

#[derive(Debug, Subcommand)]
pub enum Source {
    /// Convert a temperature from Celsius
    Celsius {
        /// Convert to Fahrenheit
        #[clap(short, long)]
        farenheit: Option<f64>,
        /// Convert to Kelvin
        #[clap(short, long)]
        kelvin: Option<f64>,
    },
    /// Convert a temperature from Kelvin
    Kelvin {
        /// Convert to Celsius
        #[clap(short, long)]
        celsius: Option<f64>,
        /// Convert to Fahrenheit
        #[clap(short, long)]
        fahrenheit: Option<f64>,
    },
    /// Convert a temperature from Fahrenheit
    Fahrenheit {
        /// Convert to Celsius
        #[clap(short, long)]
        celsius: Option<f64>,
        /// Convert to Kelvin
        #[clap(short, long)]
        kelvin: Option<f64>,
    },
}
