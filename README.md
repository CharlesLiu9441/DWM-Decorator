# DWM-Decorator
This project uses windows api and provides border colorization, toggling window topmost, and adjust transparency.
## Core Features
- Change window border color depending on whether the window is active and whether the window is topmost.
- Use hotkey to make foreground window topmost.
- Use hotkey to adjust foreground window transparency.
- Extreme low RAM usage.
## Configuration
- The config file will be generated as `config.toml`.
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
```
## Key Bindings
- Use `Ctrl + Numpad 0` to toggle topmost
- Use `Ctrl + Numpad 2` to increase transparency
- Use `Ctrl + Numpad 8` to decrease transparency
## TODO List
- [ ] Add configuration of key bindings
- [ ] Add process blacklist
