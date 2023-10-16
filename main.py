# Author: Francisco Torres (deimoshall.dev)
# Repository: https://github.com/DeimosHall/compy-cli.git
# Description: script to compress videos using ffmpeg

import subprocess
import sys

# Check if ffmpeg is installed
try:
    subprocess.check_output(['ffmpeg', '-version'])
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

# Ask the user if he wants to delete the original file
answer = input('Do you want to delete the original file? (y/n): ')

# Build the ffmpeg command with the provided arguments
hide_banner = '-v warning -hide_banner -stats'
command = f'ffmpeg -i "{input_file}" {hide_banner} -vcodec libx264 -crf 23 -acodec aac -b:a 128k -map_metadata 0 "{output_file}"'

# Run the command
subprocess.call(command, shell=True)

if answer == 'y':
    # Delete the original file and remove the compressed word from the output file
    original_file = input_file
    input_size = subprocess.check_output(f'du -sh "{input_file}"', shell=True).split()[0].decode('utf-8')
    output_size = subprocess.check_output(f'du -sh "{output_file}"', shell=True).split()[0].decode('utf-8')
    subprocess.call(f'rm "{input_file}"', shell=True)
    subprocess.call(f'mv "{output_file}" "{original_file}"', shell=True)
    print(f'Original file size: {input_size}')
    print(f'Compressed file size: {output_size}')
    print('Original file deleted')
else:
    print('Original file not deleted')
