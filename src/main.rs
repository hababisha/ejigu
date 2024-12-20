use clap::{Arg, Command};
mod commands;
mod models;
fn main() {
    let matches = Command::new("Ejigu")
        .version("1.0")
        .author("Babi <example@example.com>")
        .about("Tracks courses and absences")
        .subcommand(
            Command::new("add")
                .about("Adds a new course")
                .arg(
                    Arg::new("COURSE_NAME")
                        .help("Name of the course to add")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("absent")
                .about("Increments absence count for a course")
                .arg(
                    Arg::new("COURSE_NAME")
                        .help("Name of the course to increment absences")
                        .required(true),
                ),
        )
        .subcommand(Command::new("summary").about("Displays summary of all courses"))
        .subcommand(Command::new("reset").about("Resets all courses"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let course_name = sub_matches.get_one::<String>("COURSE_NAME").unwrap();
            commands::add_course(course_name);
        }
        Some(("absent", sub_matches)) => {
            let course_name = sub_matches.get_one::<String>("COURSE_NAME").unwrap();
            commands::increment_absence(course_name);
        }
        Some(("summary", _)) => {
            commands::view_summary();
        }
        Some(("reset", _)) => {
            commands::reset_courses();
        }
        _ => {
            eprintln!("Unknown command. Use --help for a list of commands.");
        }
    }
}
