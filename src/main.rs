mod cli;

use clap::Parser as _;
use cli::Args;
use unit_conversions::{celsius, fahrenheit, kelvin, temperature::Temperature};

fn main() {
    let args = Args::parse();

    match args.target {
        cli::Source::Celsius {
            farenheit: fa,
            kelvin: kel,
        } => {
            if let Some(k) = kel {
                let t = celsius!(k);
                println!("{}", t.to_kelvin())
            }
            if let Some(f) = fa {
                let t = celsius!(f);
                println!("{}", t.to_fahrenheit());
            }
        }
        cli::Source::Kelvin {
            celsius: cel,
            fahrenheit: fa,
        } => {
            if let Some(c) = cel {
                let t = kelvin!(c);
                println!("{}", t.to_celsius())
            }
            if let Some(f) = fa {
                let t = kelvin!(f);
                println!("{}", t.to_fahrenheit())
            }
        }
        cli::Source::Fahrenheit {
            celsius: cel,
            kelvin: kel,
        } => {
            if let Some(c) = cel {
                let t = fahrenheit!(c);
                println!("{}", t.to_celsius());
            }
            if let Some(k) = kel {
                let t = fahrenheit!(k);
                println!("{}", t.to_kelvin());
            }
        }
    }
}
