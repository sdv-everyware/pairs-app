//! pairs-app: the ticker, the weather, a gesture on a node. A tap on a card's back turns it up
//! (game.rs decides what happens); New round deals again. The weather sets the joker: it
//! deals again at once while the round has not started, else from the next round.

use crate::clock::Clock;
use crate::game::{self, Face, Tap, CARDS};
use crate::state::State;
use everyware_catalog::weather::Weather;
use everyware_ui::button::ButtonEvent;
use everyware_ui::{row_of, Gesture};
use harry::side_effects::SideEffects;
use harry::state::StateAction;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub enum PairsAction {
    /// From the ticker thread (main.rs) every TICK: the wall clock and the instant.
    Tick(Clock, Instant),
    /// A gesture on a node (ui_event_listener.rs).
    Gesture(String, Gesture),
    /// The weather of the house (house_link.rs).
    Weather(Weather),
}

impl StateAction<State, ()> for PairsAction {
    fn apply(&self, state: &mut State, _side_effects: &mut Vec<SideEffects<()>>) {
        match self {
            PairsAction::Tick(clock, now) => {
                state.clock = clock.clone();
                state.now = *now;
                if state.game.settle(*now) {
                    log::info!("pairs: miss turned back");
                }
            }
            PairsAction::Weather(w) => on_weather(state, w),
            PairsAction::Gesture(node, gesture) => on_gesture(state, node, *gesture, Instant::now()),
        }
    }
}

fn on_weather(state: &mut State, w: &Weather) {
    let before = state.joker();
    state.weather = Some(w.clone());
    let joker = state.joker();
    if joker == before {
        return;
    }
    let kind = joker.unwrap_or(Face::Cloudy);
    if state.game.started.is_none() {
        state.deal();
        log::info!("pairs: weather code {} ({}) -> joker {kind:?}, dealt again: {:?}", w.code, w.place, state.game.faces);
    } else {
        log::info!("pairs: weather code {} ({}) -> joker {kind:?} from the next round", w.code, w.place);
    }
}

/// The card of a node "#pairs/cards/<i>" (a cell of the grid slot), as an index.
fn card_of(node: &str) -> Option<usize> {
    row_of("#pairs/cards", node).filter(|&i| i < CARDS)
}

fn on_gesture(state: &mut State, node: &str, gesture: Gesture, now: Instant) {
    if matches!(gesture, Gesture::Down { .. }) {
        log::info!("pairs: down on {node}");
    }
    state.now = now;
    if node == "#pairs/new-round" {
        if state.new_round.on(gesture) == Some(ButtonEvent::Clicked) {
            state.deal();
            log::info!("pairs: new round, joker {:?}: {:?}", state.game.joker, state.game.faces);
        }
        return;
    }
    let Some(i) = card_of(node) else { return };
    if state.cards[i].on(gesture) != Some(ButtonEvent::Clicked) {
        return;
    }
    let tap = state.game.tap(i, now);
    let g = &state.game;
    log::info!(
        "pairs: card {} {:?}: {tap:?}, moves {}, pairs {}/{}, score {}",
        i + 1,
        g.faces[i],
        g.moves,
        g.found,
        game::PAIRS,
        g.score
    );
    if tap == Tap::Match {
        state.sync_buttons();
        if let Some(d) = state.game.finished {
            log::info!("pairs: done in {} moves, {}", state.game.moves, game::mm_ss(d));
        }
    }
}

