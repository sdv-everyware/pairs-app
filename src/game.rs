//! pairs-app: the round of the memory game, pure: no clock, no screen. Every call gets the
//! instant it happens at, so a test drives a round with made-up instants and a fixed seed.
//!
//! 12 cards, 6 pairs of weather kinds. The joker (the kind of the weather now) is always one of
//! the six and a matched joker pair scores two; without weather there is no joker. A tap turns a
//! card up; with two up a match stays up, a miss turns both back TURN_BACK later and taps in
//! between are ignored. The round's clock starts at the first tap and stops at the sixth pair.

use std::time::{Duration, Instant};

pub const CARDS: usize = 12;
pub const PAIRS: u32 = 6;
/// How long a miss stays up.
pub const TURN_BACK: Duration = Duration::from_secs(1);

/// A card's face: one of seven kinds of weather (weather.rs has the picture of each and the
/// face of the weather now).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    Clear,
    Partly,
    Cloudy,
    Fog,
    Rain,
    Snow,
    Thunder,
}

/// Every face.
pub const KINDS: [Face; 7] = [Face::Clear, Face::Partly, Face::Cloudy, Face::Fog, Face::Rain, Face::Snow, Face::Thunder];

/// A kind as the screen names it ("Rain counts double").
pub fn label(kind: Face) -> &'static str {
    match kind {
        Face::Clear => "Clear sky",
        Face::Partly => "Partly cloudy",
        Face::Cloudy => "Overcast",
        Face::Fog => "Fog",
        Face::Rain => "Rain",
        Face::Snow => "Snow",
        Face::Thunder => "Thunder",
    }
}

/// splitmix64: small, seeded, the same on the host and the board.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// 0..n
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// What a tap did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tap {
    /// Not taken: the card is up already, two are up, or the round is over.
    Ignored,
    /// The first card of a move is up.
    First,
    /// The second card matched its pair.
    Match,
    /// The second card missed: both turn back after TURN_BACK.
    Miss,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    /// The face of each card, left to right, top to bottom.
    pub faces: [Face; CARDS],
    /// The cards of found pairs: they stay up.
    pub matched: [bool; CARDS],
    /// The cards up in the current move (0, 1 or 2; 2 only while a miss waits to turn back).
    pub open: Vec<usize>,
    /// When the second card of a miss came up.
    pub miss_at: Option<Instant>,
    /// The joker of this deal; none without weather.
    pub joker: Option<Face>,
    pub moves: u32,
    /// Pairs found.
    pub found: u32,
    /// Pairs found, the joker's twice.
    pub score: u32,
    /// The first tap of the round.
    pub started: Option<Instant>,
    /// The round's time, once the sixth pair is found.
    pub finished: Option<Duration>,
}

impl Game {
    /// A new round: six kinds (the joker one of them), two cards each, shuffled.
    pub fn deal(rng: &mut Rng, joker: Option<Face>) -> Self {
        let mut kinds: Vec<Face> = KINDS.to_vec();
        // one of the seven stays out, never the joker
        let out: Vec<usize> = (0..kinds.len()).filter(|&i| Some(kinds[i]) != joker).collect();
        kinds.remove(out[rng.below(out.len())]);
        let mut cards: Vec<Face> = kinds.iter().flat_map(|&k| [k, k]).collect();
        for i in (1..cards.len()).rev() {
            cards.swap(i, rng.below(i + 1));
        }
        let mut faces = [Face::Clear; CARDS];
        faces.copy_from_slice(&cards);
        Game {
            faces,
            matched: [false; CARDS],
            open: Vec::new(),
            miss_at: None,
            joker,
            moves: 0,
            found: 0,
            score: 0,
            started: None,
            finished: None,
        }
    }

    /// The card shows its face.
    pub fn is_up(&self, card: usize) -> bool {
        self.matched[card] || self.open.contains(&card)
    }

    pub fn is_done(&self) -> bool {
        self.finished.is_some()
    }

    /// Turns a miss back once TURN_BACK has passed; true when it did.
    pub fn settle(&mut self, now: Instant) -> bool {
        match self.miss_at {
            Some(at) if now.duration_since(at) >= TURN_BACK => {
                self.open.clear();
                self.miss_at = None;
                true
            }
            _ => false,
        }
    }

