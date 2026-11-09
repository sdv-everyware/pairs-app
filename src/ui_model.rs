//! pairs-app: the UI model, one field per layer of #pairs-root the app writes to or taps (design:
//! design/pairs-patch.js). The names are the contract with the
//! design: rename both or neither.

use everyware_ui::model::prelude::*;

everyware_ui::ui_model! {
    time: String = "#pairs/time";
    moves: String = "#pairs/moves";
    found: String = "#pairs/found";
    clock: String = "#pairs/clock";
    status: String = "#pairs/status";
    joker: Slot = "#pairs/joker" ["SlotCustomizer"];
    joker_text: String = "#pairs/joker-text";
    new_round: Slot = "#pairs/new-round" ["SlotCustomizer"];
    cards: Slot = "#pairs/cards" ["SlotCustomizer"];
    faces: Slot = "#pairs/faces" ["SlotCustomizer"];
}
