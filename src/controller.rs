use crate::model::{Application, DataPoint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

    pub fn add_data_point(&mut self, name: String, description: String) -> usize {
        let id = self.next_data_point_id;
        let data_point = DataPoint::new(id, name, description);
        self.data_points.push(data_point);
        self.next_data_point_id += 1;
        id
    }

    pub fn add_application(&mut self, name: String, description: String) -> usize {
        let id = self.next_application_id;
        let application = Application::new(id, name, description);
        self.applications.push(application);
        self.next_application_id += 1;
        id
    }

    pub fn get_data_point_index(&self, id: usize) -> Option<usize> {
        self.data_points.iter().position(|d| d.id == id)
    }

    pub fn remove_data_point(&mut self, data_point_id: usize) -> Result<usize, String> {
        let index = self.data_points.iter().position(|d| d.id == data_point_id);

        match index {
            Some(index) => Ok(self.data_points.remove(index).id),
            None => {
                return Err(format!(
                    "Data point with ID {} doesn't exist",
                    data_point_id
                ))
            }
        }
    }

    pub fn remove_application(&mut self, application_id: usize) -> Result<usize, String> {
        let index = self
            .applications
            .iter()
            .position(|a| a.id == application_id);

        match index {
            Some(index) => Ok(self.applications.remove(index).id),
            None => {
                return Err(format!(
                    "Application with ID {} doesn't exist",
                    application_id
                ))
            }
        }
    }

    pub fn get_data_points(&self) -> &Vec<DataPoint> {
        &self.data_points
    }

    pub fn get_applications(&self) -> &Vec<Application> {
        &self.applications
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_controller() {
        let system_name = "test controller";
        let controller = Controller::new(String::from(system_name));

        assert_eq!(controller.system_name, system_name);
        assert_eq!(controller.next_data_point_id, 1);
        assert_eq!(controller.next_application_id, 1);
        assert_eq!(controller.data_points.len(), 0);
        assert_eq!(controller.applications.len(), 0);
    }

    #[test]
    fn controller_get_system_name() {
        let system_name = "test controller";
        let controller = Controller::new(String::from(system_name));

        assert_eq!(controller.system_name(), system_name);
    }

    #[test]
    fn controller_update_system_name() {
        let system_name = "abcde";
        let updated_system_name = "fghij";
        let mut controller = Controller::new(String::from(system_name));
        controller.change_name(String::from(updated_system_name));

        assert_eq!(controller.system_name, updated_system_name);
    }

    #[test]
    fn add_datapoint() {
        let name = "test data point name";
        let description = "test data point description";
        let mut controller = Controller::new(String::from("test controller for data points"));
        let expected_data_point_id = controller.next_data_point_id;
        let initial_length = controller.data_points.len();
        let actual_data_point_id =
            controller.add_data_point(String::from(name), String::from(description));
        let data_point = &controller.data_points[initial_length];

        assert_eq!(actual_data_point_id, expected_data_point_id);
        assert_eq!(controller.next_data_point_id, expected_data_point_id + 1);
        assert_eq!(controller.data_points.len(), initial_length + 1);
        assert_eq!(data_point.id, expected_data_point_id);
        assert_eq!(data_point.name, name);
        assert_eq!(data_point.description, description);
    }

    #[test]
    fn add_application() {
        let name = "test application name";
        let description = "test application description";
        let mut controller = Controller::new(String::from("test controller for applications"));
        let expected_application_id = controller.next_application_id;
        let initial_length = controller.applications.len();
        let actual_application_id =
            controller.add_application(String::from(name), String::from(description));
        let application = &controller.applications[initial_length];

        assert_eq!(actual_application_id, expected_application_id);
        assert_eq!(controller.next_application_id, expected_application_id + 1);
        assert_eq!(controller.applications.len(), initial_length + 1);
        assert_eq!(application.id, expected_application_id);
        assert_eq!(application.name, name);
        assert_eq!(application.description, description);
    }

    #[test]
    fn remove_datapoint() {
        let mut controller = Controller::new(String::from("test controller for data points"));
        let first_data_point_name = "first";
        let first_data_point_description = "first data point";
        let third_data_point_name = "third";
        let third_data_point_description = "third data point";

        let first_data_point_id = controller.add_data_point(
            String::from(first_data_point_name),
            String::from(first_data_point_description),
        );
        let second_data_point_id =
            controller.add_data_point(String::from("second"), String::from("second data point"));
        let third_data_point_id = controller.add_data_point(
            String::from(third_data_point_name),
            String::from(third_data_point_description),
        );

        let before_delete_length = controller.data_points.len();
        let deleted_data_point_id_result = controller.remove_data_point(second_data_point_id);
        assert_eq!(deleted_data_point_id_result, Ok(second_data_point_id));
        let deleted_data_point_id_result = controller.remove_data_point(second_data_point_id);
        assert_eq!(
            deleted_data_point_id_result,
            Err(format!(
                "Data point with ID {} doesn't exist",
                second_data_point_id
            ))
        );
        assert_eq!(controller.data_points.len(), before_delete_length - 1);

        let index = controller
            .data_points
            .iter()
            .position(|a| a.id == first_data_point_id).unwrap();
        assert_eq!(controller.data_points[index].name, first_data_point_name);
        assert_eq!(controller.data_points[index].description, first_data_point_description);

        let index = controller
            .data_points
            .iter()
            .position(|a| a.id == third_data_point_id).unwrap();
        assert_eq!(controller.data_points[index].name, third_data_point_name);
        assert_eq!(controller.data_points[index].description, third_data_point_description);
    }

    #[test]
    fn remove_application() {
        let mut controller = Controller::new(String::from("test controller for applications"));
        let first_application_name = "first";
        let first_application_description = "first application";
        let third_application_name = "third";
        let third_application_description = "third application";

        let first_application_id = controller.add_application(
            String::from(first_application_name),
            String::from(first_application_description),
        );
        let second_application_id =
            controller.add_application(String::from("second"), String::from("second application"));
        let third_application_id = controller.add_application(
            String::from(third_application_name),
            String::from(third_application_description),
        );

        let before_delete_length = controller.applications.len();
        let deleted_application_id_result = controller.remove_application(second_application_id);
        assert_eq!(deleted_application_id_result, Ok(second_application_id));
        let deleted_application_id_result = controller.remove_application(second_application_id);
        assert_eq!(
            deleted_application_id_result,
            Err(format!(
                "Application with ID {} doesn't exist",
                second_application_id
            ))
        );
        assert_eq!(controller.applications.len(), before_delete_length - 1);

        let index = controller
            .applications
            .iter()
            .position(|a| a.id == first_application_id).unwrap();
        assert_eq!(controller.applications[index].name, first_application_name);
        assert_eq!(controller.applications[index].description, first_application_description);

        let index = controller
            .applications
            .iter()
            .position(|a| a.id == third_application_id).unwrap();
        assert_eq!(controller.applications[index].name, third_application_name);
        assert_eq!(controller.applications[index].description, third_application_description);
    }
}
