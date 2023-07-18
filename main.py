# Author: Francisco Torres (deimoshall.dev)
# Repository: https://github.com/DeimosHall/compy.git
# Description: script to compress videos using ffmpeg

import subprocess
import sys

# Check if ffmpeg is installed
try:
    subprocess.call('ffmpeg -version', shell=True)
except:
    print('ffmpeg is not installed. Please install ffmpeg and try again.')
    sys.exit()

input_file = ''
output_file = ''

# Check if the user provided the input file
if len(sys.argv) < 2:
    print('Please provide the input file')
    sys.exit()

input_file = sys.argv[1]

# Check if the user provided the output file
if len(sys.argv) < 3:
    output_file = f'{input_file.split(".")[0]} compressed.mp4'
else:
    output_file = sys.argv[2]

print(f'Input file: {input_file}')
print(f'Output file: {output_file}')

# Build the ffmpeg command with the provided arguments
command = f'ffmpeg -i "{input_file}" -vcodec libx264 -crf 23 -acodec aac -b:a 128k -map_metadata 0 "{output_file}"'

# Run the command
subprocess.call(command, shell=True)
