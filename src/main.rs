use chrono::{NaiveDate, Datelike, Local};
use crossterm::terminal::{Clear, ClearType};
use serde::{Deserialize, Serialize};
use std::fs;

use terminal_size::{terminal_size, Width};
use crossterm::{cursor::MoveTo, cursor::Hide, ExecutableCommand};
use std::io;
use std::io::{stdout, Write};


#[derive(Serialize, Deserialize, Debug)]
struct Habit {
    name: String,
    streak: u32,
    history: Vec<String>, // store dates as YYYY-MM-DD
}

fn load_data(path: &str) -> io::Result<Vec<Habit>> {
    if let Ok(contents) = fs::read_to_string(path) {
        let habits: Vec<Habit> = serde_json::from_str(&contents).unwrap_or_default();
        Ok(habits)
    } else {
        Ok(Vec::new())
    }
}
/*
fn save_data(path: &str, habits: &Vec<Habit>) -> io::Result<()> {
    let json = serde_json::to_string_pretty(habits).unwrap();
    fs::write(path, json)
}

fn mark_habit(habits: &mut Vec<Habit>, name: &str) {
    let today = Local::now().date_naive();

    if let Some(habit) = habits.iter_mut().find(|h| h.name == name) {
        let today_str = today.to_string();
        if !habit.history.contains(&today_str) {
            habit.history.push(today_str);
            habit.streak += 1;
            println!("Habit '{}' marked! Streak: {}", habit.name, habit.streak);
        } else {
            println!("Habit '{}' is already marked today.", habit.name);
        }
    } else {
        println!("Habit not found.");
    }

fn add_habit(habits: &mut Vec<Habit>, name: &str) {
    habits.push(Habit {
        name: name.to_string(),
        streak: 0,
        history: Vec::new(),
    });

}
*/

fn load_habits(path: &str) -> Vec<Habit> {
    
    let habits = load_data(path).expect("Failed to load data");
    return habits;
}


fn print_graph(width: &u16) {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();
    for _y in 0..7 {    
            for _x in 0..width/2 {
                print!(" ");
            } print!("\n");
        }
}

fn print_habit(habit:&Habit, width:u16) {
    let mut stdout = stdout();
    let current_date = Local::now().date_naive();
    let current_week = current_date.iso_week().week();
    let current_weekday = current_date.weekday().number_from_monday();

    // Mark completed days
    for day in habit.history.iter().rev() {
        let date = NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap();
        let week = date.iso_week().week();
        let weekday = date.weekday().number_from_monday();

        let difference_week = current_week as i32 - week as i32;
        
        // compute using signed arithmetic so we can detect negative positions safely
        let calc_x = 2 * (width as i32 / 2) - 2 * difference_week - 2;
        if calc_x < 0 {
            break;
        }
        
        let position_x = calc_x as u16;
        let position_y = weekday as u16 -1;   
        
        stdout.execute(MoveTo(position_x, position_y)).unwrap();
        print!(" ");
    }
       
    // Remove upcoming days
    for i in current_weekday..8 {
        stdout.execute(MoveTo(2*(width/2)-2, i as u16)).unwrap();
        print!("  ");
    }
    
    
}


fn main() {
    
    let mut stdout = stdout();
    let width: u16;
    

    if let Some((Width(w), _)) = terminal_size() {
        print_graph(&w);
        width = w;
        
    } else {
       println!("Couldn't get terminal size.");
       std::process::exit(1);
    }

    
    let path = "/home/washington/Documents/habit-tracker/habits.json";
    let habits = load_habits(path);

    let habit = &habits[0];
    print_habit(habit, width);
    
    stdout.execute(MoveTo(0, 8)).unwrap();
    stdout.flush().unwrap();
    stdout.execute(Hide).unwrap();
    
}

