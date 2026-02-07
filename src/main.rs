mod input;
mod note;
mod instrument;
mod graphics;
mod util;

use input::Input;
use note::{Player, Note};
use instrument::{Instrument, Node};
use graphics::Staff;

fn celeste() -> (Vec<Note>, Vec<Note>) {
    const EL: Pitch = Pitch::new('e', 3);
    const FL: Pitch = Pitch::new('f', 3);
    const GL: Pitch = Pitch::new('g', 3).sh(); // G sharp
    const A: Pitch = Pitch::new('a', 3);
    const B: Pitch = Pitch::new('b', 3);
    const C: Pitch = Pitch::new('c', 4);
    const D: Pitch = Pitch::new('d', 4);
    const E: Pitch = Pitch::new('e', 4);
    const F: Pitch = Pitch::new('f', 4);
    (
        vec![
            Note::attack(A, 1.5),
            Note::attack(B, 0.5),
            Note::attack(C, 1.),
            Note::attack(F, 1.),

            Note::attack(GL, 1.),
            Note::attack(E, 0.5),
            Note::attack(D, 0.5),
            Note::attack(C, 1.),
            Note::attack(A, 1.),
            Note::attack(FL, 1.),

            Note::attack(D, 0.5),
            Note::attack(C, 0.5),
            Note::attack(B, 1.),
            Note::attack(FL, 1.),
            Note::attack(EL, 1.),

            Note::attack(E, 0.5),
            Note::attack(D, 0.5),
            Note::attack(C, 1.),
            Note::attack(B, 0.5),
            Note::attack(A, 0.5),
            Note::attack(GL, 0.5),
            Note::attack(A, 0.5),
        ],
        vec![
            Note::attack(A, 8.),
            Note::attack(A, 8.),
            Note::attack(A, 8.),
            Note::attack(A, 8.),
            // Note::attack(A, 8.),
            // Note::attack(A, 8.),
            // Note::attack(A, 8.),
            // Note::attack(A, 8.),
        ],
    )
}

fn main() {
    clear_wbg!();

    const SAMPLE_RATE: u32 = 44100;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let bpm = 240.;
    let (notes1, notes2) = celeste();

    let mut instrument1 = Instrument::new(
        Node::Lpf {
            child: Box::new(Node::Damp {
                child: Box::new(Node::Input { input: Input::Square }),
                lambda: 0.3,
            }),
            a: 0.9,
            last: 0.,
        },
        Player::new(notes1, SAMPLE_RATE, bpm)
    );

    // let mut instrument2 = Instrument::new(
    //     Node::Lpf {
    //         child: Box::new(Node::Damp {
    //             child: Box::new(Node::Input { input: Input::Square }),
    //             lambda: 0.3,
    //         }),
    //         a: 0.9,
    //         last: 0.,
    //     },
    //     Player::new(notes2, SAMPLE_RATE, bpm)
    // );

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|f| {
            let mut staff = Staff::new(f.area().width, f.area().height, 1);
            staff.write(&[&instrument1.player]);
            let text = staff.to_string();

            let p = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);

            f.render_widget(p, f.area());
        }).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => break,
                _ => ()
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();

    // for (sample1, sample2) in instrument1.iter_samples(2).zip(instrument2.iter_samples(2)) {
    //     writer.write_sample(sample1 + sample2).unwrap();
    // }
    // writer.finalize().unwrap();
}
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::note::Pitch;
