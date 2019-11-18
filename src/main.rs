use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

const VALUE_DELIMITER: char = ':';
const PARAM_DELIMITER: char = ';';

fn alert(msg: String, context: Option<String>) {
    Command::new("notify-send")
        .arg("--icon=calendar")
        .arg(format!("{}", msg))
        .arg(format!("{}", context.unwrap_or(String::new())))
        .output()
        .expect("failed");
}

enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
}

enum TodoStatus {
    NeedsAction,
    Completed,
    InProgress,
    cancelled,
}

enum JournalStatus {
    Draft,
    Final,
    Cancelled,
}

struct Organizer {
    cn: String,
    dir: String,
    sent_by: String,
    lang: String,
    other: Vec<String>,
}

// NOTE Attendee and organizer params are all optional.
// Need to make the structs reflect that or add default vals.
struct Attendee {
    cutype: String,
    member: String,
    role: String,
    partstat: String,
    rsvp: String,
    delto: String,
    delfrom: String,
    sentby: String,
    cn: String,
    dir: String,
    lang: String,
    other: Vec<String>,
}

struct Event {
    start: String,
    end: String,
    organizer: String,
    attendees: Vec<String>, //TODO better struct
    created: String,
    description: String,
    location: String,
    last_modified: String,
    status: EventStatus,
    summary: String,
}

struct Property {
    name: String,
    params: Option<Vec<(String, Vec<String>)>>,
    value: Option<String>,
}

struct IcalObject {
    properties: Vec<Property>,
    events: Vec<Event>,
    timezones: Vec<Property>,
}

impl IcalObject {
    fn new() -> IcalObject {
        IcalObject {
            properties: Vec::new(),
            events: Vec::new(),
            timezones: Vec::new(),
        }
    }
    fn add_property(&mut self, prop: Property) {
        &self.properties.push(prop);
    }
    fn add_event(&mut self, event: Event) {
        &self.events.push(event);
    }
}

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

fn parse_file(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    let calendar = IcalObject::new();
    file.read_to_string(&mut contents)?;
    contents.lines().for_each(|line| {
        // TODO construct a calendar struct by walking the lines
        let vals: Vec<&str> = line.split(VALUE_DELIMITER).collect();
        // Match all colon separated values
        match vals[0].as_ref() {
            "ATTENDEE" => println!("{} is an attendee", vals[1]),
            "DTSTART" | "DTEND" => {
                println!("The event begins {}", parse_time(vals[1]));
            }
            "DTSTAMP" => alert(format!("Invite sent {}", parse_time(vals[1])), None),
            "CREATED" => println!("Event created {}", parse_time(vals[1])),
            "SUMMARY" => alert(format!("New Event {}", vals[1]), None),
            _ => {}
        }
    });
    Ok(())
}

fn main() {
    parse_file("./invite.ics").unwrap();
}
