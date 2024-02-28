mod creational;

use crate::creational::factory::{Cargo, deliver_cargo};
use crate::creational::abstract_factory::{WindowsUIManager, AppUIManager};
use creational::builder::{PCBuilder, Processor};

fn main() {
    // Factory method calls
    let cargos = vec![
        Cargo{ weight: 123, destination: "Somewhere, Sumatra, Indonesia".to_string() },
        Cargo{ weight: 456, destination: "Somewhat, West Java, Indonesia".to_string() },
        Cargo{ weight: 222, destination: "Somehow, East Java, Indonesia".to_string() },
    ];

    deliver_cargo(cargos);

    // Abstract factory pattern
    let ui_manager = AppUIManager{ os_manager: Box::new(WindowsUIManager{}) };
    let button = ui_manager.create_button();
    button.click();
    let input = ui_manager.create_input();
    input.on_input("Hello World!".to_string());

    // Builder pattern
    let mut pc_builder = PCBuilder::new_builder();
    let processor = Processor{ socket: "AM5".to_string(), series: "7700X".to_string(), cores: 8, threads: 16, tdp: 105 };
    pc_builder.buy_processor(processor);
}
