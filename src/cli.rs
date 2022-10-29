use crate::controller::Controller;
use serde_json;
use std::fs::File;
use std::io::{self, Write};

pub fn main_menu() {
    let mut controller = Controller::new(String::from("New System"));

    loop {
        println!("");
        println!("**********************************");
        println!("***** API Mapper - Main Menu *****");
        println!("**********************************");
        println!("System Name: {}", controller.system_name());
        println!("");
        println!("1: Load from file");
        println!("2: Save to file");
        println!("3: Change system name");
        println!("4: New system");
        println!("5: System menu");
        println!("0: Exit");

        let choice = read_from_console();

        let choice: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("");
                println!("Enter filename: ");
                let filename = read_from_console();
                match load_from_file(&filename) {
                    Ok(c) => controller = c,
                    Err(e) => println!("{}: {}", filename, e),
                }
                println!("Load successful: {}", filename);
                pause();
            }
            2 => {
                println!("");
                save_to_file(&controller);
                pause();
            }
            3 => {
                println!("");
                change_system_name(&mut controller);
            }
            4 => {
                println!("");
                println!("Starting new system: ");
                controller = Controller::new(String::from("New System"));
                pause();
            }
            5 => {
                system_menu(&mut controller);
            }
            0 => {
                println!("Exiting ...");
                break;
            }
            _ => continue,
        }
        println!("");
    }
}

fn system_menu(controller: &mut Controller) {
    loop {
        println!("**********************************");
        println!("********** System Menu ***********");
        println!("**********************************");
        println!("");
        println!("1: Manage datapoints");
        println!("2: Manage applications");
        println!("0: Main menu");

        let choice = read_from_console();

        let choice: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("");
                datapoint_menu(controller);
            }
            2 => {
                println!("");
                application_menu(controller);
            }
            0 => {
                break;
            }
            _ => continue,
        }
        println!("");
    }
}

fn datapoint_menu(controller: &mut Controller) {
    loop {
        println!("**********************************");
        println!("********* Datapoint Menu *********");
        println!("**********************************");
        println!("");
        println!("1: List datapoints");
        println!("2: Add datapoint");
        println!("3: Delete datapoint");
        println!("0: System menu");

        let choice = read_from_console();

        let choice: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("");
                println!("DataPoints: ");

                for d in controller.get_data_points() {
                    println!(
                        "Name: {}. Description: {}. ID: {}",
                        d.name, d.description, d.id
                    );
                }
                pause();
            }
            2 => {
                println!("");
                println!("Add DataPoint: ");
                capture_datapoint(controller);
            }
            3 => {
                println!("");
                println!("Delete DataPoint: ");
                delete_datapoint(controller);
            }
            0 => {
                break;
            }
            _ => continue,
        }
        println!("");
    }
}

fn application_menu(controller: &mut Controller) {
    loop {
        println!("**********************************");
        println!("******** Application Menu ********");
        println!("**********************************");
        println!("");
        println!("1: List applications");
        println!("2: Add application");
        println!("3: Delete application");
        println!("0: System menu");

        let choice = read_from_console();

        let choice: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("");
                println!("Applications: ");

                for a in controller.get_applications() {
                    println!(
                        "Name: {}. Description: {}. ID: {}",
                        a.name, a.description, a.id
                    );
                }
                pause();
            }
            2 => {
                println!("");
                println!("Add Application: ");
                capture_application(controller);
            }
            3 => {
                println!("");
                println!("Delete Application: ");
                delete_application(controller);
            }
            0 => {
                break;
            }
            _ => continue,
        }
        println!("");
    }
}

fn save_to_file(controller: &Controller) {
    println!("Enter filename: ");
    let filename = read_from_console();
    let mut file = File::create(filename).unwrap();

    let serialised = serde_json::to_string_pretty(&controller).unwrap();

    file.write(serialised.as_bytes()).unwrap();
}

fn load_from_file(filename: &str) -> Result<Controller, String> {
    
    let serialised = std::fs::read_to_string(filename);
    let serialised = match serialised {
        Ok(s) => s, 
        Err(err) => return Err(err.to_string()), 
    };

    let controller = serde_json::from_str::<Controller>(&serialised);
    match controller {
        Ok(c) => return Ok(c),
        Err(err) => return Err(err.to_string()),
    }
}

fn capture_datapoint(controller: &mut Controller) {
    println!("Enter datapoint name: ");
    let name = read_from_console();

    println!("Enter datapoint description: ");
    let description = read_from_console();

    controller.add_data_point(name, description);
}

fn capture_application(controller: &mut Controller) {
    println!("Enter application name: ");
    let name = read_from_console();

    println!("Enter application description: ");
    let description = read_from_console();

    controller.add_application(name, description);
}

fn delete_datapoint(controller: &mut Controller) {
    println!("Enter datapoint ID: ");
    loop {
        let id = read_from_console();

        let id: usize = match id.parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Invalid ID: {}", error);
                break;
            }
        };

        let result = controller.remove_data_point(id);

        match result {
            Ok(_) => {
                println!("Deleted data point with ID: {}", id);
            }
            Err(msg) => {
                println!("{}", msg);
            }
        };

        break;
    }

    pause();
}

fn delete_application(controller: &mut Controller) {
    println!("Enter application ID: ");
    loop {
        let id = read_from_console();

        let id: usize = match id.parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Invalid ID: {}", error);
                break;
            }
        };

        let result = controller.remove_application(id);

        match result {
            Ok(_) => {
                println!("Deleted application with ID: {}", id);
            }
            Err(msg) => {
                println!("{}", msg);
            }
        };

        break;
    }

    pause();
}

fn change_system_name(controller: &mut Controller) {
    println!("Enter new system name: ");
    controller.change_name(read_from_console());
}

fn pause() {
    println!("Press enter to return to menu ...");
    let _useless = read_from_console();
}

fn read_from_console() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from console");

    input.trim().to_string()
}
