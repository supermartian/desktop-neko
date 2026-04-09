import os
from PIL import Image, ImageDraw

def create_sprite(filename, text, size=64, frames=4, color="blue"):
    width = size * frames
    height = size
    img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    
    for i in range(frames):
        x_offset = i * size
        # Draw a little box
        d.rectangle([x_offset + 10, 10, x_offset + size - 10, size - 10], fill=color)
        d.text((x_offset + 15, size//2), f"{text} {i}", fill="white")
    
    img.save(filename)

os.makedirs("packages/neko/sprites", exist_ok=True)
create_sprite("packages/neko/sprites/idle.png", "IDL", frames=4, color="gray")
create_sprite("packages/neko/sprites/walk_right.png", "RGT", frames=4, color="green")
create_sprite("packages/neko/sprites/walk_left.png", "LFT", frames=4, color="blue")
create_sprite("packages/neko/sprites/alert.png", "ALRT", frames=2, color="orange")
create_sprite("packages/neko/sprites/sleep.png", "Zzz", frames=4, color="purple")
create_sprite("packages/neko/sprites/happy.png", "YAY", frames=2, color="pink")
