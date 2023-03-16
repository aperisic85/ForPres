struct MyGarage(Vec<Cars>);
#[derive(Debug)]
struct Cars {
    name: String,
    car_type: CarType,
}
#[derive(Debug)]
enum CarType {
    Sport,
    Road,
    Offroad,
}

impl Cars {
    fn new(name: String, car_type: CarType) -> Cars {
        Cars { name, car_type }
    }
}

impl MyGarage {
    fn new() -> Self {
        let my_vec: Vec<Cars> = Vec::new();
        MyGarage(my_vec)
    }
}

impl IntoIterator for MyGarage {
    type Item = Cars;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn main() {
    let new_car = Cars::new(String::from("mercedes"), CarType::Sport);
    let new_car2 = Cars::new(String::from("bmw"), CarType::Road);
    let new_car3 = Cars::new(String::from("range rover"), CarType::Offroad);
    let mut new_garage = MyGarage::new();
    new_garage.0.push(new_car);
    new_garage.0.push(new_car2);
    new_garage.0.push(new_car3);

    for x in new_garage {
        println!("{:?}", x.name);
    }
}
