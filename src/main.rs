use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn parse_time(timestring: &str) -> String {
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

fn read_file(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents.lines().for_each(|line| {
        let vals: Vec<&str> = line.split(":").collect();
        println!("{:?}", vals);
        // Match all colon separated values
        match vals[0].as_ref() {
            "ATTENDEE" => println!("{} is an attendee", vals[1]),
            "DTSTART" | "DTEND" => {
                println!("The event begins {}", parse_time(vals[1]));
            }
            "DTSTAMP" => println!("Invite sent {}", parse_time(vals[1])),
            "CREATED" => println!("Event created {}", parse_time(vals[1])),
            "SUMMARY" => println!("New Event {}", vals[1]),
            _ => {}
        }
    });
    Ok(())
}

fn main() {
    read_file("./invite.ics").unwrap();
}
