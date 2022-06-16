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
        #[clap(short, long)]
        farenheit: Option<f64>,
        #[clap(short, long)]
        kelvin: Option<f64>,
    },
    /// Convert a temperature to kelvinK
    Kelvin {
        #[clap(short, long)]
        celsius: Option<f64>,
        #[clap(short, long)]
        fahrenheit: Option<f64>,
    },
    // Convert a temperature to Fahrenheit
    Fahrenheit {
        #[clap(short, long)]
        celsius: Option<f64>,
        #[clap(short, long)]
        kelvin: Option<f64>,
    },
}
