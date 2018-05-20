extern crate palette;
extern crate serde_json;

use palette::rgb::Rgb;
use palette::{Lab, Srgb};
use std::collections::HashMap;
use std::env;
use std::f32;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::Range;

type Success = ();

#[derive(Debug)]
enum Error {
    JsonError(serde_json::Error),
    IoError(io::Error),
    ParsingError(ParsingError),
    NoParent(),
    NoConfigFileSpecified(),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonError(error)
    }
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}
impl From<ParsingError> for Error {
    fn from(error: ParsingError) -> Self {
        Error::ParsingError(error)
    }
}

fn main() -> Result<Success, Error> {
    let config_file_path = env::args().nth(1).ok_or(Error::NoConfigFileSpecified())?;
    let mut config_file = File::open(config_file_path)?;
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;
    let parents = parents_from_config(&config)?;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let input = stdin.lock();
    let mut output = stdout.lock();

    let color_processor = input.lines().map(|line| {
        line.map_err(Error::IoError)
            .and_then(|input_color| parse_color(&input_color).map_err(Error::ParsingError))
            .map(|input_rgb| perceptually_nearest(input_rgb, &parents))
            .and_then(|parent| {
                parent.ok_or(Error::NoParent()).and_then(|(_, color)| {
                    output
                        .write(format!("{}\n", hex_triplet(color)).as_bytes())
                        .map_err(Error::IoError)
                        .and_then(|_| output.flush().map_err(Error::IoError))
                })
            })
    });

    color_processor.collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct ColorName<'a>(&'a str);

fn parents_from_config(config: &str) -> Result<HashMap<ColorName, Rgb>, Error> {
    let config: HashMap<&str, &str> = serde_json::from_str(config)?;

    let mut parents = HashMap::new();
    for (name, color) in config {
        let rgb = parse_color(&color)?;
        parents.insert(ColorName(&name), rgb);
    }

    Ok(parents)
}

fn perceptually_nearest<'a>(
    input_color: Rgb,
    parents: &'a HashMap<ColorName<'a>, Rgb>,
) -> Option<(ColorName<'a>, Rgb)> {
    let input_color_lab: Lab = Lab::from(input_color);
    let lowest_distance = parents
        .iter()
        .map(|(name, parent_color)| (name, Lab::from(*parent_color)))
        .map(|(name, parent_color_lab)| {
            let distance = [
                input_color_lab.l - parent_color_lab.l,
                input_color_lab.a - parent_color_lab.a,
                input_color_lab.b - parent_color_lab.b,
            ].iter()
                .map(|n| n.powi(2))
                .sum::<f32>();

            (name, distance)
        })
        .fold(None, |winner, candidate| match winner {
            None => Some(candidate),
            Some(winner) if candidate.1 < winner.1 => Some(candidate),
            _ => winner,
        });

    lowest_distance.and_then(|winner| {
        let parent_name = winner.0;
        parents.get(parent_name).map(|color| (*parent_name, *color))
    })
}

fn hex_triplet(rgb: Rgb) -> String {
    let red = (rgb.red * 255.0) as u8;
    let green = (rgb.green * 255.0) as u8;
    let blue = (rgb.blue * 255.0) as u8;
    format!("{:02x}{:02x}{:02x}", red, green, blue)
}

#[derive(Debug)]
enum ParsingError {
    ParsingError,
    InputTooLong,
}

fn parse_color(hex_triplet: &str) -> Result<Rgb, ParsingError> {
    if hex_triplet.len() > 6 {
        Err(ParsingError::InputTooLong)
    } else {
        let hex_to_u8 = |range: Range<usize>| {
            // It's safe to slice on byte boundaries here since from_str_radix only accepts
            // ascii codepoints anyway.
            hex_triplet
                .get(range)
                .and_then(|hex_slice| u8::from_str_radix(hex_slice, 16).ok())
        };
        match (hex_to_u8(0..2), hex_to_u8(2..4), hex_to_u8(4..6)) {
            (Some(red), Some(green), Some(blue)) => Ok(Srgb::new_u8(red, green, blue)),
            _ => Err(ParsingError::ParsingError),
        }
    }
}
