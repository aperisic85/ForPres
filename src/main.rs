struct MyGarage(Vec<Cars>);
struct Cars {
    name: String,
    car_type: CarType,
}

enum CarType {
    Sport,
    Road,
    Offroad,
}

impl Cars {
    fn new(name: String, car_type: CarType) -> Cars{
        Cars{name, car_type}
    }
}

impl MyGarage {
    fn new() -> Self {
        
        let mut myVec: Vec<Cars> = Vec::new();
        MyGarage(myVec)
    }
}

fn main() {
    let newCar = Cars::new(String::from("mercedes"), CarType::Sport);
    let newCar2 = Cars::new(String::from("bmw"), CarType::Road);
    let newCar3 = Cars::new(String::from("range rover"), CarType::Offroad);
    let mut newGarage = MyGarage::new();
    newGarage.0.push(newCar);
    newGarage.0.push(newCar2);
    newGarage.0.push(newCar3);

    for x in MyGarage {
        println!("{}",x);
    }

}
