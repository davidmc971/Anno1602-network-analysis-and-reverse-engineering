http://trac.ffmpeg.org/wiki/Capture/Desktop
ffmpeg -video_size 1024x768 -framerate 25 -f x11grab -i :0.0+100,200 -f pulse -ac 2 -i default output.mkv


ffmpeg -f x11grab -video_size 800x600 -i :0.0 \
	-f pulse -ac 2 -i default \
	-c:v libaom-av1 -preset 8 \
	-c:a libopus \
	~/ffmpeg-record/out/$(date +%Y-%b-%d%a--%H-%M-%S | tr A-Z a-z).mkv


ffmpeg -f x11grab -s 800x600 -i :0.0 -r 5 -g 10 -vcodec vp9 -f matroska pipe:1

ffmpeg -f x11grab -s 800x600 -i :0.0 -r 5 -g 10 -vcodec vp9 -f matroska ~/ffmpeg-record/out/$(date +%Y-%b-%d%a--%H-%M-%S | tr A-Z a-z).mkv

ffmpeg -f x11grab -video_size 1920x1080 -framerate 25 -i $DISPLAY -f alsa -i default -c:v libx264 -preset ultrafast -c:a aac screen.mp4

ffmpeg -i input.mp4 -vcodec libx264 -crf 27 -preset veryfast -c:a copy -s 960x540 output.mp4

-crf 0

ffmpeg -f x11grab -video_size 800x600 -i :0.0 \
	-f pulse -ac 2 -i default \
	-c:v libaom-av1 -crf 0 \
	-c:a libopus \
	~/ffmpeg-record/out/$(date +%Y-%b-%d%a--%H-%M-%S | tr A-Z a-z).mkv


ffmpeg -f x11grab -video_size cif -framerate 25 -i :0.0 -c:v libvpx-vp9 out.webm

ffmpeg -f x11grab -video_size 800x600 -framerate 30 -i :0.0 \
  -f pulse -ac 1 -i default \
  -c:v libvpx-vp9 \
	-c:a libopus \
  ~/ffmpeg-record/out/$(date +%Y-%b-%d%a--%H-%M-%S | tr A-Z a-z)out.webm

ffmpeg -f x11grab -video_size 800x600 -framerate 30 -i :0.0 \
  -f pulse -ac 1 -i default \
  -c:v libvpx-vp9 \
	-c:a libopus \
  -f webm pipe:1


For WebRTC:
-pix_fmt yuv420p -c:v libx264 -bsf:v h264_mp4toannexb -b:v 2M -max_delay 0 -bf 0 -f h264