use std::fmt;

#[derive(Debug)]
pub enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
}

pub enum TodoStatus {
    NeedsAction,
    Completed,
    InProgress,
    Cancelled,
}

pub enum JournalStatus {
    Draft,
    Final,
    Cancelled,
}

pub struct Organizer {
    cn: String,
    dir: String,
    sent_by: String,
    lang: String,
    other: Vec<String>,
}

// NOTE Attendee and organizer params are all optional.
// Need to make the structs reflect that or add default vals.
#[derive(Debug)]
pub struct Attendee {
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

#[derive(Debug)]
pub struct Event {
    start: String,
    end: String,
    organizer: String,
    attendees: Vec<Attendee>,
    created: String,
    description: String,
    location: String,
    last_modified: String,
    status: EventStatus,
    summary: String,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: Option<String>,
    pub params: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct IcalObject {
    pub properties: Vec<Property>,
    pub events: Option<Event>,
    pub timezones: Vec<Property>,
    pub todos: Option<String>, // TODO implement todos ;)
    pub journals: Option<String>,
}

impl IcalObject {
    pub fn new() -> IcalObject {
        IcalObject {
            properties: Vec::new(),
            events: None,
            timezones: Vec::new(),
            journals: None,
            todos: None,
        }
    }
    pub fn add_property(&mut self, prop: Property) {
        &self.properties.push(prop);
    }
    pub fn add_event(&mut self, event: Event) {
        self.events = Some(event);
    }
}

// impl fmt::Debug for IcalObject {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self.properties);
//     }
// }
