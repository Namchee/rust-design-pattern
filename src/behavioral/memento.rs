// Memento is a pattern that allows you to 'undo'
// an object to its previous state without knowing
// the implementation details of that object.
//
// In this example, we have a simplified text editor app
// that supports undoing the previous state

type EditorColor = (u8, u8, u8, u8);

pub trait Snapshot {
    fn restore(&self, editor: &mut Editor);
    fn print(&self);
}

pub struct Editor {
    pub text: String,
    pub font: String,
    pub color: EditorColor,
}
impl Editor {
    pub fn new() -> Editor {
        Editor { text: "".to_string(), font: "Times New Roman".to_string(), color: (0, 0, 0, 255) }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text; 
    }

    pub fn save_state(&self) -> EditorSnapshot {
        EditorSnapshot::new("SAVE-STATE".to_string(), self.text.clone(), self.font.clone(), self.color)
    }
}

pub struct EditorSnapshot {
    pub name: String,
    pub text: String,
    pub font: String,
    pub color: (u8, u8, u8, u8),
}
impl EditorSnapshot {
    pub fn new(name: String, text: String, font: String, color: EditorColor) -> EditorSnapshot {
        EditorSnapshot { name, text, font, color }
    }
}
impl Snapshot for EditorSnapshot {
    fn restore(&self, editor: &mut Editor) {
        editor.text = self.text.clone();
        editor.font = self.font.clone();
        editor.color = self.color;
    }

    fn print(&self) {
        println!("Snapshot {}", self.name);
        println!("Text: {}", self.text);
        println!("Font: {}", self.font);
        println!("Color: {:?}", self.color);
    }
}

pub struct SnapshotManager {
    snapshots: Vec<EditorSnapshot>,
}
impl SnapshotManager {
    pub fn new() -> SnapshotManager {
        SnapshotManager { snapshots: vec![] }
    }

    pub fn get_snapshot(&self, name: String) -> Option<&EditorSnapshot> {
        self.snapshots.iter().find(|s| s.name == name)
    }

    pub fn save_snapshot(&mut self, snapshot: EditorSnapshot) {
        self.snapshots.push(snapshot);
    }
}
