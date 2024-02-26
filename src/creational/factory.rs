// Factory method pattern is a creational design pattern to simplify object creation.
// A factory method creates products to be used by clients.
//
// In this example, you are running a trustworthy transport company that boast
// an armada of trucks to transport cargo intra-island as fast as possible. In recent years,
// you want to expand your business to handle cross-island shippings. To ship across the seas,
// your company need ships as there are no trucks that can run in the bottom of the ocean.
//
// Previously, your company directly deal with a well-known truck manufacturer (constructor). Although
// the process is quite complex, it's something that your company can handle by itself.
// However, you found out that dealing ships (constructor) are way more complex than trucks. Dealing with
// ship manufacturer directly is not something your company can do anymore.
//
// To adhere this problem, your company contacts a vehicle dealer (factory) that delivers
// correct vehicle as your company requested. 

// Vehicle is the product
trait Vehicle {
    fn deliver(&self, destination: String);
}

pub struct Cargo {
    pub weight: u32,
    pub destination: String,
}

// Truck is a concrete product. Logically, a Truck is not something a non-specialist can assemble alone.
#[allow(dead_code)]
pub struct Truck {
    brand: String,
    wheel: u8,
    fuel: u16,
}

impl Vehicle for Truck {
    fn deliver(&self, destination: String) {
        println!("Delivering cargo by truck to {}", destination);
    }
}

// Ship is a concrete product. Same as Truck, a Ship isn't something a non-specialist can assemble alone.
#[allow(dead_code)]
pub struct Ship {
    name: String,
    fuel: u16,
}

impl Vehicle for Ship {
    fn deliver(&self, destination: String) {
        println!("Shipping cargo across islands to {}", destination);
    }
}

// VehicleDealer is a factory that simplifies vehicle creation
struct VehicleDealer;
impl VehicleDealer {
    fn is_on_java(&self, destination: String) -> bool {
        // Let's just say that your company is based in Java
        destination.contains("Java")
    }

    pub fn get_suitable_vehicle(&self, destination: String) -> Box<dyn Vehicle> {
        if self.is_on_java(destination) {
            // Let's just assume that this is hard to manufacture
            return Box::new(Truck { brand: "Toyoda".to_string(), wheel: 4, fuel: 10000 });
        }
        
        // Let's just assume that this is hard to manufacture
        Box::new(Ship{ name: "S.S Anne".to_string(), fuel: 20000 })
    }
}

// deliver_goods is the client, which will pick suitable vehicle based on cargo destination
pub fn deliver_cargo(cargos: Vec<Cargo>) {
    let dealer = VehicleDealer{};

    for cargo in cargos.iter() {
        dealer.get_suitable_vehicle(cargo.destination.clone()).deliver(cargo.destination.clone());
    }
}
