// State is a design pattern that changes object behavior when
// its internal changed. It's closely related to finite-state machine (FSM)
//
// In this example, we have a music player that have 3 states: Playing, Stopped,
// and Paused. Change in those states may affect how the music player behaves, such
// as changing tracks, rewinding, and state transitions.

pub trait State {
    fn play(&self, player: &mut Player) -> Box<dyn State>;
    fn stop(&self, player: &mut Player) -> Box<dyn State>;
}
impl dyn State {
    pub fn next(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.next_track();

        self
    }

    pub fn prev(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.prev_track();

        self
    }
}

pub struct PlayingState;
impl State for PlayingState {
    fn play(&self, _: &mut Player) -> Box<dyn State> {
        Box::new(PlayingState)
    }

    fn stop(&self, player: &mut Player) -> Box<dyn State> {
        player.pause();
        player.rewind();

        Box::new(StoppedState)
    }
}
pub struct StoppedState;
impl State for StoppedState {
    fn play(&self, player: &mut Player) -> Box<dyn State> {
        player.play();

        Box::new(PlayingState)
    }

    fn stop(&self, _: &mut Player) -> Box<dyn State> {
        Box::new(StoppedState)
    }
}
pub struct PausedState;
impl State for PausedState {
    fn play(&self, player: &mut Player) -> Box<dyn State> {
        player.play();

        Box::new(PlayingState)
    }

    fn stop(&self, player: &mut Player) -> Box<dyn State> {
        player.rewind();

        Box::new(StoppedState)
    }
}


pub struct Song {
    pub title: String,
    pub duration: u32,

    cursor: u32,
}
impl Song {
    pub fn new(title: &str, duration: u32) -> Self {
        Song { title: title.into(), duration, cursor: 0 }
    }
}

pub struct Player {
    pub playlist: Vec<Song>,
    pub current_track: usize,
    volume: u8,
}

impl Player {
    pub fn new() -> Self {
        Player { playlist: vec![], current_track: 0, volume: 0 }
    }

    pub fn add_song(&mut self, song: Song) {
        self.playlist.push(song);
    }

    pub fn next_track(&mut self) {
        self.current_track = (self.current_track + 1) % self.playlist.len();
    }

    pub fn prev_track(&mut self) {
        self.current_track = (self.playlist.len() + self.current_track + 1) % self.playlist.len();
    }

    pub fn get_current_track(&self) -> (&Song, usize) {
        (&self.playlist[self.current_track], self.current_track)
    }

    pub fn play(&mut self) {
        self.playlist[self.current_track].cursor = 45;
    }

    pub fn pause(&mut self) {
        self.playlist[self.current_track].cursor = 27;
    }

    pub fn rewind(&mut self) {
        self.playlist[self.current_track].cursor = 0;
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume;
    }
}

pub struct MusicPlayer {
    player: Player,
    state: Box<dyn State>,
}
impl MusicPlayer {
    pub fn new(player: Player) -> Self {
        MusicPlayer{ player, state: Box::new(StoppedState) }
    }

    pub fn play(&mut self) {
        let new_state = self.state.play(&mut self.player);

        self.state = new_state;
    }

    pub fn stop(&mut self) {
        let new_state = self.state.stop(&mut self.player);

        self.state = new_state;
    }
}
