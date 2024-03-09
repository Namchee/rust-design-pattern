// Visitor is a design pattern that separates algorithm
// from the object they operate on
//
// In this example, we have multiple shapes that can be exported
// to a specialized JSON format

const PI: f32 = 22 as f32 / 7 as f32;

pub trait Shape {
    fn area(&self) -> f32;
    fn circumference(&self) -> f32;
    fn export(&self, exporter: &impl Visitor);
}

pub struct Circle {
    pub radius: f32,
}
impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * PI
    }

    fn circumference(&self) -> f32 {
        self.radius * 2 as f32 * PI
    }

    fn export(&self, exporter: &impl Visitor) {
        exporter.export_circle(self);
    }
}
pub struct Square {
    pub side: u32,
}
impl Shape for Square {
    fn area(&self) -> f32 {
        self.side as f32 * self.side as f32
    }

    fn circumference(&self) -> f32 {
        4 as f32 * self.side as f32
    }

    fn export(&self, exporter: &impl Visitor) {
        exporter.export_square(self);
    }
}

pub trait Visitor {
    fn export_circle(&self, shape: &Circle);
    fn export_square(&self, shape: &Square);
}
pub struct JSONExporter;
impl Visitor for JSONExporter {
    fn export_circle(&self, shape: &Circle) {
        println!("Exporting circle with radius {} as JSON...", shape.radius);
    }

    fn export_square(&self, shape: &Square) {
        println!("Exporting square with side {} as JSON...", shape.side);
    }
}
