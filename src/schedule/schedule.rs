use std::collections::LinkedList;
use serde::{Deserialize, Serialize};
use clokwerk::{Scheduler, TimeUnits, ScheduleHandle, Interval};
use clokwerk::Interval::*;
use std::time::Duration;
use mpd::{Client, Song};
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;
use std::ops::Deref;
use actix_web::error::ReadlinesError::LimitOverflow;
use dirs::home_dir;
use serde_json::{to_writer, from_reader};
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone)]
pub struct Alarm {
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    start_hours: u8,
    start_minutes: u8,
    end_hours: u8,
    end_minutes: u8,
    url: String,
    id: Option<Uuid>,
}

pub struct CronSchedule {
    timer: Scheduler,
    alarms: Mutex<LinkedList<Alarm>>
}

fn days_for(alarm: &Alarm) -> LinkedList<Interval> {
    let mut items = LinkedList::new();
    if alarm.sunday {
        items.push_back(Sunday)
    }
    if alarm.monday {
        items.push_back(Monday)
    }
    if alarm.tuesday {
        items.push_back(Tuesday)
    }
    if alarm.wednesday {
        items.push_back(Wednesday)
    }
    if alarm.thursday {
        items.push_back(Thursday)
    }
    if alarm.friday {
        items.push_back(Friday)
    }
    if alarm.saturday {
        items.push_back(Saturday)
    }
    return items
}

impl CronSchedule {

    pub fn new() -> CronSchedule {
        CronSchedule {
            timer: Scheduler::new(),
            alarms: Mutex::new(LinkedList::new())
        }
    }

    fn save(&mut self) {
        let path = format!("{}/.rustyradio", home_dir().unwrap().to_str().unwrap());
        to_writer(&File::create(path).unwrap(), &self.alarms);
    }

    pub fn load(&mut self) {
        let path = format!("{}/.rustyradio", home_dir().unwrap().to_str().unwrap());
        let file = File::open(path);
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let data = from_reader(reader);
                self.alarms = data.unwrap();
                self.update_schedule();
            },
            Err(e) => println!("No initial data to load {}", e),
        }
    }

    fn update_schedule(&mut self) {
        self.save();
        self.timer = Scheduler::new();
        for item in self.alarms.lock().unwrap().iter() {
            let start_time = format!("{:02}:{:02}", item.start_hours, item.start_minutes);
            let end_time = format!("{:02}:{:02}", item.end_hours, item.end_minutes);
            println!("Sceduling for {} to {}", start_time, end_time);

            let days = days_for(item);
            if days.is_empty() {
                println!("No days selected - ignoring");
            } else {
                let first_start = self.timer.every(*(days.front().unwrap())).at(start_time.as_str());
                let sched_start = days.iter().skip(1).fold(first_start, |a, b| a.and_every(*b).at(start_time.as_str()));

                let url = item.url.clone();

                sched_start.run(move || {
                    let mut conn = Client::connect("localhost:6600").unwrap();
                    conn.clear().unwrap();
                    println!("Playing {}", url);
                    let song = Song {
                        file: url.to_string(),
                        name: None,
                        title: None,
                        last_mod: None,
                        duration: None,
                        place: None,
                        range: None,
                        tags: Default::default()
                    };
                    conn.push(song).unwrap();
                    conn.play().unwrap();
                });

                let first_end = self.timer.every(*(days.front().unwrap())).at(end_time.as_str());
                let sched_end = days.iter().skip(1).fold(first_end, |a, b| a.and_every(*b).at(end_time.as_str()));

                sched_end.run(move || {
                    let mut conn = Client::connect("localhost:6600").unwrap();
                    println!("Stopping");
                    conn.stop().unwrap();
                    conn.clear().unwrap();
                });
            }
        }
    }

    pub fn add_schedule(&mut self, thing: &Alarm) -> Uuid {
        let mut item = thing.clone();
        let id = Uuid::new_v4();
        item.id = Some(id);
        self.alarms.lock().unwrap().push_back(item);
        self.update_schedule();
        id.clone()
    }

    pub fn remove_schedule(&mut self, id: &String) -> bool {
        let before = self.alarms.lock().unwrap().len();
        let orig = self.alarms.lock().unwrap().clone();
        let matched = Some(Uuid::parse_str(id).unwrap());
        let result: LinkedList<Alarm> = orig.into_iter().filter(| item | item.id != matched).collect();
        self.alarms = Mutex::new(result.clone());
        let after = self.alarms.lock().unwrap().len();
        self.update_schedule();

        before > after
    }

    pub fn get_schedule(&mut self) -> LinkedList<Alarm> {
        self.alarms.lock().unwrap().clone()
    }

    pub fn tick(&mut self) {
        self.timer.run_pending();
    }

}
