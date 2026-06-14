# DWM-Decorator
This project uses Windows api and provides border colorization, toggling window topmost, and adjust transparency.
## Core Features
- Change window border color depending on whether the window is active and whether the window is topmost.
- Change window title color and title text color depending on whether the window is active (optional feature)
- Use hotkey to make foreground window topmost.
- Use hotkey to adjust foreground window transparency.
- Extreme low RAM usage.
## Logging
- The log file will be generated as `dwm-decorator.log.<date>`
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

---

# DWM-Decorator

该项目使用 Windows API，提供边框着色、切换窗口置顶以及调节透明度的功能。

## 核心功能

* 根据窗口是否激活以及窗口是否置顶，改变窗口边框颜色。
* 根据窗口是否激活，改变窗口标题颜色和标题文本颜色（可选功能）。
* 使用快捷键使前台窗口置顶。
* 使用快捷键调节前台窗口透明度。
* 极低的内存占用。

## 日志记录

* 日志文件将生成为 `dwm-decorator.log.<日期>`

## 配置

* 配置文件将生成为 `config.toml`。

```toml
# 强制色彩方案
# 可选值: "Dark", "Light", "NoChange"
force_color_scheme = "NoChange"

# 强制边框圆角
# 可选值: "Round", "Rect", "NoChange"
force_border_radius = "NoChange"

# --- 边框颜色设置 ---
# 可选值: "#RRGGBB", "rgb(r,g,b)" 等

# 激活状态下普通窗口的边框颜色
active_border_color = "#00aaff"

# 激活状态下置顶窗口的边框颜色
active_topmost_border_color = "#ffba00"

# 非激活状态下普通窗口的边框颜色
inactive_border_color = "#808080"

# 非激活状态下置顶窗口的边框颜色
inactive_topmost_border_color = "#775500"

# --- 按键配置 ---
key_toggle_topmost = "Ctrl+Keypad0"
key_increase_transparency = "Ctrl+Keypad2"
key_decrease_transparency = "Ctrl+Keypad8"

# --- 可选配置 ---
# 你可以使用 active_title_color、inactive_title_color、active_text_color、inactive_text_color
# 来配置窗口标题的颜色以及标题上的文本颜色。
# 例如：

# active_title_color = "#202020"
# inactive_title_color = "#202020"
# active_text_color = "#f0f8ff"
# inactive_text_color = "#c0c8cf"

# 以上配置为可选配置

```

## 默认键位绑定

* 使用 `Ctrl + Numpad 0` 切换置顶状态
* 使用 `Ctrl + Numpad 2` 增加透明度
* 使用 `Ctrl + Numpad 8` 减少透明度

## 系统要求

* $\ge$ Windows 11 (Build 22000)

## 待办事项

* [ ] 添加进程黑名单