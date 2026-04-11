// Copyright (c) 2026 Desktop Neko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::collections::HashMap;
use std::path::Path;
use slint::{SharedPixelBuffer, Rgba8Pixel};
use super::manifest::Manifest;
use super::behavior::BehaviorConfig;
use super::sprite::load_sprite_sheet;

pub struct LoadedPackage {
    pub manifest: Manifest,
    pub behavior: BehaviorConfig,
    pub animations: HashMap<String, Vec<SharedPixelBuffer<Rgba8Pixel>>>,
}

/// Load a package from a directory on disk.
/// The directory must contain `manifest.toml` and `behaviors.toml`.
/// Sprite sheets referenced in the manifest are resolved relative to the directory.
pub fn load_package_from_dir(dir: &Path) -> Result<LoadedPackage, String> {
    let manifest_path = dir.join("manifest.toml");
    let behavior_path = dir.join("behaviors.toml");

    if !manifest_path.exists() {
        return Err(format!("Missing manifest.toml in {}", dir.display()));
    }
    if !behavior_path.exists() {
        return Err(format!("Missing behaviors.toml in {}", dir.display()));
    }

    let manifest_str = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest.toml: {}", e))?;
    let manifest: Manifest = toml::from_str(&manifest_str)
        .map_err(|e| format!("Failed to parse manifest.toml: {}", e))?;

    let behavior_str = std::fs::read_to_string(&behavior_path)
        .map_err(|e| format!("Failed to read behaviors.toml: {}", e))?;
    let behavior: BehaviorConfig = toml::from_str(&behavior_str)
        .map_err(|e| format!("Failed to parse behaviors.toml: {}", e))?;

    let mut animations = HashMap::new();
    for anim in &manifest.animations {
        // Sheet paths in manifest are relative to the package dir (e.g. "sprites/idle.png")
        let sheet_path = dir.join(&anim.sheet);
        let sheet_bytes = std::fs::read(&sheet_path)
            .map_err(|e| format!("Failed to read sprite sheet '{}': {}", sheet_path.display(), e))?;
        let buffers = load_sprite_sheet(
            &sheet_bytes,
            anim.frames,
            manifest.sprite.width,
            manifest.sprite.height,
        )?;
        animations.insert(anim.name.clone(), buffers);
    }

    Ok(LoadedPackage {
        manifest,
        behavior,
        animations,
    })
}
