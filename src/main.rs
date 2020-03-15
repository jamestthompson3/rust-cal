use std::collections::HashMap;
use std::process::Command;

mod entities;
mod parse;

fn alert(msg: Option<&String>, context: Option<&String>) {
    Command::new("notify-send")
        .arg("--icon=calendar")
        .arg(format!("{}", msg.unwrap()))
        .arg(format!("{}", context.unwrap_or(&String::new())))
        .output()
        .expect("failed");
}

fn main() {
    let cal = parse::parse_file("./reocurr.ics").unwrap();
    let mut notification = HashMap::new();
    for prop in cal.properties {
        match prop.name.as_str() {
            "SUMMARY" => {
                notification.insert("summary".to_string(), prop.value);
            }
            "LOCATION" => {
                notification.insert("location".to_string(), prop.value);
            }
            "DTSTAMP" => {
                notification.insert(
                    "time".to_string(),
                    Some(parse::parse_time(&prop.value.unwrap())),
                );
            }
            _ => {}
        }
    }
    alert(
        Some(&format!(
            "{}     {}",
            notification.get("summary").unwrap().as_ref().unwrap(),
            notification.get("time").unwrap().as_ref().unwrap()
        )),
        notification.get("location").unwrap().as_ref(),
    );
}
