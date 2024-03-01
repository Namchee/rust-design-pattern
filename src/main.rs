mod creational;
mod structural;

use crate::creational::factory::{Cargo, deliver_cargo};
use crate::creational::abstract_factory::{WindowsUIManager, AppUIManager};
use creational::builder::{PCBuilder, Processor};
use creational::singleton::exec;

use structural::adapter::{EuropeanSocket, LaptopCharger, PowerConverter};
use structural::bridge::{NextGenerationRemoteControl, RemoteControl, TV};
use structural::composite::{Button, Component, Dialog, Input};

fn main() {
    /* Creational Patterns */

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
    let _result = pc_builder.build_pc(); // evaluate here 

    // Singleton pattern
    exec("INSERT INTO users (name, age) VALUES ('John Doe', 123)".to_string());

    /* Structural Patterns */
    
    // Adapter pattern
    let laptop_charger = LaptopCharger{ rating: 320 };
    let socket = EuropeanSocket{};
    let adapter = PowerConverter{ charger: laptop_charger };
    socket.plug_in(Box::new(adapter));

    // Bridge
    let rc = RemoteControl{};
    let tv = TV{ status: false, volume: 0 };
    rc.turn_device_on(Box::new(tv));
    rc.increment_volume(Box::new(tv));
    let fancy_rc = NextGenerationRemoteControl{ rc: rc };
    fancy_rc.mute(Box::new(tv));
    fancy_rc.turn_device_on(Box::new(tv));

    // Composite
    let button = Button{ x: 1, y: 2 };
    let input = Input{ x: 2, y: 4, value: "".to_string() };
    let mut dialog = Dialog::new();
    dialog.add_component(Box::new(button));
    dialog.add_component(Box::new(input));

    dialog.translate(10, 20);
}
