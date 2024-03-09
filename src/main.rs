mod creational;
mod structural;
mod behavioral;

use creational::builder::{PCBuilder, Processor};
use creational::factory::{Cargo, deliver_cargo};
use creational::abstract_factory::{WindowsUIManager, AppUIManager};
use creational::singleton::exec;

use structural::adapter::{EuropeanSocket, LaptopCharger, PowerConverter};
use structural::bridge::{NextGenerationRemoteControl, RemoteControl, TV};
use structural::composite::{Button, Component, Dialog, Input};
use structural::facade::{CheckoutFacade, DeliveryService, InventoryManagement, Order, PaymentGateway, ShoppingCart, User, OrderStatus};
use structural::flyweight::{Forest, TreeFactory};

use structural::decorator::{UserPostgreRepository, UserRepository, UserRepositoryWithLogger};
use structural::proxy::{Cache, UserRepositoryWithCache};

use behavioral::chain_of_resposibility::{DeliveryProcess, ManufacturingLine, PackagingProcess, QAProcess, Shirt};
use behavioral::command::{AccountHandler, CallCenter, TransactionHandler};
use behavioral::mediator::{Anon, Forum};
use behavioral::memento::{Editor, Snapshot, SnapshotManager};
use behavioral::state::{MusicPlayer, Player, Song};
use behavioral::template::{parse_config, JSONConfigurationManager, YAMLConfigurationManager};

use crate::behavioral::visitor::{Circle, JSONExporter, Shape, Square};

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

    // Decorator
    let mut original_repo = Box::new(UserPostgreRepository{});
    println!("{}", original_repo.find_user("John Doe".to_string()).unwrap().name);

    let mut benchmarked_repo = UserRepositoryWithLogger{ repo: original_repo.clone() };

    println!("{}", benchmarked_repo.find_user("John Doe".to_string()).unwrap().name);

    // Facade, instead of directly calling these services, just call the facade!
    let cart = ShoppingCart{};
    let delivery = DeliveryService{};
    let inventory = InventoryManagement{};
    let payment = PaymentGateway{};
    let facade = CheckoutFacade{ cart, delivery, inventory, payment_gateway: payment };

    let user = User{
        id: 12,
        name: "John Doe".to_string(),
    };
    let order = Order{
        number: "THIS-IS-UUID".to_string(),
        items: vec![],
        status: OrderStatus::IN_PROCESS,
        payment_method: "STEAM_WALLET".to_string(),
    };
    let _ = facade.checkout(user, order);

    // Flyweight
    let factory = TreeFactory::new();
    let mut forest = Forest::new(factory);
    forest.plant_trees(0, 123, "Maple".to_string(), (255, 0, 0, 255), "maple_texture.jpg".to_string());

    // Proxy
    let cache = Cache::new();
    let mut cached = UserRepositoryWithCache{ repo: original_repo.clone(), cache  };

    println!("{}", cached.find_user("John Doe".to_string()).unwrap().name);

    /* Behavioral Patterns */

    // Chain of responsibility
    let mut qa_process = QAProcess::new();
    let mut packaging = PackagingProcess::new();
    let delivery = DeliveryProcess::new();
    packaging.set_next(Box::new(delivery));
    qa_process.set_next(Box::new(packaging));

    qa_process.handle(Shirt { color: (255, 255, 255), size: "XL".to_string() });

    // Command
    let mut call_center = CallCenter::new();
    let account_handler = AccountHandler{};
    let transaction_handler = TransactionHandler{};

    call_center.register_handler("1".to_string(), Box::new(account_handler));
    call_center.register_handler("2".to_string(), Box::new(transaction_handler));

    call_center.handle("123".to_string());

    // Mediator
    let forum = Forum::new();
    let andy = Anon::new("xXx_Darkness_xXx".to_string());
    let tom = Anon::new("Tom4321".to_string());

    andy.send_message("Hello World!".to_string(), &forum);
    tom.send_message("Hello Darkness!".to_string(), &forum);

    // Memento
    let mut editor = Editor::new();
    let mut manager = SnapshotManager::new();
    editor.set_text("Hello World!".to_string());
    let new_state = editor.save_state();
    let state_name = new_state.name.clone();
    manager.save_snapshot(new_state);

    // let's just say that this is too vulgar and we want to reset it
    editor.set_text("Goodbye cruel world!".to_string());

    let target_snapshot = manager.get_snapshot(state_name);
    if target_snapshot.is_some() {
        target_snapshot.unwrap().restore(&mut editor);
    }
    
    // State pattern
    let mut player = Player::new();
    let pop = Song::new("Beat It!", 30_000);
    let jazz = Song::new("The Blues", 999_000);
    let rock = Song::new("Stairway to Heaven", 5_000);

    player.add_song(pop);
    player.add_song(jazz);
    player.add_song(rock);

    let mut music_player = MusicPlayer::new(player);
    music_player.play();

    // Template method pattern
    let _json_cfg = parse_config(None, JSONConfigurationManager);
    let _yaml_cfg = parse_config(None, YAMLConfigurationManager);

    // Visitor
    let json_exporter = JSONExporter;
    let circle = Circle{ radius: 1.0 };
    let square = Square{ side: 2 };

    circle.export(&json_exporter);
    square.export(&json_exporter);
}
