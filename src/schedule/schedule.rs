use std::collections::LinkedList;
use serde::{Deserialize, Serialize};
use clokwerk::{Scheduler, TimeUnits, ScheduleHandle, Interval};
use clokwerk::Interval::*;
use std::time::Duration;
use mpd::{Client, Song};
use std::sync::Arc;
use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Deserialize,Serialize)]
pub struct ScheduleEntry {
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    hours: u8,
    minutes: u8,
    url: String,
    stop: bool
}

#[derive(Deserialize,Serialize,Clone)]
pub struct ScheduleStart {
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    hours: u8,
    minutes: u8,
    url: String,
    stop: bool
}

#[derive(Deserialize,Serialize)]
pub struct ScheduleStop {
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    hours: u8,
    minutes: u8,
}

pub struct CronSchedule {
    timer: Scheduler,
    items: LinkedList<ScheduleEntry>
}

fn days_for(thing: &ScheduleStart) -> LinkedList<Interval> {
    let mut items = LinkedList::new();
    if thing.sunday {
        items.push_back(Sunday)
    }
    if thing.monday {
        items.push_back(Monday)
    }
    if thing.tuesday {
        items.push_back(Tuesday)
    }
    if thing.wednesday {
        items.push_back(Wednesday)
    }
    if thing.thursday {
        items.push_back(Thursday)
    }
    if thing.friday {
        items.push_back(Friday)
    }
    if thing.saturday {
        items.push_back(Saturday)
    }
    return items
}

impl CronSchedule {
    pub fn new() -> CronSchedule {
        CronSchedule {
            timer: Scheduler::new(),
            items: LinkedList::new()
        }
    }

    pub fn add_schedule_start(&mut self, thing: &ScheduleStart) {
        let time = format!("{:02}:{:02}", thing.hours, thing.minutes);

        let days = days_for(thing);
        let first = self.timer.every(*(days.front().unwrap()));
        let sched = days.iter().skip(1).fold(first, |a, b| a.and_every(*b) );

        let url = thing.url.clone();

        sched.at(time.clone().as_str()).run(move || {
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
    }

    pub fn tick(&mut self) {
        self.timer.run_pending();
    }

}
