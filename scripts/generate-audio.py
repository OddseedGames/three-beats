import math
from pathlib import Path
import random
import struct
import wave


root = Path(__file__).resolve().parent.parent / "assets"
rng = random.Random(3)
for name, duration in [("hit", 0.18), ("bell", 0.32)]:
    frames = []
    for i in range(int(22050 * duration)):
        t = i / 22050
        if name == "hit":
            value = (rng.uniform(-1, 1) * 0.6 + math.sin(t * math.tau * 85) * 0.4) * math.exp(-t * 24)
        else:
            value = (math.sin(t * math.tau * 660) + math.sin(t * math.tau * 990) * 0.3) * math.exp(-t * 12) * 0.3
        frames.append(struct.pack("<h", int(value * 22000)))
    with wave.open(str(root / (name + ".wav")), "wb") as audio:
        audio.setnchannels(1)
        audio.setsampwidth(2)
        audio.setframerate(22050)
        audio.writeframes(b"".join(frames))
