// Builder pattern lets you construct a complex object step by step
//
// In this case, let's say that you want to build a PC. As a first time
// however, you have difficulties in choosing the best part for your dream
// rig. Moreover, it seems like a new-and-better product will be coming soon.
// So, you buy those parts step-by-step and build them when all required
// parts are present.
//
// Moreover, a builder prevents you from making rookie mistakes such as choosing the wrong
// socket of CPU and motherboard or choosing outdated memory.

#[derive(Clone)]
pub struct Processor {
    pub socket: String,
    pub series: String,
    pub cores: u8,
    pub threads: u16,
    pub tdp: u16,
}

#[derive(Clone)]
struct Motherboard {
    socket: String,
    memory_type: String,
    storage_type: Vec<String>,
    pcie: String,
}

#[derive(Clone)]
struct GraphicCard {
    socket: String,
    memory: u32,
    tdp: u16,
}

#[derive(Clone)]
struct Memory {
    socket: String,
    capacity: u32,
}

#[derive(Clone)]
struct Storage {
    socket: String,
}

#[derive(Clone)]
struct PowerSupply {
    capacity: u32,
}

pub struct PC {
    pub processor: Processor,
    pub motherboard: Motherboard,
    pub graphic_card: GraphicCard,
    pub memory: Memory,
    pub storage: Storage,
    pub power_supply: PowerSupply,
}

pub struct PCBuilder {
    processor: Option<Processor>,
    motherboard: Option<Motherboard>,
    graphic_card: Option<GraphicCard>,
    memory: Option<Memory>,
    storage: Option<Storage>,
    power_supply: Option<PowerSupply>,
}

impl PCBuilder {
    pub fn new_builder() -> PCBuilder {
        PCBuilder { processor: None, motherboard: None, graphic_card: None, memory: None, storage: None, power_supply: None }
    }

    pub fn buy_processor(&mut self, processor: Processor) -> &PCBuilder {
        if self.motherboard.is_some() {
            let skt = self.motherboard.as_ref().unwrap().socket.clone();

            if skt != processor.socket {
                println!("Incompatible processor socket. The current motherboard only supports {} processors", skt);

                return self;
            }
        }

        self.processor = Some(processor);

        self
    }

    pub fn sell_processor(&mut self) -> &PCBuilder {
        self.processor = None;

        self
    }

    pub fn buy_motherboard(&mut self, motherboard: Motherboard) -> &PCBuilder {
        if self.processor.is_some() {
            let skt = self.processor.as_ref().unwrap().socket.clone();

            if skt != motherboard.socket {
                println!(
                    "Incompatible motherboard socket. The current processor requires {} socket",
                    skt
                );

                return self;
            }
        }

        if self.memory.is_some() {
            let skt = self.memory.as_ref().unwrap().socket.clone();

            if skt != motherboard.memory_type {
                println!(
                    "Incompatible memory socket. The current memory is a {} RAM",
                    skt
                );

                return self;
            }
        }

        if self.storage.is_some() {
            let skt = self.storage.as_ref().unwrap().socket.clone();

            if !motherboard.storage_type.iter().any(|f| *f == skt) {
                println!(
                    "Unsupported storage. The current storage requires {} interface",
                    skt
                );

                return self;
            }
        }

        self.motherboard = Some(motherboard);

        self
    }

    pub fn sell_motherboard(&mut self) -> &PCBuilder {
        self.motherboard = None;

        self
    }

    pub fn buy_graphic_card(&mut self, vga: GraphicCard) -> &PCBuilder {
        self.graphic_card = Some(vga);

        self
    }

    pub fn sell_graphic_card(&mut self) -> &PCBuilder {
        self.graphic_card = None;

        self
    }

    pub fn buy_memory(&mut self, ram: Memory) -> &PCBuilder {
        if self.motherboard.is_some() {
            let skt = self.motherboard.as_ref().unwrap().memory_type.clone();

            if skt != ram.socket {
                println!(
                    "Incompatible motherboard socket. The current motherboard only supports {} RAM",
                    skt
                );

                return self;
            }
        }

        self.memory = Some(ram);

        self
    }

    pub fn sell_memory(&mut self) -> &PCBuilder {
        self.memory = None;

        self
    }

    pub fn buy_storage(&mut self, storage: Storage) -> &PCBuilder {
        if self.motherboard.is_some() {
            let skt = self.motherboard.as_ref().unwrap().storage_type.clone();

            if !skt.iter().any(|socket| *socket == storage.socket) {
                println!(
                    "Incompatible motherboard. The current motherboard does not support {}",
                    storage.socket,
                );

                return self;
            }
        }

        self.storage = Some(storage);

        self
    }

    pub fn sell_storage(&mut self) -> &PCBuilder {
        self.storage = None;

        self
    }

    pub fn buy_power_supply(&mut self, psu: PowerSupply) -> &PCBuilder {
        self.power_supply = Some(psu);

        self
    }

    pub fn sell_power_supply(&mut self) -> &PCBuilder {
        self.power_supply = None;

        self
    }

    pub fn sell_all_parts(&mut self) -> &PCBuilder {
        self.processor = None;
        self.motherboard = None;
        self.graphic_card = None;
        self.memory = None;
        self.storage = None;
        self.power_supply = None;

        self
    }

    pub fn build_pc(&self) -> Result<PC, String> {
        if self.processor.is_none() || self.motherboard.is_none() || self.graphic_card.is_none() || self.memory.is_none() || self.storage.is_none() || self.power_supply.is_none() {
            return Err("This PC is still missing some required parts. Please check the parts again.".to_string());
        }

        let new_pc = PC{
            processor: self.processor.as_ref().unwrap().to_owned(),
            motherboard: self.motherboard.as_ref().unwrap().to_owned(),
            graphic_card: self.graphic_card.as_ref().unwrap().to_owned(),
            memory: self.memory.as_ref().unwrap().to_owned(),
            storage: self.storage.as_ref().unwrap().to_owned(),
            power_supply: self.power_supply.as_ref().unwrap().to_owned(),
        };



        Ok(new_pc)
    }
}
