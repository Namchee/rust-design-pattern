// Chain of Responsibility is a design pattern
// that allows you to pass requests along a chain of handlers
// instead of calling them one by one
//
// In this example, we have a factory that produces shirts.
// After a shirt is sewn, it goes through the QA process,
// packaging process, and finally delivery to warehouse

pub struct Shirt {
    pub color: (u8, u8, u8),
    pub size: String,
}

pub trait ManufacturingLine {
    fn set_next(&mut self, handler: Box<dyn ManufacturingLine>);
    fn handle(&self, shirt: Shirt);
}

pub struct QAProcess{
    next: Option<Box<dyn ManufacturingLine> >, 
}
impl QAProcess {
    pub fn new() -> QAProcess {
        QAProcess { next: None }
    }
}
impl ManufacturingLine for QAProcess {
    fn set_next(&mut self, handler: Box<dyn ManufacturingLine>) {
        self.next = Some(handler);
    }
    fn handle(&self, shirt: Shirt) {
        println!("Checking shirt quality");

        if self.next.is_some() {
            self.next.as_ref().unwrap().handle(shirt);
        }
    }
}

pub struct PackagingProcess{
    next: Option<Box<dyn ManufacturingLine> >, 
}
impl PackagingProcess {
    pub fn new() -> PackagingProcess {
        PackagingProcess { next: None }
    }
}
impl ManufacturingLine for PackagingProcess {
    fn set_next(&mut self, handler: Box<dyn ManufacturingLine>) {
        self.next = Some(handler);
    }
    fn handle(&self, shirt: Shirt) {
        println!("Packaging shirt nicely...");

        if self.next.is_some() {
            self.next.as_ref().unwrap().handle(shirt);
        }
    }
}

pub struct DeliveryProcess{
    next: Option<Box<dyn ManufacturingLine> >, 
}
impl DeliveryProcess {
    pub fn new() -> DeliveryProcess {
        DeliveryProcess { next: None }
    }
}
impl ManufacturingLine for DeliveryProcess {
    fn set_next(&mut self, handler: Box<dyn ManufacturingLine>) {
        self.next = Some(handler);
    }
    fn handle(&self, shirt: Shirt) {
        println!("Delivering packaged shirt to warehouse...");

        if self.next.is_some() {
            self.next.as_ref().unwrap().handle(shirt);
        }
    }
}
