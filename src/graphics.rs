use crate::{note::{Pitch, Player}, wbg};

enum Clef {
    Treble,
    Bass,
}
impl Clef {
    fn get_index(&self, pitch: Pitch, sharps: bool) -> Option<(i32, bool)> {
        match pitch {
            Pitch::Pitch(n) => {
                let octave = (n+9)/12;
                let offset = if sharps {
                    match (n+9) % 12 {
                        0|1 => 0,//c
                        2|3 => 1,
                        4 => 2,
                        5|6 => 3,
                        7|8 => 4,
                        9|10 => 5,
                        11 => 6,
                        _ => unreachable!()
                    }
                } else {
                    match (n+9) % 12 {
                        0 => 0,//c
                        1|2 => 1,
                        3|4 => 2,
                        5 => 3,
                        6|7 => 4,
                        8|9 => 5,
                        10|11 => 6,
                        _ => unreachable!()
                    }
                };
                let staff_offset = match self  {
                    Clef::Treble => 7 + 7*3,
                    Clef::Bass => 1 + 7*2,
                } - (octave*7 + offset) as i32;
                Some((staff_offset/2, staff_offset % 2 == 0))
            },
            Pitch::Rest => None,
        }
    }
}

pub struct Staff {
    text: Vec<Vec<char>>,
    b_poses: Vec<usize>,
    clefs: Vec<Clef>,
    time: (usize, usize),
}

impl Staff {
    pub fn new(width: u16, height: u16, n_instruments: usize) -> Self {
        let mut text = Vec::new();
        for _ in 0..height {
            let mut line = Vec::new();
            for _ in 0..width {
                line.push(' ');
            }
            text.push(line)
        }

        let height_per = (height / 2) as usize;
        let mut b_poses = Vec::new();
        let mut clefs = Vec::new();
        let time = (4,4);
        for i in 0..n_instruments {
            let b_pos = i * height_per + (height_per/2);
            for c in &mut text[b_pos-2] { *c = '\u{2500}'; }
            for c in &mut text[b_pos-1] { *c = '\u{2500}'; }
            for c in &mut text[b_pos] { *c = '\u{2500}'; }
            for c in &mut text[b_pos+1] { *c = '\u{2500}'; }
            for c in &mut text[b_pos+2] { *c = '\u{2500}'; }
            b_poses.push(b_pos);

            let clef = if i == 0 {Clef::Treble} else {Clef::Bass};
            match clef {
                /*
                       /\   
                    ---||---
                    ---/----
                    --/=\---
                    -||--|--
                    --\=/---
                      ._|    
                */
                Clef::Treble => {
                    text[b_pos-3][4] = '/';
                    text[b_pos-3][5] = '\\';

                    text[b_pos-2][4] = '|';
                    text[b_pos-2][5] = '|';

                    text[b_pos-1][4] = '/';

                    text[b_pos][3] = '/';
                    text[b_pos][4] = '=';
                    text[b_pos][5] = '\\';

                    text[b_pos+1][2] = '|';
                    text[b_pos+1][3] = '|';
                    text[b_pos+1][6] = '|';

                    text[b_pos+2][3] = '\\';
                    text[b_pos+2][4] = '=';
                    text[b_pos+2][5] = '/';

                    text[b_pos+3][3] = '.';
                    text[b_pos+3][4] = '_';
                    text[b_pos+3][5] = '|';
                },
                /*
                    -/==\--
                    -.---|:-
                    ----/---
                    -_/-----
                    --------
                */
                Clef::Bass => {
                    text[b_pos-2][1] = '/';
                    text[b_pos-2][2] = '=';
                    text[b_pos-2][3] = '=';
                    text[b_pos-2][4] = '\\';

                    text[b_pos-1][1] = '.';
                    text[b_pos-1][5] = '|';
                    text[b_pos-1][6] = ':';

                    text[b_pos][4] = '/';

                    text[b_pos+1][1] = '_';
                    text[b_pos+1][2] = '/';
                    text[b_pos+1][3] = '/';
                },
            }
            clefs.push(clef);

            text[b_pos-1][8] = char::from_u32(48+time.0 as u32).unwrap();
            text[b_pos+1][8] = char::from_u32(48+time.1 as u32).unwrap();
            
        }

        Self {
            text,
            b_poses,
            clefs,
            time,
        }
    }

    pub fn write(&mut self, players: &[&Player]) {
        let mut note_iterators = players.iter().map(|i| i.notes.iter()).collect::<Vec<_>>();
        let mut beat_progress = vec![0.; players.len()];
        let mut on = vec![true; players.len()];
        let mut x = 11;

        loop {
            let min_beat_progress = beat_progress.iter().fold(1e32f64, |a, c| a.min(*c));
            for i in 0..players.len() {
                if !on[i] {continue;}
                if beat_progress[i] == min_beat_progress {
                    // Pull next note
                    match note_iterators[i].next() {
                        Some(note) => {
                            match self.clefs[i].get_index(note.pitch, true) {
                                Some((index, half)) => {
                                    let row = (self.b_poses[i] as i32 + index) as usize;
                                    if half {
                                        self.text[row][x] = '\u{2580}';
                                        self.text[row-1][x] = '\u{2584}';
                                    } else {
                                        self.text[row][x] = '\u{2588}';
                                    }
                                    self.text[row][x+1] = '\u{2518}';
                                    // self.text[row-1][x+1] = '\u{253c}';
                                    // self.text[row-2][x+1] = '\u{253c}';
                                    self.text[row-1][x+1] = '\u{2502}';
                                    self.text[row-2][x+1] = '\u{2502}';
                                    if half {
                                        self.text[row-3][x+1] = '\u{2577}';
                                    }
                                    if note.dur == 0.5 {
                                        self.text[row-2][x+2] = '\\';
                                        self.text[row-1][x+2] = '/';
                                    }
                                    if note.dur == 1.5 {
                                        self.text[row][x+2] = '.';
                                    }
                                },
                                None => {
                                    let row = self.b_poses[i];
                                    self.text[row-1][x] = '7';
                                    self.text[row][x] = '7';
                                    self.text[row+1][x] = '7';
                                }
                            }
                            beat_progress[i] += note.dur;
                            x += 4;
                        },
                        None => {
                            on[i] = false;
                            beat_progress[i] = 1e32;
                        },
                    }
                }
            }
            if !on.iter().fold(false, |a, c| a || *c) {
                break;
            }
        }
    }

    pub fn to_string(&self) -> String {
        self.text.iter().map(|l| format!("{}\n", l.iter().collect::<String>())).collect()
    }
}