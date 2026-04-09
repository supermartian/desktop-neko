# Creating Custom Desktop Pets!

A `.petpkg` file is just a standard ZIP archive (renamed from `.zip` to `.petpkg`) containing your sprites and two configuration files. 

### Structure of a `.petpkg`
If you extract a package, the internal folder structure looks exactly like this:
```text
manifest.toml
behaviors.toml
sprites/
  ├── idle.png
  ├── walk_right.png
  ├── walk_left.png
  └── fall.png
```

### 1. `manifest.toml`
This file defines the physical dimensions of your pet and organizes your sprite sheets (the images inside the `sprites/` folder).
```toml
[package]
name = "My Unique Neko"
version = "1.0.0"
author = "Your Name"

[sprite]
width = 64
height = 64
fps = 10

[[animations]]
name = "idle"
sheet = "sprites/idle.png"
frames = 4

[[animations]]
name = "walk_right"
sheet = "sprites/walk_right.png"
frames = 4
```
*Note: Make sure your `.png` sprite sheets strictly follow the `width` * `frames` layout (i.e. if width is 64 and frames is 4, the image must be exactly 256x64 pixels).*

### 2. `behaviors.toml`
This is the brain of your pet—a powerful state machine. It ties the `name` of the animation to physical movement rules and transition triggers (like timers or edge collisions).
```toml
[behavior]
initial_state = "idle"
tick_rate_ms = 100

[[states]]
name = "idle"
animation = "idle"
[[states.transitions]]
condition = { type = "timer", min_seconds = 5.0, max_seconds = 10.0 }
# You can use a single target:
# target = "walk_right"
# Or you can define multiple weighted targets for non-deterministic transitions:
targets = [
    { state = "walk_right", weight = 1.0 },
    { state = "walk_left", weight = 1.0 }
]

[[states]]
name = "walk_right"
animation = "walk_right"
movement = { direction = "right", speed_px_s = 60.0 }
[[states.transitions]]
condition = { type = "timer", min_seconds = 3.0, max_seconds = 8.0 }
target = "idle"
```

### 3. Zipping it up
Once your files are ready, select the `manifest.toml`, `behaviors.toml`, and the `sprites` folder together, and compress them into a `.zip` archive. 
**Crucial:** Do not zip the parent folder! Select the files directly so they sit at the "root" of the zip. Finally, rename `.zip` to `.petpkg`.
