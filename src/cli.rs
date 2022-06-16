use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    /// The conversion target
    #[clap(subcommand)]
    pub target: Target,
}

#[derive(Debug, Subcommand)]
pub enum Target {
    /// Convert a temperature to Celsius
    Celsius {
        /// Convert from Fahrenheit
        #[clap(short, long)]
        farenheit: Option<f64>,
        /// Convert from Kelvin
        #[clap(short, long)]
        kelvin: Option<f64>,
    },
    /// Convert a temperature to Kelvin
    Kelvin {
        /// Convert from Celsius
        #[clap(short, long)]
        celsius: Option<f64>,
        /// Convert from Fahrenheit
        #[clap(short, long)]
        fahrenheit: Option<f64>,
    },
    /// Convert a temperature to Fahrenheit
    Fahrenheit {
        /// Convert from Celsius
        #[clap(short, long)]
        celsius: Option<f64>,
        /// Convert from Kelvin
        #[clap(short, long)]
        kelvin: Option<f64>,
    },
}
