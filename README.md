# Sandbox for different simulators. 

Simulators don't have UI, but the generate event data.

## Utils

Generate video from frames:
```
ffmpeg -framerate 30 -i data/images/%04d.png data/video/video.mp4
```