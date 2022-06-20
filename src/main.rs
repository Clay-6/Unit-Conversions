mod cli;

use clap::Parser as _;
use cli::Args;
use unit_conversions::{
    celsius, fahrenheit, kelvin,
    length::{km_to_miles, miles_to_km},
    temperature::Temperature,
    weight::{kg_to_pounds, pounds_to_kg},
};

const NO_TARGET_ERR: &str = "No conversion target specified";

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    match args.source {
        cli::Source::Celsius {
            value,
            fahrenheit,
            kelvin,
        } => {
            if !fahrenheit && !kelvin {
                return Err(NO_TARGET_ERR);
            }

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
            if !celsius && !fahrenheit {
                return Err(NO_TARGET_ERR);
            }

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
            if !celsius && !kelvin {
                return Err(NO_TARGET_ERR);
            }

            let temp = fahrenheit!(value);

            if celsius {
                println!("{}", temp.to_celsius());
            }
            if kelvin {
                println!("{}", temp.to_kelvin())
            }
        }
        cli::Source::Kilometers { value, miles } => {
            if !miles {
                return Err(NO_TARGET_ERR);
            }

            if miles {
                println!("{}", km_to_miles(value));
            }
        }
        cli::Source::Miles { value, km } => {
            if !km {
                return Err(NO_TARGET_ERR);
            }

            if km {
                println!("{}", miles_to_km(value));
            }
        }
        cli::Source::Kilograms { value, lbs, grams } => {
            if !(lbs || grams) {
                return Err(NO_TARGET_ERR);
            }

            if lbs {
                println!("{}lb", kg_to_pounds(value));
            }
            if grams {
                println!("{}g", value * 1000.0);
            }
        }
        cli::Source::Pounds { value, kg } => {
            if !kg {
                return Err(NO_TARGET_ERR);
            }

            if kg {
                println!("{}kg", pounds_to_kg(value))
            }
        }
    }

    Ok(())
}
