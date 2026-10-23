//! A horizontal bar showing how much of a fixed capacity is used: a rounded fill on a rounded track, coloured by the fraction — accent below the warning threshold, warning up to the danger threshold, danger from there on.
//!
//! ```no_run
//! # type Element<'a> = iced_widget::core::Element<'a, (), iced_widget::core::Theme, ()>;
//! use iced_storage_bar::storage_bar;
//!
//! let bar: Element = storage_bar(0.82).height(6.0).danger_at(0.95).into();
//! ```

use std::rc::Rc;

use iced_widget::{
    Space, container,
    core::{Color, Element, Length, Theme, border},
    row,
};

/// Which colour the fill uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarFill {
    Accent,
    Warning,
    Danger,
}

/// The fractions at which the fill changes colour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Thresholds {
    /// The fill switches to [`BarFill::Warning`] at this fraction.
    pub warning: f32,
    /// The fill switches to [`BarFill::Danger`] at this fraction; it wins over `warning` if it is lower.
    pub danger: f32,
}

impl Default for Thresholds {
    /// Warning from 70 %, danger from 90 %.
    fn default() -> Self {
        Self {
            warning: 0.7,
            danger: 0.9,
        }
    }
}

impl Thresholds {
    /// The fill colour at this fraction.
    pub fn fill(&self, fraction: f32) -> BarFill {
        if fraction >= self.danger {
            BarFill::Danger
        } else if fraction >= self.warning {
            BarFill::Warning
        } else {
            BarFill::Accent
        }
    }
}

/// The colours the bar draws with.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// Fill below the warning threshold.
    pub accent: Color,
    /// Fill from the warning threshold.
    pub warning: Color,
    /// Fill from the danger threshold.
    pub danger: Color,
    /// The unfilled track behind the fill.
    pub track: Color,
}

/// The theme's primary, warning and danger colours, on its strong background as the track.
pub fn default(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        accent: palette.primary.base.color,
        warning: palette.warning.base.color,
        danger: palette.danger.base.color,
        track: palette.background.strong.color,
    }
}

/// A storage bar; build it with [`storage_bar`] and turn it into an [`Element`] with `.into()`.
pub struct StorageBar<'a> {
    fraction: f32,
    height: f32,
    thresholds: Thresholds,
    style: Rc<dyn Fn(&Theme) -> Style + 'a>,
}

/// A bar filled to `fraction` (clamped to 0..=1), 8 px high, stretching to its container, with the [`Thresholds::default`] and the [`default`] style.
pub fn storage_bar<'a>(fraction: f32) -> StorageBar<'a> {
    StorageBar {
        fraction,
        height: 8.0,
        thresholds: Thresholds::default(),
        style: Rc::new(default),
    }
}

impl<'a> StorageBar<'a> {
    /// Height in logical pixels; the ends are rounded with half of it.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// The fraction from which the fill uses the warning colour; above 1 it never does.
    pub fn warning_at(mut self, fraction: f32) -> Self {
        self.thresholds.warning = fraction;
        self
    }

    /// The fraction from which the fill uses the danger colour; above 1 it never does.
    pub fn danger_at(mut self, fraction: f32) -> Self {
        self.thresholds.danger = fraction;
        self
    }

    /// Both thresholds at once.
    pub fn thresholds(mut self, thresholds: Thresholds) -> Self {
        self.thresholds = thresholds;
        self
    }

    /// Colours resolved from the theme at draw time, so the bar follows theme switches.
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self {
        self.style = Rc::new(style);
        self
    }
}

impl<'a, Message, Renderer> From<StorageBar<'a>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    fn from(bar: StorageBar<'a>) -> Self {
        let StorageBar {
            fraction,
            height,
            thresholds,
            style,
        } = bar;
        let filled = (fraction.clamp(0.0, 1.0) * 1000.0).round() as u16;
        let colour = thresholds.fill(fraction);
        let round = border::rounded(height / 2.0);
        let fill_style = Rc::clone(&style);
        let fill = container(Space::new())
            .width(Length::FillPortion(filled.max(1)))
            .height(Length::Fixed(height))
            .style(move |t: &Theme| {
                let colours = fill_style(t);
                container::Style::default()
                    .background(match colour {
                        BarFill::Accent => colours.accent,
                        BarFill::Warning => colours.warning,
                        BarFill::Danger => colours.danger,
                    })
                    .border(round)
            });
        let rest = Space::new().width(Length::FillPortion(1000 - filled));
        // The fill sits on a rounded track, so both ends of the bar are round.
        container(row![fill, rest])
            .width(Length::Fill)
            .style(move |t: &Theme| {
                container::Style::default()
                    .background(style(t).track)
                    .border(round)
            })
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::{BarFill, Thresholds};

    #[test]
    fn default_thresholds() {
        let t = Thresholds::default();
        assert_eq!(t.fill(0.0), BarFill::Accent);
        assert_eq!(t.fill(0.69), BarFill::Accent);
        assert_eq!(t.fill(0.7), BarFill::Warning);
        assert_eq!(t.fill(0.89), BarFill::Warning);
        assert_eq!(t.fill(0.9), BarFill::Danger);
    }

    #[test]
    fn custom_thresholds() {
        let t = Thresholds {
            warning: 0.8,
            danger: 0.95,
        };
        assert_eq!(t.fill(0.79), BarFill::Accent);
        assert_eq!(t.fill(0.9), BarFill::Warning);
        assert_eq!(t.fill(0.95), BarFill::Danger);
        // A threshold above 1 is never reached.
        let t = Thresholds {
            warning: 2.0,
            danger: 2.0,
        };
        assert_eq!(t.fill(1.0), BarFill::Accent);
    }
}
