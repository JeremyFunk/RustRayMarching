ffmpeg -r 24 -i results/%%01d.png  -vcodec libx264 -crf 22 -y results/movie.mp4