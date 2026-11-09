//! pairs-app: state -> UI model, after every action.

use crate::game::{self, CARDS, PAIRS};
use crate::state::State;
use crate::ui_model::UiModel;
use crate::game::Face;
use crate::weather::Picture;
use everyware_ui::{grid, grid_shown, slot, slot_shown};
use harry::presentation::Presenter;

pub struct UiModelPresenter;

impl Presenter<State, UiModel> for UiModelPresenter {
    fn present(model: &mut UiModel, state: &State) {
        let g = &state.game;
        model.time = state.clock.time.clone();
        model.moves = g.moves.to_string();
        model.found = format!("{}/{PAIRS}", g.found);
        model.clock = game::mm_ss(g.elapsed(state.now));
        model.status = match g.finished {
            Some(d) => format!("Done in {} moves, {}\nScore {}", g.moves, game::mm_ss(d), g.score),
            None if g.score > g.found => format!("Score {}", g.score),
            None => " ".to_string(),
        };
        match g.joker {
            Some(kind) => {
                model.joker = slot_shown("#pairs/joker", &Picture(kind), true);
                model.joker_text = format!("{} counts double", game::label(kind));
            }
            None => {
                model.joker = slot_shown("#pairs/joker", &Picture(Face::Cloudy), false);
                model.joker_text = if state.weather.is_some() {
                    "From the next round".to_string()
                } else {
                    "No weather yet: no joker".to_string()
                };
            }
        }
        model.cards = grid("#pairs/cards", &state.cards, COLUMNS, PITCH);
        let faces: Vec<Picture> = g.faces.iter().map(|&face| Picture(face)).collect();
        model.faces = grid_shown("#pairs/faces", faces.iter().zip((0..CARDS).map(|i| g.is_up(i))), COLUMNS, PITCH);
        model.new_round = slot("#pairs/new-round", &state.new_round);
    }
}

/// The cells of the card grid: the button's size plus the gaps of pairs-patch.js (200 + 64,
/// 56 + 72). The faces' grid has the same pitch, its frame moved to the backs' middles.
const PITCH: (f32, f32) = (264.0, 128.0);
const COLUMNS: usize = 4;