    /// A tap on card `card` at `now`.
    pub fn tap(&mut self, card: usize, now: Instant) -> Tap {
        self.settle(now);
        if card >= CARDS || self.is_done() || self.miss_at.is_some() || self.is_up(card) {
            return Tap::Ignored;
        }
        self.started.get_or_insert(now);
        self.open.push(card);
        if self.open.len() < 2 {
            return Tap::First;
        }
        self.moves += 1;
        let (a, b) = (self.open[0], self.open[1]);
        if self.faces[a] != self.faces[b] {
            self.miss_at = Some(now);
            return Tap::Miss;
        }
        self.matched[a] = true;
        self.matched[b] = true;
        self.open.clear();
        self.found += 1;
        self.score += if Some(self.faces[a]) == self.joker { 2 } else { 1 };
        if self.found == PAIRS {
            self.finished = self.started.map(|s| now.duration_since(s));
        }
        Tap::Match
    }

    /// The round's time at `now`: zero before the first tap, frozen when done.
    pub fn elapsed(&self, now: Instant) -> Duration {
        match (self.finished, self.started) {
            (Some(d), _) => d,
            (None, Some(s)) => now.saturating_duration_since(s),
            (None, None) => Duration::ZERO,
        }
    }
}

/// "mm:ss"
pub fn mm_ss(d: Duration) -> String {
    let s = d.as_secs();
    format!("{:02}:{:02}", s / 60, s % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 42;

    fn game(joker: Option<Face>) -> Game {
        Game::deal(&mut Rng::new(SEED), joker)
    }

    /// The other card of `card`'s pair.
    fn partner(g: &Game, card: usize) -> usize {
        (0..CARDS).find(|&i| i != card && g.faces[i] == g.faces[card]).unwrap()
    }

    /// A card that is not `card`'s partner (and not `card`).
    fn stranger(g: &Game, card: usize) -> usize {
        (0..CARDS).find(|&i| i != card && g.faces[i] != g.faces[card]).unwrap()
    }

    fn ms(t0: Instant, ms: u64) -> Instant {
        t0 + Duration::from_millis(ms)
    }

    #[test]
    fn a_deal_has_six_pairs_and_the_joker() {
        for seed in 0..200 {
            for joker in KINDS {
                let g = Game::deal(&mut Rng::new(seed), Some(joker));
                for k in KINDS {
                    let n = g.faces.iter().filter(|&&f| f == k).count();
                    assert!(n == 0 || n == 2, "seed {seed}: {k:?} {n} times");
                }
                let kinds = KINDS.iter().filter(|k| g.faces.contains(k)).count();
                assert_eq!(kinds, 6, "seed {seed}");
                assert!(g.faces.contains(&joker), "seed {seed}: no joker {joker:?}");
            }
        }
    }

    #[test]
    fn the_seed_fixes_the_deal_and_the_next_round_differs() {
        assert_eq!(game(None).faces, game(None).faces);
        let mut rng = Rng::new(SEED);
        let first = Game::deal(&mut rng, None);
        let second = Game::deal(&mut rng, None);
        assert_ne!(first.faces, second.faces);
    }

    #[test]
    fn a_match_stays_up() {
        let t0 = Instant::now();
        let mut g = game(None);
        assert_eq!(g.tap(0, t0), Tap::First);
        let p = partner(&g, 0);
        assert_eq!(g.tap(p, ms(t0, 500)), Tap::Match);
        assert!(g.is_up(0) && g.is_up(p));
        assert!(g.open.is_empty());
        assert_eq!((g.moves, g.found, g.score), (1, 1, 1));
        g.settle(ms(t0, 5000));
        assert!(g.is_up(0) && g.is_up(p));
    }

    #[test]
    fn a_miss_turns_back_after_one_second() {
        let t0 = Instant::now();
        let mut g = game(None);
        let s = stranger(&g, 0);
        g.tap(0, t0);
        assert_eq!(g.tap(s, ms(t0, 200)), Tap::Miss);
        assert_eq!((g.moves, g.found), (1, 0));
        assert!(!g.settle(ms(t0, 1199)));
        assert!(g.is_up(0) && g.is_up(s));
        assert!(g.settle(ms(t0, 1200)));
        assert!(!g.is_up(0) && !g.is_up(s));
    }

    #[test]
    fn taps_while_two_are_up_are_ignored() {
        let t0 = Instant::now();
        let mut g = game(None);
        let s = stranger(&g, 0);
        let third = (0..CARDS).find(|&i| i != 0 && i != s).unwrap();
        g.tap(0, t0);
        g.tap(s, t0);
        assert_eq!(g.tap(third, ms(t0, 999)), Tap::Ignored);
        assert!(!g.is_up(third));
        assert_eq!(g.moves, 1);
        // a second later the miss is back down and the tap counts again
        assert_eq!(g.tap(third, ms(t0, 1000)), Tap::First);
        assert_eq!(g.open, vec![third]);
    }

    #[test]
    fn a_card_up_takes_no_second_tap() {
        let t0 = Instant::now();
        let mut g = game(None);
        g.tap(0, t0);
        assert_eq!(g.tap(0, t0), Tap::Ignored);
        assert_eq!(g.moves, 0);
    }

    #[test]
    fn the_joker_pair_counts_double() {
        let t0 = Instant::now();
        let mut g = game(Some(Face::Rain));
        let a = g.faces.iter().position(|&f| f == Face::Rain).unwrap();
        let other = g.faces.iter().position(|&f| f != Face::Rain).unwrap();
        g.tap(other, t0);
        g.tap(partner(&g, other), t0);
        assert_eq!(g.score, 1);
        g.tap(a, t0);
        g.tap(partner(&g, a), t0);
        assert_eq!((g.found, g.score), (2, 3));
    }

    #[test]
    fn no_weather_no_joker() {
        let t0 = Instant::now();
        let mut g = game(None);
        for card in 0..CARDS {
            if !g.is_up(card) {
                g.tap(card, t0);
                g.tap(partner(&g, card), t0);
            }
        }
        assert_eq!((g.found, g.score), (6, 6));
    }

    /// Plays a round to the end: one miss, then every pair, a second apart; the last match at t0 + 7 s.
    fn play_out(g: &mut Game, t0: Instant) {
        g.tap(0, t0);
        g.tap(stranger(g, 0), ms(t0, 100));
        let mut t = 2000;
        for card in 0..CARDS {
            if !g.is_up(card) {
                g.tap(card, ms(t0, t));
                g.tap(partner(g, card), ms(t0, t));
                t += 1000;
            }
        }
    }

    #[test]
    fn done_after_six_pairs() {
        let t0 = Instant::now();
        let mut g = game(Some(Face::Snow));
        assert_eq!(g.elapsed(ms(t0, 9000)), Duration::ZERO);
        play_out(&mut g, t0);
        assert!(g.is_done());
        assert_eq!((g.moves, g.found, g.score), (7, 6, 7));
        assert_eq!(g.finished, Some(Duration::from_millis(7000)));
        // the clock stops, taps are over
        assert_eq!(mm_ss(g.elapsed(ms(t0, 600_000))), "00:07");
        let snapshot = g.clone();
        assert_eq!(g.tap(0, ms(t0, 9000)), Tap::Ignored);
        assert_eq!(g, snapshot);
    }

    #[test]
    fn new_round_resets_everything() {
        let t0 = Instant::now();
        let mut rng = Rng::new(SEED);
        let mut g = Game::deal(&mut rng, Some(Face::Fog));
        play_out(&mut g, t0);
        let g2 = Game::deal(&mut rng, Some(Face::Fog));
        assert_eq!(g2.matched, [false; CARDS]);
        assert!(g2.open.is_empty() && g2.miss_at.is_none());
        assert_eq!((g2.moves, g2.found, g2.score), (0, 0, 0));
        assert_eq!((g2.started, g2.finished), (None, None));
        assert!(g2.faces.contains(&Face::Fog));
        assert_ne!(g2.faces, g.faces);
        assert_eq!(g2.elapsed(ms(t0, 99_000)), Duration::ZERO);
    }

    #[test]
    fn the_clock_text() {
        assert_eq!(mm_ss(Duration::from_secs(83)), "01:23");
        assert_eq!(mm_ss(Duration::from_millis(59_999)), "00:59");
    }
}
