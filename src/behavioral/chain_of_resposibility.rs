// Chain of Responsibility is a design pattern
// that allows you to pass requests along a chain of handlers
// instead of calling them one by one
//
// In this example, we have a call center for a e-commerce platform
// that have multiple menus. When buidling a small app, it does make
// sense to unify the handlers in one single struct. However,
// this reduces the maintainability of the handler. To resolve this,
// we can apply the chain of responsibility pattern by defining a trait
// for handlers to follow. User's request will be handled by central
// call center which will then forwarded to handle chains based on
// its command.

use std::collections::HashMap;

pub trait Handler {
    fn handle(&self, command: String);
}

pub struct CallCenter {
    handler_map: HashMap<String, Box<dyn Handler> >,
}
impl CallCenter {
    pub fn new() -> CallCenter {
        CallCenter { handler_map: HashMap::new() }
    }
    pub fn register_handler(&mut self, id: String, handler: Box<dyn Handler>) {
        self.handler_map.insert(id, handler);
    }
}
impl Handler for CallCenter {
    fn handle(&self, command: String) {
        println!("Thank you for calling our call center!");

        let cmd = command.get(0..1).unwrap().to_string();

        if let Some(handler) = self.handler_map.get(&cmd) {
            handler.handle(command[1..].to_string());

            return;
        }

        println!("I'm sorry, but I don't know what that is");
    }
}

pub struct AccountHandler{
    handler_map: HashMap<String, Box<dyn Handler> >,
}
impl AccountHandler {
    pub fn new() -> CallCenter {
        CallCenter { handler_map: HashMap::new() }
    }
    pub fn register_handler(&mut self, id: String, handler: Box<dyn Handler>) {
        self.handler_map.insert(id, handler);
    }
}
impl Handler for AccountHandler {
    fn handle(&self, command: String) {
        println!("Hiya, I'm account handler! Happy to assist you with any problems in your account!");

        let cmd = command.get(0..1).unwrap().to_string();

        if let Some(handler) = self.handler_map.get(&cmd) {
            handler.handle(command[1..].to_string());

            return;
        }
    }
}

pub struct TransactionHandler{}
impl Handler for TransactionHandler {
    fn handle(&self, _: String) {
        println!("Hola! I'm transaction handler. Ready to assist you with any problems regarding transactions.")
    }
}
