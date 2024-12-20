use crate::models::Course;
use serde_json::{from_str, to_string_pretty};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

const FILE_PATH: &str = "courses.json";

pub fn add_course(course_name: &str) {
    let mut courses = load_courses();
    if courses.iter().any(|c| c.name == course_name) {
        println!("Course '{}' already exists!", course_name);
        return;
    }
    courses.push(Course::new(course_name.to_string()));
    save_courses(&courses);
    println!("Course '{}' added.", course_name);
}

pub fn increment_absence(course_name: &str) {
    let mut courses = load_courses();
    if let Some(index) = courses.iter().position(|c| c.name == course_name) {
        courses[index].absences += 1;
        save_courses(&courses);
        println!(
            "Absence for '{}' incremented. Total: {}",
            course_name, courses[index].absences
        );
    } else {
        println!("Course '{}' not found.", course_name);
    }
}


pub fn view_summary() {
    let courses = load_courses();
    if courses.is_empty() {
        println!("No courses tracked yet.");
    } else {
        println!("Course Summary:");
        for course in courses {
            println!("{}: {} gize ketekal", course.name, course.absences);
        }
    }
}

pub fn reset_courses() {
    save_courses(&Vec::new());
    println!("All courses have been reset.");
}

fn load_courses() -> Vec<Course> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Unable to open or create courses.json");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_default();
    if content.is_empty() {
        Vec::new()
    } else {
        from_str(&content).unwrap_or_else(|_| {
            println!("Corrupted data! Resetting file.");
            Vec::new()
        })
    }
}

fn save_courses(courses: &[Course]) {
    let json = to_string_pretty(courses).expect("Failed to serialize courses");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Unable to open courses.json");
    file.write_all(json.as_bytes()).expect("Failed to write to courses.json");
}
