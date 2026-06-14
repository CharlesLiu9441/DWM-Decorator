# DWM-Decorator
This project uses Windows api and provides border colorization, toggling window topmost, and adjust transparency.
## Core Features
- Change window border color depending on whether the window is active and whether the window is topmost.
- Change window title color and title text color depending on whether the window is active (optional feature)
- Use hotkey to make foreground window topmost.
- Use hotkey to adjust foreground window transparency.
- Extreme low RAM usage.
## Configuration
- The config file will be generated as `config.toml`.
## Logging
- The log file will be generated as `dwm-decorator.log.<date>`
```toml
# Force Color Scheme
# Acceptable value: "Dark", "Light", "NoChange"
force_color_scheme = "NoChange"

# Force Border Radius
# Acceptable value: "Round", "Rect", "NoChange"
force_border_radius = "NoChange"

# --- Border Color Settings ---
# Acceptable value: "#RRGGBB", "rgb(r,g,b)", etc.

# border color of active normal window
active_border_color = "#00aaff"

# border color of active topmost window
active_topmost_border_color = "#ffba00"

# border color of inactive normal window
inactive_border_color = "#808080"

# border color of inactive topmost window
inactive_topmost_border_color = "#775500"

# --- Key Configuration ---
key_toggle_topmost = "Ctrl+Keypad0"
key_increase_transparency = "Ctrl+Keypad2"
key_decrease_transparency = "Ctrl+Keypad8"

# --- Optional Configuration ---
# you can use active_title_color, inactive_title_color, active_text_color, inactive_text_color
# to configure the color of title of the window and text on the title.
# e.g.:

# active_title_color = "#202020"
# inactive_title_color = "#202020"
# active_text_color = "#f0f8ff"
# inactive_text_color = "#c0c8cf"

# these configurations are optional
```
## Default Key Bindings
- Use `Ctrl + Numpad 0` to toggle topmost
- Use `Ctrl + Numpad 2` to increase transparency
- Use `Ctrl + Numpad 8` to decrease transparency
## System Requirement
- $\ge$ Windows 11 (Build 22000)
## TODO List
- [ ] Add process blacklist
