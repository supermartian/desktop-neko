import os
import sys
import subprocess
import zipfile

def install_and_import():
    try:
        from PIL import Image
    except ImportError:
        print("Installing Pillow...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "Pillow"])
        from PIL import Image
    return Image

Image = install_and_import()

source_image_path = r"C:\Users\Super\Downloads\Gemini_Generated_Image_drj01drj01drj01d (1).png"
if not os.path.exists(source_image_path):
    source_image_path = r"C:\Users\Super\Downloads\Gemini_Generated_Image_drj01drj01drj01d.png"

print(f"Using source: {source_image_path}")

img = Image.open(source_image_path)
width, height = img.size

# 12 columns, 7 rows
frame_width = width // 12
frame_height = height // 7

# Ensure exact multiple for Slint
crop_width = frame_width * 12
crop_height = frame_height

rows = ["alert", "happy", "idle", "sleep", "walk_left", "walk_right", "unused"]

pkg_dir = "packages/custom_pet"
sprites_dir = os.path.join(pkg_dir, "sprites")
os.makedirs(sprites_dir, exist_ok=True)

manifest_text = f"""[package]
name = "Custom Neko"
version = "1.0.0"
author = "User"

[sprite]
width = {frame_width}
height = {frame_height}
fps = 10

"""

for i, row_name in enumerate(rows):
    if row_name == "unused":
        continue
    
    # Crop the row
    top = i * frame_height
    bottom = top + frame_height
    box = (0, top, crop_width, bottom)
    row_img = img.crop(box)
    
    # The output slice contains 12 frames
    out_path = os.path.join(sprites_dir, f"{row_name}.png")
    row_img.save(out_path)
    print(f"Saved {out_path}")
    
    manifest_text += f"""[[animations]]
name = "{row_name}"
sheet = "sprites/{row_name}.png"
frames = 12

"""

with open(os.path.join(pkg_dir, "manifest.toml"), "w") as f:
    f.write(manifest_text)

behaviors_text = """[behavior]
initial_state = "idle"
tick_rate_ms = 100

[[states]]
name = "idle"
animation = "idle"
[[states.transitions]]
condition = { type = "timer", min_seconds = 5.0, max_seconds = 10.0 }
target = "walk_right"

[[states]]
name = "walk_right"
animation = "walk_right"
movement = { direction = "right", speed_px_s = 60.0 }
[[states.transitions]]
condition = { type = "timer", min_seconds = 3.0, max_seconds = 8.0 }
target = "idle"

[[states]]
name = "walk_left"
animation = "walk_left"
movement = { direction = "left", speed_px_s = 60.0 }
[[states.transitions]]
condition = { type = "timer", min_seconds = 3.0, max_seconds = 8.0 }
target = "idle"

[[states]]
name = "sleep"
animation = "sleep"
[[states.transitions]]
condition = { type = "timer", min_seconds = 10.0, max_seconds = 20.0 }
target = "idle"

[[states]]
name = "happy"
animation = "happy"
[[states.transitions]]
condition = { type = "timer", min_seconds = 3.0, max_seconds = 5.0 }
target = "idle"

[[states]]
name = "alert"
animation = "alert"
[[states.transitions]]
condition = { type = "timer", min_seconds = 3.0, max_seconds = 5.0 }
target = "idle"

[[states]]
name = "fall"
animation = "idle"
[[states.transitions]]
condition = { type = "timer", min_seconds = 1.0, max_seconds = 2.0 }
target = "idle"
"""

with open(os.path.join(pkg_dir, "behaviors.toml"), "w") as f:
    f.write(behaviors_text)

print("Zipping package...")
zip_path = "packages/custom_pet.petpkg"
with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
    for root, dirs, files in os.walk(pkg_dir):
        for file in files:
            file_path = os.path.join(root, file)
            arcname = os.path.relpath(file_path, pkg_dir)
            zipf.write(file_path, arcname)

print(f"Successfully packaged to {zip_path}")
