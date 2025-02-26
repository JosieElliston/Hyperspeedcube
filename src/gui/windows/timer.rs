use instant::{Duration, Instant};

use super::Window;

// TODO: use linear approximation in keybind references too

pub(crate) const TIMER: Window = Window {
    name: "Timer",
    build: |ui, app| {
        ui.add(egui::Button::new(autosize_button_text(
            ui,
            &if app.puzzle.has_been_fully_scrambled() {
                match if app.prefs.interaction.timer_blind_mode {
                    &app.timer.blind
                } else {
                    &app.timer.sight
                } {
                    Stopwatch::NotStarted => "Ready".to_string(),
                    Stopwatch::Running(start) => duration_to_string(start.elapsed()),
                    Stopwatch::Stopped(duration) => duration_to_string(*duration),
                }
            } else {
                "Scramble".to_string()
            },
            ui.available_width() - ui.spacing().button_padding.x * 2.0,
        )));
    },
    ..Window::DEFAULT
};

fn text_and_width_of_font_size(
    ui: &egui::Ui,
    mut text: egui::RichText,
    font_size: f32,
) -> (egui::RichText, f32) {
    // this function signature is annoying but idk how else to use text.size without cloning
    text = text.size(font_size);
    let text_size = egui::WidgetText::RichText(text.clone())
        .into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button)
        .size();
    (text, text_size.x)
}

/// returns a RichText whose width is close to but no larger than the target width
fn autosize_button_text(ui: &egui::Ui, text: &str, target_width: f32) -> egui::RichText {
    // use that width of text is ~linear in font size to generate an initial guess then fix it
    let mut text = egui::RichText::new(text);
    let initial_font_size = 100.0;
    let initial_width;
    (text, initial_width) = text_and_width_of_font_size(ui, text, initial_font_size);
    let font_size_per_width = initial_font_size / initial_width;
    let mut font_size = (target_width * font_size_per_width).max(2.0);
    let mut width;
    (text, width) = text_and_width_of_font_size(ui, text, font_size);
    // this should only run at most 4 times, typically 0 or 1
    while width > target_width && font_size > 2.0 {
        // point sizes have a resolution of ~0.5
        font_size = (font_size - 0.5).max(2.0);
        (text, width) = text_and_width_of_font_size(ui, text, font_size);
    }
    debug_assert!(width <= target_width);
    text
}

#[derive(Debug)]
pub(crate) enum Stopwatch {
    NotStarted,
    Running(Instant),
    Stopped(Duration),
}
impl Stopwatch {
    fn reset(&mut self) {
        *self = Stopwatch::NotStarted;
    }

    fn start(&mut self) {
        if let Self::NotStarted = self {
            *self = Self::Running(Instant::now());
        } else {
            debug_assert!(false, "Can only start a NotStarted timer. This is a horrible unrecoverable logic error in the scope of timer, but it's recoverable in the scope of the entire program.");
            self.reset();
        }
    }

    fn stop(&mut self) {
        if let Self::Running(beginning) = *self {
            *self = Self::Stopped(beginning.elapsed());
        } else {
            debug_assert!(false, "Can only stop a Running timer. This is a horrible unrecoverable logic error in the scope of timer, but it's recoverable in the scope of the entire program.");
            self.reset();
        }
    }
}

#[derive(Debug)]
pub(crate) struct Timer {
    sight: Stopwatch,
    blind: Stopwatch,
}
impl Timer {
    pub(crate) fn new() -> Self {
        Self {
            sight: Stopwatch::NotStarted,
            blind: Stopwatch::NotStarted,
        }
    }

    pub(crate) fn on_puzzle_reset(&mut self) {
        self.sight.reset();
        self.blind.reset();
    }

    pub(crate) fn on_scramble(&mut self) {
        self.sight.reset();
        self.blind.reset();
        self.blind.start();
    }

    pub(crate) fn on_non_rotation_twist(&mut self) {
        // check if the twist is the first one
        if matches!(self.sight, Stopwatch::NotStarted) {
            self.sight.start();
        }
    }

    pub(crate) fn on_solve(&mut self) {
        self.sight.stop();
    }

    pub(crate) fn on_blindfold_off(&mut self) {
        self.blind.stop();
    }
}

fn duration_to_string(duration: Duration) -> String {
    let milliseconds = duration.as_millis();
    let seconds = milliseconds / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    debug_assert_eq!(
        60 * 60 * 1000 * hours
            + 60 * 1000 * (minutes % 60)
            + 1000 * (seconds % 60)
            + milliseconds % 1000,
        duration.as_millis()
    );

    [
        if hours == 0 {
            "".to_owned()
        } else {
            format!("{}:", hours)
        },
        if minutes == 0 {
            "".to_owned()
        } else if hours == 0 {
            format!("{}:", minutes % 60)
        } else {
            format!("{:02}:", minutes % 60)
        },
        if minutes == 0 {
            format!("{}.", seconds % 60)
        } else {
            format!("{:02}.", seconds % 60)
        },
        format!("{:03}", milliseconds % 1000),
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_duration_to_str() {
        for (s, millis) in [
            ("0.000", 0),
            ("0.001", 1),
            ("0.010", 10),
            ("0.100", 100),
            ("1.000", 1000),
            ("10.000", 10000),
            ("1:00.000", 60000),
            ("1:01.000", 61000),
            ("1:10.000", 70000),
            ("10:00.000", 600000),
            ("11:00.000", 660000),
            ("11:10.000", 670000),
            ("11:11.000", 671000),
            ("1:00:00.000", 3600000),
            ("10:00:00.000", 36000000),
            ("100:00:00.000", 360000000),
            ("23:02:14.903", 82934903),
        ] {
            assert_eq!(s, duration_to_string(Duration::from_millis(millis)));
        }
    }
}
