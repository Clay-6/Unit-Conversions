mod cli;

use clap::Parser as _;
use cli::Args;
use unit_conversions::{
    celsius,
    distance::{km_to_miles, miles_to_km},
    fahrenheit, kelvin,
    temperature::Temperature,
};

fn main() {
    let args = Args::parse();
    match args.source {
        cli::Source::Celsius {
            value,
            fahrenheit,
            kelvin,
        } => {
            let temp = celsius!(value);

            if fahrenheit {
                println!("{}", temp.to_fahrenheit());
            }
            if kelvin {
                println!("{}", temp.to_kelvin());
            }
        }
        cli::Source::Kelvin {
            value,
            celsius,
            fahrenheit,
        } => {
            let temp = kelvin!(value);

            if celsius {
                println!("{}", temp.to_celsius());
            }
            if fahrenheit {
                println!("{}", temp.to_fahrenheit());
            }
        }
        cli::Source::Fahrenheit {
            value,
            celsius,
            kelvin,
        } => {
            let temp = fahrenheit!(value);

            if celsius {
                println!("{}", temp.to_celsius());
            }
            if kelvin {
                println!("{}", temp.to_kelvin())
            }
        }
        cli::Source::Kilometers { value, miles } => {
            if miles {
                println!("{}", km_to_miles(value));
            }
        }
        cli::Source::Miles { value, km } => {
            if km {
                println!("{}", miles_to_km(value));
            }
        }
    }
}
