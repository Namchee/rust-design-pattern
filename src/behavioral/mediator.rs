// Mediator is a design pattern that simplifies
// communication between dependencies. Instead of directly
// communicating with each other, dependencies communicate through
// intermediate object that forwards message to correct channel.
//
// In this example, we have an anonymouse web forum app that allows users to post
// messages in chatroom. Since one forum may consist of many anons,
// it would be a hassle if anon need to notify all other anons in the chatroom.
// Moreover, anon shouldn't know other anons information in the chatroom.
// Hence, it's best to use another object as a mediator that broadcasts all messages
// to other anons. 

use uuid::Uuid;

pub struct Forum<'a> {
    pub anons: Vec<&'a Anon<'a>>,
}
impl Forum<'_> {
    pub fn new() -> Forum<'static> {
        Forum { anons: vec![] }
    }

    pub fn add_user(&mut self, anon: &Anon) {
        self.anons.push(anon);
    } 

    pub fn broadcast(&self, source: String, msg: String) {
        for anon in self.anons.iter() {
            if anon.id != source {
                anon.receive_msg(msg.clone());
            }
        }
    }
}

pub struct Anon<'a> {
    pub id: String,
    pub display: String,

    pub forum: Option<&'a Forum<'a>>,
}
impl Anon<'_> {
    pub fn new(display: String) -> Anon<'static> {
        Anon { id: Uuid::new_v4().to_string(), display, forum: None }
    }

    pub fn join_forum(&mut self, forum: &mut Forum) {
        forum.add_user(self);
    }

    pub fn send_message(&self, msg: String) {
        if self.forum.is_some() {
            self.forum.as_ref().unwrap().broadcast(self.id.clone(), msg);
        }
    }

    pub fn receive_msg(&self, msg: String) {
        println!("Received {} from anon!", msg);
    }
}
