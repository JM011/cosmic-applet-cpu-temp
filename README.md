# Cosmic CPU Temperature Applet

A simple applet for the COSMIC desktop environment that displays CPU temperature in the panel.

## Features

- Displays CPU temperature in degrees Celsius
- Updates every 2 seconds
- Uses native Rust `sysinfo` crate for efficient temperature reading (no external dependencies)

## Requirements

- Rust toolchain

  (Only if you build from source)
- `libxkbcommon-dev`
-  `pkg-config`

## Building

```bash
sudo apt-get install -y libxkbcommon-dev pkg-config
cargo build --release
```

Installation

1. Build and install the binary:
   ```bash
   cargo build --release
   sudo cp target/release/cosmic-applet-cpu-temp /usr/local/bin/
   ```
   Or for user installation:
   ```bash
   cp target/release/cosmic-applet-cpu-temp ~/.local/bin/
   ```

2. Install the desktop file:
   ```bash
   sudo cp com.system76.CosmicAppletCpuTemp.desktop /usr/share/applications/
   ```
   Or for user installation:
   ```bash
   cp com.system76.CosmicAppletCpuTemp.desktop ~/.local/share/applications/
   ```

3. Add to panel configuration in `~/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings`:
   ```
   "com.system76.CosmicAppletCpuTemp",
   ```

5. Restart the panel or log out and back in.

## License

MIT
