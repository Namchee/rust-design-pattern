// Bride is a structural pattern that enforces separation between
// abstraction and implementation. It separates complex classes
// into separate individual hierarchy that can be developed in parallel.

// In this example, we have a magic remote control that can control a device.
// The next day, a manufacturer releases a better remote control: RemoteControlV2
// that has the ability to mute. If the logic is coupled to a struct.
//
// Naively, we can just create a new class and re-implement the methods from scratch.
// We can save time if use the original remote control as a prop of RemoteControlV2

pub trait Device {
    fn is_on(&self) -> bool;
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn get_volume(&self) -> u16;
    fn set_volume(&mut self, volume: u16);
}

#[derive(Clone, Copy)]
pub struct TV {
    pub status: bool,
    pub volume: u16,
}

impl Device for TV {
    fn is_on(&self) -> bool {
        self.status
    }

    fn turn_on(&mut self) {
        self.status = true;
    }

    fn turn_off(&mut self) {
        self.status = false;
    }

    fn get_volume(&self) -> u16 {
        self.volume
    }

    fn set_volume(&mut self, volume: u16) {
        self.volume = volume
    }
}

pub struct RemoteControl {}

#[allow(dead_code)]
impl RemoteControl {
    pub fn turn_device_on(&self, mut device: Box<dyn Device>) {
        if !device.is_on() {
            device.turn_on();
        }
    }

    pub fn turn_device_off(&self, mut device: Box<dyn Device>) {
        if device.is_on() {
            device.turn_off();
        }
    }

    pub fn increment_volume(&self, mut device: Box<dyn Device>) {
        device.set_volume(device.get_volume() + 1);
    }

    pub fn decrement_volume(&self, mut device: Box<dyn Device>) {
        device.set_volume(device.get_volume() - 1);
    }
}

pub struct NextGenerationRemoteControl {
    pub rc: RemoteControl,
}

#[allow(dead_code)]
impl NextGenerationRemoteControl {
    pub fn mute(&self, mut device: Box<dyn Device>) {
        device.set_volume(0);
    }
}

impl std::ops::Deref for NextGenerationRemoteControl {
    type Target = RemoteControl;
    fn deref(&self) -> &Self::Target {
        &self.rc
    }
}
