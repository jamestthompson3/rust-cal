use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;

use crate::entities::{IcalObject, Property};

const VALUE_DELIMITER: char = ':';
const PARAM_DELIMITER: char = ';';

pub fn parse_time(timestring: &str) -> String {
    let year = &timestring[..4];
    let month = &timestring[4..6];
    let day = &timestring[6..8];
    let offsethour = &timestring[8..11];
    let offsetmin = &timestring[11..13];
    let offsetsecs = &timestring[13..];
    let utc = format!(
        "{}-{}-{}{}:{}:{}",
        year, month, day, offsethour, offsetmin, offsetsecs
    );
    let parsed = utc.parse::<DateTime<Local>>().unwrap();
    parsed.format("%a %b %e %T %Y").to_string()
}

pub fn parse_file(path: &str) -> std::io::Result<IcalObject> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    let mut calendar = IcalObject::new();
    let mut event_properties: Vec<Property> = Vec::new();
    file.read_to_string(&mut content)?;
    let lines = content.lines();
    let mut cal_parsing = false;
    let mut event_parsing = false;
    lines.for_each(|line| {
        match line {
            "BEGIN:VCALENDAR" => cal_parsing = true,
            "BEGIN:VEVENT" => event_parsing = true,
            "END:VEVENT" => event_parsing = false,
            "END:VCALENDAR" => cal_parsing = false,
            _ => {}
        };
        // each line is a property
        // TODO currently things like https:// get split to https, //
        let vals: Vec<&str> = line.split(VALUE_DELIMITER).collect();
        if cal_parsing && !event_parsing {
            calendar.add_property(Property {
                name: String::from(vals[0]),
                value: Some(String::from(vals[1])),
                params: None,
            })
        }
        if cal_parsing && event_parsing {
            // Match all colon separated values
            // create event outside of for loop
            match vals.len() {
                2 => event_properties.push(Property {
                    name: vals[0].to_string(),
                    value: Some(vals[1].to_string().trim().to_string()),
                    params: None,
                }),
                1 => {
                    if vals[0].to_string().starts_with(" ") {
                        let prev_prop = event_properties.pop().unwrap();
                        event_properties.push(Property {
                            value: Some(format!(
                                "{}{}",
                                &prev_prop.value.unwrap(),
                                vals[0].to_string().trim_start().to_string()
                            )),
                            ..prev_prop
                        })
                    }
                }
                _ => {}
            }
        }
    });
    for prop in event_properties {
        calendar.properties.push(prop);
    }
    Ok(calendar)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_parse_time() {
        assert_eq!(parse_time("20191116T142754Z"), "Sat Nov 16 16:27:54 2019");
    }
}
