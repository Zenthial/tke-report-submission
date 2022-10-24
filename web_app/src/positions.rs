use std::{collections::HashMap, fs::File, io::Result};

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};

#[derive(Deserialize, Serialize, Clone)]
pub struct Report {
    report: String,
    filled: bool,
}

#[derive(Hash, Deserialize, Serialize, Eq, Clone)]
pub struct Position {
    position: String,
    sort_order: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.sort_order == other.sort_order
    }
}

pub fn get_positions() -> Vec<String> {
    vec![
        "Prytanis",
        "Epiprytanis",
        "Grammateus",
        "Crysophylos",
        "Histor",
        "Hypophetes",
        "Pylortes",
        "Hegemon",
        "Rush",
        "Community Service",
        "Dorms",
        "Last Blast",
        "Philanthropy",
        "Public Relations",
        "Programming",
        "Red Carnation Ball (RCB)",
        "Relay For Life",
        "Social",
        "Sports",
        "Party",
        "Quartermaster",
        "Christmas Party",
        "IFC",
        "Sweethearts",
        "Restoration",
        "Jump",
        "Diversity",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn get_reports() -> Result<HashMap<String, (Position, Report)>> {
    let reports: HashMap<String, (Position, Report)> = from_reader(&File::open("reports.json")?)?;

    Ok(reports)
}

pub fn write_report(position: String, report: String) -> Result<()> {
    let mut reports = get_reports().unwrap();
    let (position_struct, mut report_struct) = match reports.get(&position) {
        Some((pos, rep)) => (pos.clone(), rep.clone()),
        None => panic!("this shouldn't happen"),
    };

    if report_struct.filled != true {
        report_struct.report = report;
        report_struct.filled = true;

        reports.insert(position, (position_struct, report_struct));
        to_writer(&File::create("reports.json")?, &reports)?;
    };

    Ok(())
}
