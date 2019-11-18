use std::process::Command;

mod entities;
mod parse;

fn alert(msg: String, context: Option<String>) {
    Command::new("notify-send")
        .arg("--icon=calendar")
        .arg(format!("{}", msg))
        .arg(format!("{}", context.unwrap_or(String::new())))
        .output()
        .expect("failed");
}

fn main() {
    parse::parse_file("./invite.ics").unwrap();
}
