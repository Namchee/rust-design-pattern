// Composite is a design pattern that groups smaller objects (leaves) into
// one big object (composite). The main purpose of using this pattern is enforcing
// hierarchy or working with all the components at the same time.
//
// In this example, we have a Button and Input component as leaves and
// Dialog component as a composite object. When we move the Dialog component,
// all of the component needs to be translated too.

pub trait Component {
    fn translate(&mut self, x: u32, y: u32);
    fn get_x(&self) -> u32;
    fn get_y(&self) -> u32;
}

pub struct Button {
    pub x: u32,
    pub y: u32,
}

impl Component for Button {
    fn translate(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    fn get_x(&self) -> u32 {
        self.x
    }

    fn get_y(&self) -> u32 {
        self.y
    }
}

#[allow(dead_code)]
pub struct Input {
    pub x: u32,
    pub y: u32,

    pub value: String,
}

impl Component for Input {
    fn translate(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    fn get_x(&self) -> u32 {
        self.x
    }

    fn get_y(&self) -> u32 {
        self.y
    }
}

pub struct Dialog {
    x: u32,
    y: u32,
    components: Vec<Box<dyn Component>>,
}

#[allow(dead_code)]
impl Dialog {
    pub fn new() -> Dialog {
        Dialog{ components: vec![], x: 0, y: 0 }
    }

    pub fn new_with_coordinate(x: u32, y: u32) -> Dialog {
        Dialog{ components: vec![], x: x, y: y }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
}

impl Component for Dialog {
    fn translate(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;

        for comp in self.components.iter_mut() {
            let comp_x = comp.get_x();
            let comp_y = comp.get_y();

            comp.as_mut().translate(comp_x + x, comp_y + y);
        }
    }

    fn get_x(&self) -> u32 {
        self.x
    }

    fn get_y(&self) -> u32 {
        self.y
    }
}
