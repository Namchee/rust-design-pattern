// Adapter is a structural pattern that acts as compatibility layer
// between interfaces that aren't originally compatible/
//
// In this example, let's say we are travelling from America
// to an European country. You will be surprised that your electronic
// charger isn't compatible with the socket they have in Europe even
// though they do the same thing! Since you need to use your laptop,
// you decided to buy an adapter so your charger can properly connect
// to the socket.

pub struct LaptopCharger {
    pub rating: u32,
}

pub trait EuropeanPort {
    fn get_power(&self) -> u32;
}

pub struct EuropeanSocket {}
impl EuropeanSocket {
    pub fn plug_in(&self, port: Box<dyn EuropeanPort>) {
        println!("I am plugged in, in Europe! With power of {}", port.get_power());
    }
}

pub struct PowerConverter {
    pub charger: LaptopCharger,
}

impl EuropeanPort for PowerConverter {
    fn get_power(&self) -> u32 {
        // I don't know how to actually convert it, let's just say it's this way
        self.charger.rating - 45 * 9
    }
}
