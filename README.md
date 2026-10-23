# iced-storage-bar

A horizontal bar for [iced](https://github.com/iced-rs/iced) that shows how much of a fixed capacity is used, such as a cloud storage quota. The fill and the track behind it have round ends, and the fill changes colour with the fraction used:

- **Accent** below the warning threshold (default 70 %)
- **Warning** from the warning threshold up to the danger threshold
- **Danger** from the danger threshold (default 90 %)

```rust
use iced_storage_bar::storage_bar;

// 82 % used: warning colour, 6 px high, stretching to its container.
let bar = storage_bar(0.82).height(6.0);

// Other thresholds and your own colours.
let bar = storage_bar(0.82)
    .warning_at(0.8)
    .danger_at(0.95)
    .style(|theme| iced_storage_bar::Style { track: theme.extended_palette().background.weak.color, ..iced_storage_bar::default(theme) });
```

Without `.style`, the colours come from the theme's palette: primary, warning, danger and the strong background for the track. The style callback runs at draw time, so the bar follows theme switches.

It needs iced 0.14. An app on an iced fork can make the bar use the fork's widgets, so Cargo builds only one copy of iced:

```toml
[patch.crates-io]
iced_widget = { git = "https://github.com/your/iced", tag = "your-tag" }
```

## License

MIT, see [LICENSE](LICENSE).
