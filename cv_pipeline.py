import os
import sys
import subprocess
import zipfile

def install_deps():
    try:
        import cv2
        import numpy as np
        from PIL import Image
    except ImportError:
        print("Installing cv2, numpy, Pillow...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "opencv-python", "numpy", "Pillow"])
        import cv2
        import numpy as np
        from PIL import Image
    return cv2, np, Image

cv2, np, Image = install_deps()

source_path = r"C:\Users\Super\Downloads\Gemini_Generated_Image_drj01drj01drj01d (1).png"
if not os.path.exists(source_path):
    source_path = r"C:\Users\Super\Downloads\Gemini_Generated_Image_drj01drj01drj01d.png"

print(f"Loading {source_path}")
img = cv2.imread(source_path, cv2.IMREAD_UNCHANGED)

# Add alpha channel if missing
if img.shape[2] == 3:
    img = cv2.cvtColor(img, cv2.COLOR_BGR2BGRA)

# Make white background transparent
lower_white = np.array([240, 240, 240, 255])
upper_white = np.array([255, 255, 255, 255])
white_mask = cv2.inRange(img, lower_white, upper_white)
# Optional: find exactly white background component 
# Alternatively, any pixel with r>240, g>240, b>240 gets A=0
img[white_mask == 255] = [0, 0, 0, 0]

height, width = img.shape[:2]
row_height = height // 7

rows_config = ["alert", "happy", "idle", "sleep", "walk_left", "walk_right"]

# First pass: Extract all frames to find global max bounding box
raw_frames = [] # list of lists: raw_frames[row_index][frame_index] = cropped_rgba

for i in range(6):
    row_img = img[i * row_height : (i + 1) * row_height, 0:width]
    
    # Threshold for finding contours (non-transparent pixels)
    gray = cv2.cvtColor(row_img, cv2.COLOR_BGRA2GRAY)
    _, thresh = cv2.threshold(gray, 1, 255, cv2.THRESH_BINARY)
    
    # Find contours
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    # Filter small noise
    valid_boxes = []
    for cnt in contours:
        x, y, w, h = cv2.boundingRect(cnt)
        if w > 20 and h > 20: 
            valid_boxes.append((x, y, w, h))
            
    # Sort boxes from left to right
    valid_boxes.sort(key=lambda b: b[0])
    
    # If we mapped exactly 12 cats (sometimes AI joins them, let's just pick top 12 or split evenly)
    # Actually, we can just split the image perfectly into 12 columns, and THEN shrink to fit the bounding box within that column.
    
    col_width = width // 12
    row_frames = []
    for c in range(12):
        col_img = row_img[:, c * col_width : (c + 1) * col_width]
        c_gray = cv2.cvtColor(col_img, cv2.COLOR_BGRA2GRAY)
        _, c_thresh = cv2.threshold(c_gray, 1, 255, cv2.THRESH_BINARY)
        c_contours, _ = cv2.findContours(c_thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        
        if len(c_contours) > 0:
            c_boxes = [cv2.boundingRect(cnt) for cnt in c_contours]
            bx = min([b[0] for b in c_boxes])
            by = min([b[1] for b in c_boxes])
            bw = max([b[0]+b[2] for b in c_boxes]) - bx
            bh = max([b[1]+b[3] for b in c_boxes]) - by
            
            # Crop to isolated cat
            isolated = col_img[by:by+bh, bx:bx+bw]
            row_frames.append(isolated)
        else:
            # Empty frame?
            row_frames.append(np.zeros((10, 10, 4), dtype=np.uint8))
            
    raw_frames.append(row_frames)

# Second pass: Compute global max width and height
max_w = max([max([f.shape[1] for f in r]) for r in raw_frames])
max_h = max([max([f.shape[0] for f in r]) for r in raw_frames])

# Add some padding
pad = 10
final_w = max_w + pad*2
final_h = max_h + pad*2

print(f"Optimal frame size calculated: {final_w} x {final_h}")

pkg_dir = "packages/custom_pet"
sprites_dir = os.path.join(pkg_dir, "sprites")
os.makedirs(sprites_dir, exist_ok=True)

manifest_text = f"""[package]
name = "Ultimate Neko"
version = "2.0.0"
author = "CV Master"

[sprite]
width = {final_w}
height = {final_h}
fps = 10

"""

# Third pass: Paste into arranged row grids
for i, row_name in enumerate(rows_config):
    canvas = np.zeros((final_h, final_w * 12, 4), dtype=np.uint8)
    
    for c, frame in enumerate(raw_frames[i]):
        fh, fw = frame.shape[:2]
        
        # Center the frame at the bottom (so they walk on the ground symmetrically)
        # bottom alignment instead of perfect center creates stable gravity.
        x_offset = c * final_w + (final_w - fw) // 2
        y_offset = final_h - fh - pad # bottom anchor
        
        # In case height is 0 (empty frame)
        if fh > 0 and fw > 0:
            canvas[y_offset : y_offset+fh, x_offset : x_offset+fw] = frame
            
    out_path = os.path.join(sprites_dir, f"{row_name}.png")
    cv2.imwrite(out_path, canvas)
    
    manifest_text += f"""[[animations]]
name = "{row_name}"
sheet = "sprites/{row_name}.png"
frames = 12

"""

with open(os.path.join(pkg_dir, "manifest.toml"), "w") as f:
    f.write(manifest_text)

import shutil

print("Zipping refined package...")
zip_path = "packages/custom_pet.petpkg"
if os.path.exists(zip_path):
    os.remove(zip_path)

shutil.make_archive("packages/custom_pet", 'zip', pkg_dir)
os.rename("packages/custom_pet.zip", zip_path)

print("CV packaging complete!")
