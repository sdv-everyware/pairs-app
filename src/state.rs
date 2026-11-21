//! pairs-app: everything the screen knows: the clock, the weather now, the round (game.rs) and
//! the buttons: the 12 card backs and New round.

use crate::clock::Clock;
use crate::game::{Face, Game, Rng, CARDS};
use everyware_catalog::weather::Weather;
use everyware_ui::button::{Button, ButtonKind};
use std::time::Instant;

pub struct State {
    pub clock: Clock,
    /// The instant of the last action: the round's clock reads it.
    pub now: Instant,
    pub weather: Option<Weather>,
    pub rng: Rng,
    pub game: Game,
    /// The backs of the cards: they take the taps (the cells of #pairs/cards).
    pub cards: [Button; CARDS],
    pub new_round: Button,
}

impl State {
    pub fn new(seed: u64, now: Instant) -> Self {
        let mut rng = Rng::new(seed);
        let game = Game::deal(&mut rng, None);
        let mut state = State {
            clock: Clock::default(),
            now,
            weather: None,
            rng,
            game,
            cards: std::array::from_fn(|_| Button::new(ButtonKind::Secondary, "")),
            new_round: Button::new(ButtonKind::Primary, "New round"),
        };
        state.sync_buttons();
        state
    }

    /// The joker for the next deal: the kind of the weather now.
    pub fn joker(&self) -> Option<Face> {
        self.weather.as_ref().map(|w| crate::weather::face_of(w.code))
    }

    /// Deals again, with the joker of the weather now.
    pub fn deal(&mut self) {
        let joker = self.joker();
        self.game = Game::deal(&mut self.rng, joker);
        self.sync_buttons();
    }

    /// The buttons follow the round: a found pair's backs are disabled, New round only when done.
    pub fn sync_buttons(&mut self) {
        for (i, card) in self.cards.iter_mut().enumerate() {
            card.press.set_enabled(!self.game.matched[i]);
        }
        self.new_round.press.set_enabled(self.game.is_done());
    }
}
