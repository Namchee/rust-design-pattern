// Flyweight is a pattern that allows us to fit more objects into the memory
// by sharing common parts between multiple objects.
//
// In this example, we have a forest that consists of Trees.
// Logically, trees inside a forest are only divided into few types.
// It doesn't make sense if we store render information in each trees
// as it will waste memory without any benefits.

type Color = (u8, u8, u8, u8);

trait Canvas {
    fn draw(&self, x: u64, y: u64);
}

#[derive(Clone)]
pub struct TreeType {
    pub name: String,
    pub color: Color,
    pub texture: String,
}
impl TreeType {
    pub fn draw(&self, canvas: &Box<dyn Canvas>, x: u64, y: u64) {
        canvas.draw(x, y);
    }
}

pub struct TreeFactory {
    tree_type: Vec<TreeType>,
}
impl TreeFactory {
    pub fn new() -> TreeFactory {
        TreeFactory { tree_type: vec![] }
    }

    pub fn get_tree_type(&mut self, name: String, color: Color, texture: String) -> TreeType {
        let tree_type = self.tree_type.iter().find(|t| *t.name == name && t.color == color && *t.texture == texture);
        if tree_type.is_none() {
            let new_tree_type = TreeType{ name, color, texture };
            self.tree_type.push(new_tree_type.clone());

            return new_tree_type;
        }

        tree_type.unwrap().clone()
    }
}

struct Tree {
    x: u64,
    y: u64,
    tree_type: TreeType,
}
impl Tree {
    pub fn draw(&self, canvas: &Box<dyn Canvas>) {
        self.tree_type.draw(canvas, self.x, self.y);
    }
}

pub struct Forest {
    pub trees: Vec<Tree>,
    pub tree_factory: TreeFactory,
}
impl Forest {
    pub fn new(factory: TreeFactory) -> Forest {
        Forest{
            trees: vec![],
            tree_factory: factory,
        }
    }
    pub fn plant_trees(&mut self, x: u64, y: u64, name: String, color: Color, texture: String) {
        let tree_type = self.tree_factory.get_tree_type(name, color, texture);
        let new_tree = Tree{ tree_type, x, y };
        self.trees.push(new_tree);
    }

    pub fn draw(&self, canvas: &Box<dyn Canvas>) {
        for tree in self.trees.iter() {
            tree.draw(canvas);
        }
    }
}
