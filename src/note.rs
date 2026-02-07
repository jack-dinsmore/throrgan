const STYLE_ATTACK: u8 = 0x0000_0001;

#[derive(Clone, Copy, Debug)]
pub enum Pitch {
    Pitch(u8),
    Rest,
}
impl Pitch {
    pub fn freq(&self) -> Option<f64> {
        match self {
            Self::Pitch(b) => Some(440. * ((*b as f64 - 48.) / 12.).powf(1./12.)),
            Self::Rest => None,
        }
    }
    
    pub const fn new(note: char, level: u8) -> Self {
        Self::Pitch(match note {
            'a' => 9,
            'b' => 11,
            'c' => 0,
            'd' => 2,
            'e' => 4,
            'f' => 5,
            'g' => 7,
            _ => panic!("Invalid note name")
        } + level*12 - 9)
    }
    
    pub const fn sh(&self) -> Self {
        match self {
            Pitch::Pitch(n) => Self::Pitch(*n + 1),
            Pitch::Rest => *self,
        }
    }
    
    pub const fn fl(&self) -> Self {
        match self {
            Pitch::Pitch(n) => Self::Pitch(*n - 1),
            Pitch::Rest => *self,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub dur: f64,
    style: u8,
}
impl Note {
    pub fn attack(pitch: Pitch, dur: f64) -> Self {
        Self {
            pitch,
            dur,
            style: STYLE_ATTACK,
        }
    }
    
    pub(crate) fn rest(dur: f64) -> Self {
        Self {
            pitch: Pitch::Rest,
            dur,
            style: 0
        }
    }
}

pub struct Player {
    time_per_sample: f64,
    secs_per_beat: f64,
    pub notes: Vec<Note>
}

impl Player {
    pub fn new(notes: Vec<Note>, sample_rate: u32, bpm: f64) -> Self {
        Self {
            time_per_sample: 1. / sample_rate as f64,
            secs_per_beat: 60. / bpm,
            notes,
        }
    }
    pub fn iter_samples<'a>(&'a self) -> PlayerIter<'a> {
        let end_time = self.notes[0].dur * self.secs_per_beat;
        
        PlayerIter {
            player: self,
            note_index: 0,
            phase: 0.,
            time: 0.,
            end_time,
            start_time: 0.,
        }
    }
}

pub struct PlayerIter<'a> {
    player: &'a Player,
    note_index: usize,
    phase: f64,
    time: f64,
    end_time: f64,
    start_time: f64,
}

impl<'a> Iterator for PlayerIter<'a> {
    type Item=(f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let note = if self.time < self.end_time {
            self.player.notes[self.note_index]
        } else {
            self.note_index += 1;
            if self.note_index >= self.player.notes.len() {
                return None;
            }
            let note = self.player.notes[self.note_index];
            self.start_time = self.end_time;
            self.end_time += note.dur * self.player.secs_per_beat;
            note
        };

        let freq = note.pitch.freq().unwrap();
        let time = if (note.style & STYLE_ATTACK) != 0 {
            self.time - self.start_time
        } else {
            0.
        };
        self.phase += freq * self.player.time_per_sample;

        self.time += self.player.time_per_sample;
        Some((self.phase % 1., time))
    }
}