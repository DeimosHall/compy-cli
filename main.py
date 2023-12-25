# Author: Francisco Torres (deimoshall.dev)
# Repository: https://github.com/DeimosHall/compy-cli.git
# Description: script to compress videos using ffmpeg

import subprocess
import sys
import os

# Check if ffmpeg is installed
try:
    subprocess.check_output(['ffmpeg', '-version'])
except:
    print('ffmpeg is not installed. Please install ffmpeg and try again.')
    sys.exit()

input_file = ''
output_file = ''
output_file_provided = False

# Check if the user provided the input file
if len(sys.argv) < 2:
    print('Please provide the input file')
    sys.exit()

input_source = sys.argv[1]

# Check if the input is a file or a directory
if os.path.isfile(input_source):
    input_file = input_source
else:
    print('Please provide a file')
    sys.exit()

# Check if the user provided the output file
if len(sys.argv) < 3:
    output_file = f'{input_file[::-1].split(".", 1)[1][::-1]} compressed.mp4' # Get the file name without the extension and add compressed.mp4
else:
    output_file_provided = True
    output_file = sys.argv[2]

delete_original_file = input('Do you want to delete the original file? (y/n): ')

def compress_video(input_file, output_file):
    hide_banner = '-v warning -hide_banner -stats'
    command = f'ffmpeg -i "{input_file}" {hide_banner} -vcodec libx264 -crf 23 -acodec aac -b:a 128k -map_metadata 0 "{output_file}"'
    subprocess.call(command, shell=True)

if os.path.isfile(input_source):
    compress_video(input_file, output_file)

if delete_original_file == 'y':
    # Delete the original file and remove the compressed word from the output file if needed
    original_file = input_file
    input_size = subprocess.check_output(f'du -sh "{input_file}"', shell=True).split()[0].decode('utf-8')
    output_size = subprocess.check_output(f'du -sh "{output_file}"', shell=True).split()[0].decode('utf-8')
    
    print(f'Original file size: {input_size}')
    print(f'Compressed file size: {output_size}')
    if input_size <= output_size:
        print('Compressed file is bigger than the original file. Original file not deleted')
        sys.exit()

    subprocess.call(f'rm "{input_file}"', shell=True)
    if not output_file_provided:
        subprocess.call(f'mv "{output_file}" "{original_file}"', shell=True)

    print('Original file deleted')
else:
    print('Original file not deleted')
