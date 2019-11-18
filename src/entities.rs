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

pub struct Event {
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

pub struct Property {
    name: String,
    params: Option<Vec<(String, Vec<String>)>>,
    value: Option<String>,
}

pub struct IcalObject {
    pub properties: Vec<Property>,
    pub events: Vec<Event>,
    pub timezones: Vec<Property>,
}

impl IcalObject {
    pub fn new() -> IcalObject {
        IcalObject {
            properties: Vec::new(),
            events: Vec::new(),
            timezones: Vec::new(),
        }
    }
    pub fn add_property(&mut self, prop: Property) {
        &self.properties.push(prop);
    }
    pub fn add_event(&mut self, event: Event) {
        &self.events.push(event);
    }
}
