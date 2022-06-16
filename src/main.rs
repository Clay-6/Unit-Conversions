mod cli;

use clap::Parser as _;
use cli::Args;
use unit_conversions::{celsius, fahrenheit, kelvin, temperature::Temperature};

fn main() {
    let args = Args::parse();

    match args.target {
        cli::Target::Celsius {
            farenheit: fa,
            kelvin: kel,
        } => {
            if let Some(k) = kel {
                let t = kelvin!(k);
                println!("{}", t.to_celsius())
            }
            if let Some(f) = fa {
                let t = fahrenheit!(f);
                println!("{}", t.to_celsius());
            }
        }
        cli::Target::Kelvin {
            celsius: cel,
            fahrenheit: fa,
        } => {
            if let Some(c) = cel {
                let t = celsius!(c);
                println!("{}", t.to_kelvin())
            }
            if let Some(f) = fa {
                let t = fahrenheit!(f);
                println!("{}", t.to_kelvin())
            }
        }
        cli::Target::Fahrenheit {
            celsius: cel,
            kelvin: kel,
        } => {
            if let Some(c) = cel {
                let t = celsius!(c);
                println!("{}", t.to_fahrenheit());
            }
            if let Some(k) = kel {
                let t = kelvin!(k);
                println!("{}", t.to_fahrenheit());
            }
        }
    }
}
