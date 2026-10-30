//! Four quotas in the light and the dark theme: below, between and above the default thresholds, and one with the thresholds turned off. `cargo run --example demo`

use iced::widget::{column, row, text};
use iced::{Element, Length, Theme};
use iced_storage_bar::{Thresholds, fraction, storage_bar};

const GIB: u64 = 1 << 30;

/// (label, used, total, thresholds)
const QUOTAS: [(&str, u64, u64, Thresholds); 4] = [
    ("Google Drive", 4 * GIB, 15 * GIB, Thresholds::DEFAULT),
    ("Proton Drive", 39 * GIB / 10, 5 * GIB, Thresholds::DEFAULT),
    ("Dropbox", 19 * GIB / 10, 2 * GIB, Thresholds::DEFAULT),
    ("NAS, thresholds off", 19 * GIB / 10, 2 * GIB, Thresholds::NONE),
];

fn panel<'a>(theme: Theme) -> Element<'a, ()> {
    let bars = QUOTAS.iter().map(|&(label, used, total, thresholds)| {
        let share = fraction(used, total);
        column![
            text(format!("{label}: {:.1} of {} GiB used ({:.0} %)", used as f64 / GIB as f64, total / GIB, share * 100.0)).size(14),
            storage_bar(share).height(6.0).thresholds(thresholds),
        ]
        .spacing(6)
        .into()
    });
    iced::widget::themer(
        Some(theme),
        iced::widget::container(column(bars).spacing(18)).padding(20).width(Length::Fill).height(Length::Fill).style(|t: &Theme| {
            iced::widget::container::Style::default().background(t.palette().background).color(t.palette().text)
        }),
    )
    .into()
}

fn view(_: &()) -> Element<'_, ()> {
    row![panel(Theme::Light), panel(Theme::Dark)].into()
}

fn main() -> iced::Result {
    iced::application(|| (), |_: &mut (), _: ()| {}, view)
        .window_size((760.0, 222.0))
        .run()
}
