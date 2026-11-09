//! pairs-app: the weather on this screen. The dashboard bundle sends the data (a WMO weather
//! code); which face that is, and which picture shows a face, is decided here. The pictures are
//! this app's own component set (#pairs-weather, design/weather-pictures.js), not a component
//! of the UI library: the same kinds in both.

use crate::game::Face;
use everyware_ui::{Change, Component, ComponentKind, Slotted};

/// The face for a WMO weather interpretation code (the bundle's `weather_code`).
pub fn face_of(code: u32) -> Face {
    match code {
        0 | 1 => Face::Clear,
        2 => Face::Partly,
        3 => Face::Cloudy,
        45 | 48 => Face::Fog,
        51..=67 | 80..=82 => Face::Rain,
        71..=77 | 85 | 86 => Face::Snow,
        95..=99 => Face::Thunder,
        _ => Face::Cloudy,
    }
}

/// A face's picture: a variant of #pairs-weather.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Picture(pub Face);

impl Component for Picture {
    const KIND: ComponentKind = ComponentKind::App;

    fn variant(&self) -> String {
        let kind = match self.0 {
            Face::Clear => "clear",
            Face::Partly => "partly",
            Face::Cloudy => "cloudy",
            Face::Fog => "fog",
            Face::Rain => "rain",
            Face::Snow => "snow",
            Face::Thunder => "thunder",
        };
        format!("kind={kind}")
    }

    fn last_change(&self) -> Option<Change> {
        None
    }
}

impl Slotted for Picture {
    const SET: &'static str = "#pairs-weather";
}
