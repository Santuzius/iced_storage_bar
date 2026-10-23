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

The crate currently builds on the [iced fork with Android support](https://github.com/Santuzius/iced) (tag `iced_android-v0.1.0`), which is iced 0.14 with patches.

## License

MIT, see [LICENSE](LICENSE).
