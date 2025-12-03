from PIL import Image, ImageDraw
import os

# Crea icona 1024x1024
size = 1024
img = Image.new(RGBA, (size, size), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Sfondo gradiente viola
for y in range(size):
    r = int(102 + (118-102) * y/size)
    g = int(126 + (75-126) * y/size)
    b = int(234 + (162-234) * y/size)
    draw.line([(0, y), (size, y)], fill=(r, g, b, 255))

# Cerchio centrale
margin = 150
draw.ellipse([margin, margin, size-margin, size-margin], fill=(255, 255, 255, 40))

# Simbolo play/audio
center = size // 2
draw.polygon([
    (center - 100, center - 150),
    (center - 100, center + 150),
    (center + 150, center)
], fill=(255, 255, 255, 230))

img.save(icon.png)
print("icon.png creata")

# Genera altre dimensioni
for s in [32, 128, 256, 512]:
    resized = img.resize((s, s), Image.LANCZOS)
    resized.save(f{s}x{s}.png)
    if s == 128:
        resized2x = img.resize((256, 256), Image.LANCZOS)
        resized2x.sfrom PIL import Image, ImageDraw
import os

# Crea icona 1024x1024
size = pip3 install Pillow --break-system-packages 2>/dev/null; cd ~/Projects/whisper-subtitles/src-tauri/icons && python3 create_icon.py
cd ~/Projects/whisper-subtitles/src-tauri/icons && curl -L -o icon.png "https://cdn-icons-png.flaticon.com/256/4666/4666770.png"
ls -la ~/Projects/whisper-subtitles/src-tauri/icons/

