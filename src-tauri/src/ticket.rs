use core::fmt::{Display, Formatter, Result};
use home::{self, home_dir};
use std::{
    fs::{create_dir, read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
use tauri::command;

struct Ticket {
    id: u16,
    region_type: String,
    date: String,
    station1: String,
    station2: String,
    price: u16,
    distance: u16,
    number: String,
}

impl Display for Ticket {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{},{},{},{},P,{},{},{},{}",
            self.id,
            self.region_type,
            self.date,
            self.number,
            self.station1,
            self.station2,
            self.distance,
            self.price
        )
    }
}

fn get_dir() -> PathBuf {
    let homedir_result = home_dir();
    let homedir = match homedir_result {
        Some(path) => path,
        None => panic!("could not find the home dir"),
    };
    let filename_dir = "tickets";
    let path = homedir.join("Desktop").join(filename_dir);
    let path_exists: bool = Path::new(&path).is_dir();
    if !path_exists {
        create_dir(&path).unwrap();
    }
    return path;
}

fn get_filename(station1: &str, station2: &str) -> String {
    let station1_uppercase_string = station1.to_uppercase();
    let station1_uppercase_str = station1_uppercase_string.as_str();
    let station2_uppercase_string = station2.to_uppercase();
    let station2_uppercase_str = station2_uppercase_string.as_str();

    let filename = if station1 < station2 {
        format!(
            "tickets_{}_{}.csv",
            station1_uppercase_str, station2_uppercase_str
        )
    } else {
        format!(
            "tickets_{}_{}.csv",
            station2_uppercase_str, station1_uppercase_str
        )
    };
    let filename_path = get_dir().join(&filename);
    let filename_path_string: String = filename_path.into_os_string().into_string().unwrap();
    filename_path_string
}

#[command]
pub fn get_all_tickets(station1: &str, station2: &str) -> String {
    let filename = get_filename(station1, station2);
    let file_content_result = read_to_string(filename);
    let file_content: String = match file_content_result {
        Ok(file) => file,
        Err(..) => "".to_owned(),
    };
    file_content
}

#[command]
pub fn remove_last_ticket(station1: &str, station2: &str) {
    let filename = get_filename(station1, station2);
    let file_content = read_to_string(filename).unwrap();

    let mut lines = file_content.split("\n").collect::<Vec<&str>>();
    if lines.len() < 2 {
        return;
    };
    lines.pop();
    lines.pop();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_filename(station1, station2))
        .unwrap();

    file.write_all(b"").unwrap();

    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }
}

pub fn create_ticket_file(station1: &str, station2: &str) {
    let filename = get_filename(station1, station2);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(filename)
        .unwrap();

    writeln!(
        file,
        "nr,type,date,ticket_number,Tip,Departure,Destination,distance,price"
    )
    .unwrap();
}

fn get_ticket_id(station1: &str, station2: &str) -> u16 {
    let filename = get_filename(station1, station2);
    let file_content_result = read_to_string(filename);
    let file_content = match file_content_result {
        Ok(file) => file,
        Err(..) => return 1,
    };
    let lines = file_content.split("\n").collect::<Vec<&str>>();
    (lines.len() as u16) - 1
}

fn convert_data(date: &str) -> String {
    let date_split = date.split("-").collect::<Vec<&str>>();
    format!("{}-{}-{}", date_split[2], date_split[1], date_split[0])
}

#[command]
#[allow(non_snake_case)]
pub fn create_ticket(
    regionType: &str,
    date: &str,
    station1: &str,
    station2: &str,
    price: u16,
    distance: u16,
    number: &str,
) {
    let filename = get_filename(station1, station2);

    let file_exists = Path::new(&String::from(filename)).exists();

    if !file_exists {
        create_ticket_file(station1, station2);
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(get_filename(station1, station2))
        .unwrap();

    let ticket = Ticket {
        id: get_ticket_id(station1, station2),
        region_type: regionType.to_string(),
        date: convert_data(date),
        station1: station1.to_string(),
        station2: station2.to_string(),
        price,
        distance,
        number: number.to_string(),
    };
    writeln!(file, "{}", ticket).unwrap();
}
