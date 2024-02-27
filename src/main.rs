mod creational;

use crate::creational::factory::{Cargo, deliver_cargo};
use crate::creational::abstract_factory::{WindowsUIManager, AppUIManager};

fn main() {
    // Factory method calls
    let cargos = vec![
        Cargo{ weight: 123, destination: "Somewhere, Sumatra, Indonesia".to_string() },
        Cargo{ weight: 456, destination: "Somewhat, West Java, Indonesia".to_string() },
        Cargo{ weight: 222, destination: "Somehow, East Java, Indonesia".to_string() },
    ];

    deliver_cargo(cargos);

    // Abstract factory calls
    let ui_manager = AppUIManager{ os_manager: Box::new(WindowsUIManager{}) };
    let button = ui_manager.create_button();
    button.click();
    let input = ui_manager.create_input();
    input.on_input("Hello World!".to_string());
}
