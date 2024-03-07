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

pub struct Forum {
    anons: Vec<Anon>,
}
impl Forum {
    pub fn new() -> Forum {
        Forum { anons: vec![] }
    }

    pub fn broadcast(&self, source: &str, msg: &str) {
        for anon in self.anons.iter() {
            if anon.id != source {
                anon.receive_msg(msg, source);
            }
        }
    }
}

pub struct Anon {
    pub id: String,
    pub display: String,
}
impl Anon {
    pub fn new(display: String) -> Anon {
        Anon { id: Uuid::new_v4().to_string(), display }
    }

    pub fn send_message(&self, msg: String, forum: &Forum) {
        forum.broadcast(&self.id, msg.as_str());
    }

    pub fn receive_msg(&self, msg: &str, src: &str) {
        println!("Received {} from {}!", msg, src);
    }
}
