pub struct Brand {
    pub name: String,
    pub _country: String,
}

pub struct Bike<'a> {
    pub name: String,
    pub _model: String,
    pub _price: Option<f64>,
    pub brand: &'a Brand,
}

impl<'a> Bike<'a> {
    pub fn new(name: String, model: String, price: Option<f64>, brand: &Brand) -> Bike {
        Bike {
            name,
            _model: model,
            _price: price,
            brand,
        }
    }
}

pub struct Car {
    pub name: String,
    pub _model: String,
    pub _price: Option<f64>,
    pub top_speed: f64,
    pub _capacity: u8,
}

impl Car {
    pub fn new(name: String, model: String, price: Option<f64>) -> Car {
        Car {
            name,
            _model: model,
            _price: price,
            top_speed: 200.0,
            _capacity: 5,
        }
    }
}

pub struct Plane {
    pub name: String,
    pub _model: String,
    pub _price: Option<f64>,
    pub max_speed: f64,
    pub _max_altitude: f64,
}

pub trait VehicleDetails {
    fn max_speed(&self) -> f64;
    fn display_details(&self);
}

impl<'a> VehicleDetails for Bike<'a> {
    fn max_speed(&self) -> f64 {
        50.0
    }

    fn display_details(&self) {
        println!(
            "Bike details: {}. top speed {} km/h; Brand: {}",
            self.name,
            self.max_speed(),
            self.brand.name
        );
    }
}

impl VehicleDetails for Car {
    fn max_speed(&self) -> f64 {
        self.top_speed
    }

    fn display_details(&self) {
        println!(
            "Car details: {}. top speed {} km/h",
            self.name,
            self.max_speed()
        );
    }
}

impl VehicleDetails for Plane {
    fn max_speed(&self) -> f64 {
        self.max_speed
    }

    fn display_details(&self) {
        println!(
            "Plane details: {}. top speed {} km/h",
            self.name,
            self.max_speed()
        );
    }
}
