// Abstract factory pattern is an upgraded version of factory method. In layman terms,
// abstract factory method introduces 'master' factory that calls other factory for client.
//
// In this example, we are building a cross-platform GUI app. Your app runs on Windows and
// Mac that have their own quirks related to UI elements such as buttons and inputs. Fortunately To
// return the correct 'UI Kit'

// Button is an abstract UI component that every OS have
pub trait Button {
    fn click(&self);
}

// Input is another abstract UI component that every OS have
pub trait Input {
    fn on_input(&self, value: String);
    fn on_focus(&self);
    fn on_blur(&self);
}

// Concrete implementations of button on Windows and Mac
struct WindowsButton {}
struct MacButton{}

impl Button for WindowsButton {
    fn click(&self) {
        println!("Hello from Fluent");
    }
}

impl Button for MacButton {
    fn click(&self) {
        println!("Hello Cupertino");
    }
}

// Concrete implementations of button on Windows and Mac
struct WindowsInput {}
struct MacInput {}

impl Input for WindowsInput {
    fn on_input(&self, _: String) {
        println!("Fluent is typing...");
    }
    
    fn on_focus(&self) {
        println!("Fluent is now focused!");
    }

    fn on_blur(&self) {
        println!("Goodbye Fluent!");
    }
}

impl Input for MacInput {
    fn on_input(&self, _: String) {
        println!("Cupertino is typing...");
    }
    
    fn on_focus(&self) {
        println!("Cupertino is now focused!");
    }

    fn on_blur(&self) {
        println!("Goodbye Cupertino!");
    }
}

// UIManager is an abstract factory interface
pub trait UIManager {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_input(&self) -> Box<dyn Input>;
}

pub struct WindowsUIManager {}
impl UIManager for WindowsUIManager {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton{})
    }

    fn create_input(&self) -> Box<dyn Input> {
        Box::new(WindowsInput{})
    }
}

pub struct MacUIManager {}
impl UIManager for MacUIManager {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton{})
    }

    fn create_input(&self) -> Box<dyn Input> {
        Box::new(MacInput{})
    }
}

// AppUIManager is the 'master' factory that encapsulates specialized factories
pub struct AppUIManager {
    pub os_manager: Box<dyn UIManager>,
}
impl AppUIManager {
    pub fn create_button(&self) -> Box<dyn Button> {
        self.os_manager.create_button()
    }

    pub fn create_input(&self) -> Box<dyn Input> {
        self.os_manager.create_input()
    }
}
