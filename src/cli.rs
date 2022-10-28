use crate::controller::Controller;
use std::io;

pub fn menu() {
    let mut controller = Controller::new();

    loop {
        println!("**********************");
        println!("***** API Mapper *****");
        println!("**********************");
        println!("");
        println!("1: Add datapoint");
        println!("2: Delete datapoint");
        println!("3: List datapoints");
        println!("0: Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read from console");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("");
                println!("Add DataPoint: ");
                capture_datapoint(&mut controller);
            }
            2 => {
                println!("");
                println!("Delete DataPoint: ");
                delete_datapoint(&mut controller);
            }
            3 => {
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
            0 => {
                println!("Exiting ...");
                break;
            }
            _ => continue,
        }
        println!("");
    }
}

fn capture_datapoint(controller: &mut Controller) {
    let mut name = String::new();
    let mut description = String::new();

    println!("Enter datapoint name: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name from console");

    println!("Enter datapoint description: ");

    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read description from console");

    controller.add_data_point(name.trim().to_string(), description.trim().to_string());
}

fn delete_datapoint(controller: &mut Controller) {
    println!("Enter datapoint ID: ");
    loop {
        let mut id = String::new();

        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read ID from console");

        let id: usize = match id.trim().parse() {
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

fn pause() {
    println!("Press enter to return to menu ...");
    let mut useless = String::new();
    io::stdin()
        .read_line(&mut useless)
        .expect("Failed to read console input");
}
