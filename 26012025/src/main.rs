mod vehicles;

use vehicles::{Bike, Brand, Car};

fn display_vehicles_details(vehicle: Box<dyn vehicles::VehicleDetails + '_>) {
    vehicle.display_details();
}

fn main() {
    let brand = Brand {
        name: "Yamaha".to_string(),
        _country: "Japan".to_string(),
    };

    let vehicles: Vec<Box<dyn vehicles::VehicleDetails>> = vec![
        Box::new(Bike::new(
            "Yamaha".to_string(),
            String::from("Yamaha 2020"),
            None,
            &brand,
        )),
        Box::new(Car::new(
            "Toyota".to_string(),
            String::from("Toyota 2020"),
            None,
        )),
    ];

    vehicles
        .into_iter()
        .for_each(|vehicle| display_vehicles_details(vehicle));
}
