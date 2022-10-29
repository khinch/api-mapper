use serde::{Serialize,Deserialize};
use crate::model::{Application, DataPoint};

#[derive(Serialize,Deserialize)]
pub struct Controller {
    system_name: String,
    data_points: Vec<DataPoint>,
    next_data_point_id: usize,
    applications: Vec<Application>,
    next_application_id: usize,
}

impl Controller {
    pub fn new(system_name: String) -> Self {
        Controller {
            system_name,
            data_points: Vec::new(),
            next_data_point_id: 1,
            applications: Vec::new(),
            next_application_id: 1,
        }
    }

    pub fn system_name(&self) -> &str {
        &self.system_name
    }

    pub fn change_name(&mut self, new_name: String) {
        self.system_name = new_name;
    }

    pub fn add_data_point(&mut self, name: String, description: String) {
        let data_point = DataPoint::new(self.next_data_point_id, name, description);
        self.data_points.push(data_point);
        self.next_data_point_id += 1;
    }

    pub fn add_application(&mut self, name: String, description: String) {
        let application = Application::new(self.next_application_id, name, description);
        self.applications.push(application);
        self.next_application_id += 1;
    }

    pub fn get_data_point_index(&self, id: usize) -> Option<usize> {
        self.data_points.iter().position(|d| d.id == id)
    }

    pub fn remove_data_point(&mut self, data_point_id: usize) -> Result<(), String> {
        let index = self.data_points.iter().position(|d| d.id == data_point_id);

        match index {
            Some(index) => {
                self.data_points.remove(index);
                return Ok(());
            }
            None => {
                return Err(format!(
                    "Data point with ID {} doesn't exist",
                    data_point_id
                ))
            }
        };
    }

    pub fn remove_application(&mut self, application_id: usize) -> Result<(), String> {
        let index = self.applications.iter().position(|a| a.id == application_id);

        match index {
            Some(index) => {
                self.applications.remove(index);
                return Ok(());
            }
            None => {
                return Err(format!(
                    "Application with ID {} doesn't exist",
                    application_id
                ))
            }
        };
    }

    pub fn get_data_points(&self) -> &Vec<DataPoint> {
        &self.data_points
    }

    pub fn get_applications(&self) -> &Vec<Application> {
        &self.applications
    }
}
