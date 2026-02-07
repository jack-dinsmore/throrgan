use crate::{input::Input, note::{Player, PlayerIter}};

pub enum Node {
    Input { input: Input },
    Add { a: Box<Self>, b: Box<Self> },
    // Mul { a: Box<Self>, i: u32, },
    Damp { child: Box<Self>, lambda: f64 },
    Lpf { child: Box<Self>, a: f64, last: f64 },
}

impl Node {
    fn evaluate(&mut self, phase: f64, time: f64) -> f64 {
        match self {
            Node::Input { input } => input.get_sample(phase),
            Node::Add { a, b } => a.evaluate(phase, time) + b.evaluate(phase, time),
            // Node::Mul { a, b } => a.evaluate(phase, time) + b.evaluate(phase, time),
            Node::Damp { child, lambda } => {
                child.evaluate(phase, time) * (-time / *lambda).exp()
            },
            Node::Lpf { child, a, last } => {
                let now = *last * *a + child.evaluate(phase, time) * (1. - *a);
                *last = now;
                now
            },
        }
    }
}

pub struct Instrument {
    node: Node,
    pub player: Player,
}

impl Instrument {
    pub fn new(node: Node, player: Player) -> Self {
        Self {
            node,
            player,
        }
    }
    pub fn iter_samples<'a>(&'a mut self, n_instruments: usize) -> InstrumentIter<'a> {
        InstrumentIter {
            node: &mut self.node,
            iter: self.player.iter_samples(),
            ampl_per: 1. / n_instruments as f64
        }
    }
}

pub struct InstrumentIter<'a> {
    node: &'a mut Node,
    iter: PlayerIter<'a>,
    ampl_per: f64,
}
impl<'a> Iterator for InstrumentIter<'a> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let (phase, time) = match self.iter.next() {
            Some(t) => t,
            None => return None,
        };
        Some((self.node.evaluate(phase, time) * self.ampl_per * std::i16::MAX as f64) as i16)
    }
}