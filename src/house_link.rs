//! pairs-app: the house's weather, for the joker (the news are not on this screen). The
//! dashboard bundle (vendor/sdv-everyware/everyware/bundles/dashboard) pushes the house's data
//! over binder to com.everyware.screen.IScreenLink/default, which the panel app serves; the pairs
//! screen takes the same name (one of the two runs at a time) through the shared link
//! (vendor/sdv-everyware/everyware/interfaces/screen/link). Everything but the weather is
//! dropped by the link's defaults.

use crate::actions::PairsAction;
use everyware_catalog::weather::Weather;
use everyware_screen_link::Screen;
use harry::reducer::Actions;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

struct Pairs {
    actions: Mutex<Sender<Actions<PairsAction>>>,
}

impl Screen for Pairs {
    fn weather(&self, weather: Weather) {
        log::info!("pairs: weather from the bundle: code {}, {:.0}°, {}", weather.code, weather.temperature_c, weather.place);
        let _ = self.actions.lock().expect("actions").send(Actions::CustomAction(PairsAction::Weather(weather)));
    }
}

/// Serves the link until the process ends; returns at once.
pub fn spawn(actions: Sender<Actions<PairsAction>>) {
    everyware_screen_link::spawn(Pairs { actions: Mutex::new(actions) });
}
