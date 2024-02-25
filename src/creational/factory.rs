// Vehicle is the product
trait Vehicle {
    fn deliver(&self, destination: String);
}

#[derive(Clone, Copy)]
pub struct Cargo {
    pub weight: u32,
    pub destination: String,
}

// Truck is a concrete product
pub struct Truck {
    brand: String,
    wheel: u8,
    fuel: u16,
}

// Ship is a concrete product
pub struct Ship {
    name: String,
    fuel: u16,
}

// order_truck is a factory function to create trucks
fn order_truck() -> Truck { 
    // This is just a placeholder
    Truck { brand: "Toyoda".to_string(), wheel: 4, fuel: 10000 }
}

// order_ship is a factory function to create ships
fn order_ship() -> Ship {
    // This is a placeholder
    Ship { name: "S.S Anne".to_string(), fuel: 20000 }
}


impl Vehicle for Truck {
    fn deliver(&self, destination: String) {
        println!("Delivering cargo by truck to {}", destination);
    }
}

impl Vehicle for Ship {
    fn deliver(&self, destination: String) {
        println!("Shipping cargo across islands to {}", destination);
    }
}

// a dummy function to determine if it's possible to deliver cargo just by truck
fn is_on_java(destination: String) -> bool {
    return destination.contains("Java")
}


// deliver_goods is the client, which will pick suitable vehicle based on cargo destination
pub fn deliver_cargo(cargos: Vec<Cargo>) {
    for cargo in cargos.iter() {
        if is_on_java(cargo.destination) {
            order_truck().deliver(cargo.destination);
        } else {
            order_truck().deliver("nearest harbor".to_string());
            order_ship().deliver(cargo.destination);
            order_truck().deliver(cargo.destination);
        }
    }
}
