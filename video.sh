#!/bin/bash

# Check if the number of arguments is less than 1
if [ $# -lt 1 ]; then
    echo "Usage: $0 <input_video>"
    exit 1
fi

input_video="$1"  # Assign the first command line argument to input_video
filename=$(basename -- "$input_video")
filename_no_ext="${filename%.*}"
output_folder="${filename_no_ext}_frames"

mkdir -p "$output_folder"

ffmpeg -i "$input_video" "$output_folder/%d.png"
