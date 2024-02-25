mod creational;

use creational::factory::deliver_cargo;

use crate::creational::factory::Cargo;

fn main() {
    let cargos = vec![
        Cargo{ weight: 123, destination: "Somewhere, Sumatra, Indonesia".to_string() },
        Cargo{ weight: 456, destination: "Somewhat, West Java, Indonesia".to_string() },
        Cargo{ weight: 222, destination: "Somehow, East Java, Indonesia".to_string() },
    ];

    deliver_cargo(cargos);
}
