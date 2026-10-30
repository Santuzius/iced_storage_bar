//! A horizontal bar showing how much of a fixed capacity is used: a rounded fill on a rounded track, coloured by the fraction — accent below the warning threshold, warning up to the danger threshold, danger from there on.
//!
//! ```no_run
//! # type Element<'a> = iced_widget::core::Element<'a, (), iced_widget::core::Theme, ()>;
//! use iced_storage_bar::storage_bar;
//!
//! let bar: Element = storage_bar(0.82).height(6.0).danger_at(0.95).warning_at(None).into();
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

/// The fractions at which the fill changes colour; `None` turns that colour off.
///
/// The bar is rebuilt on every draw, so an app keeps its `Thresholds` in its own state or settings (with the `serde` feature they serialise; a missing field takes its default, `null` turns it off) and hands them to [`StorageBar::thresholds`] each time; changing them takes effect on the next draw.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Thresholds {
    /// The fill switches to [`BarFill::Warning`] from this fraction on.
    pub warning: Option<f32>,
    /// The fill switches to [`BarFill::Danger`] from this fraction on; it wins over `warning` if it is lower.
    pub danger: Option<f32>,
}

impl Thresholds {
    /// Warning from 70 %, danger from 90 %.
    pub const DEFAULT: Self = Self {
        warning: Some(0.7),
        danger: Some(0.9),
    };

    /// The accent colour whatever the fraction.
    pub const NONE: Self = Self {
        warning: None,
        danger: None,
    };

    /// The fill colour at this fraction.
    pub fn fill(&self, fraction: f32) -> BarFill {
        let reached = |threshold: Option<f32>| threshold.is_some_and(|t| fraction >= t);
        if reached(self.danger) {
            BarFill::Danger
        } else if reached(self.warning) {
            BarFill::Warning
        } else {
            BarFill::Accent
        }
    }
}

impl Default for Thresholds {
    /// [`Thresholds::DEFAULT`].
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The fraction `used` makes of `total`, for [`storage_bar`]; 0 when `total` is 0 (an unknown or unlimited quota).
pub fn fraction(used: u64, total: u64) -> f32 {
    if total == 0 {
        0.0
    } else {
        (used as f64 / total as f64) as f32
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
    width: Length,
    height: f32,
    thresholds: Thresholds,
    style: Rc<dyn Fn(&Theme) -> Style + 'a>,
}

/// A bar filled to `fraction` (clamped to 0..=1; [`fraction`] computes it from bytes), 8 px high, stretching to its container, with the [`Thresholds::DEFAULT`] and the [`default`] style.
pub fn storage_bar<'a>(fraction: f32) -> StorageBar<'a> {
    StorageBar {
        fraction,
        width: Length::Fill,
        height: 8.0,
        thresholds: Thresholds::default(),
        style: Rc::new(default),
    }
}

impl<'a> StorageBar<'a> {
    /// Width of the bar; [`Length::Fill`] by default.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Height in logical pixels; the ends are rounded with half of it.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// The fraction from which the fill uses the warning colour: `.warning_at(0.8)`, or `.warning_at(None)` for never.
    pub fn warning_at(mut self, fraction: impl Into<Option<f32>>) -> Self {
        self.thresholds.warning = fraction.into();
        self
    }

    /// The fraction from which the fill uses the danger colour: `.danger_at(0.95)`, or `.danger_at(None)` for never.
    pub fn danger_at(mut self, fraction: impl Into<Option<f32>>) -> Self {
        self.thresholds.danger = fraction.into();
        self
    }

    /// Both thresholds at once, such as the ones from the app's settings; [`Thresholds::NONE`] keeps the accent colour.
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
            width,
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
            .width(width)
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
    use super::{BarFill, Thresholds, fraction};

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
            warning: Some(0.8),
            danger: Some(0.95),
        };
        assert_eq!(t.fill(0.79), BarFill::Accent);
        assert_eq!(t.fill(0.9), BarFill::Warning);
        assert_eq!(t.fill(0.95), BarFill::Danger);
    }

    #[test]
    fn thresholds_turn_off() {
        assert_eq!(Thresholds::NONE.fill(1.0), BarFill::Accent);
        let only_danger = Thresholds {
            warning: None,
            ..Thresholds::DEFAULT
        };
        assert_eq!(only_danger.fill(0.8), BarFill::Accent);
        assert_eq!(only_danger.fill(0.9), BarFill::Danger);
    }

    #[test]
    fn fraction_of_bytes() {
        assert_eq!(fraction(512, 1024), 0.5);
        assert_eq!(fraction(10, 0), 0.0);
    }

    /// A settings file names what differs: a missing field keeps its default, `null` turns it off.
    #[cfg(feature = "serde")]
    #[test]
    fn thresholds_from_settings() {
        let t: Thresholds = serde_json::from_str(r#"{"warning": null}"#).unwrap();
        assert_eq!(t, Thresholds { warning: None, danger: Some(0.9) });
        let t: Thresholds = serde_json::from_str("{}").unwrap();
        assert_eq!(t, Thresholds::DEFAULT);
    }
}
