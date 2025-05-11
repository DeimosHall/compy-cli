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
    # Check if the user provided the output file
    if len(sys.argv) < 3:
        # Get the file name without the extension and add compressed.mp4
        output_file = f'{
            input_file[::-1].split(".", 1)[1][::-1]} compressed.mp4'
    else:
        output_file_provided = True
        output_file = sys.argv[2]

delete_original_file = input(
    'Do you want to delete the original file? (y/n): ')


def compress_video(input_file, output_file):
    hide_banner = '-v warning -hide_banner -stats'
    # command = f'ffmpeg -i "{input_file}" {hide_banner} -vf "scale=-1:720" -vcodec libx264 -crf 23 -acodec aac -b:a 128k -map_metadata 0 "{output_file}"'
    command = f'ffmpeg -i "{input_file}" {
        hide_banner} -vcodec libx264 -crf 23 -acodec aac -b:a 128k -map_metadata 0 "{output_file}"'
    subprocess.call(command, shell=True)


def get_video_duration(file) -> str:
    duration = subprocess.check_output(
        f'ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 "{file}"', shell=True).decode('utf-8')
    # Format HH:MM:SS
    duration = int(float(duration))
    hours = duration // 3600
    minutes = (duration % 3600) // 60
    seconds = duration % 60
    return f'{hours:02}:{minutes:02}:{seconds:02}'


def clone_metadata(input_file, output_file):
    command = f'exiftool -tagsFromFile "{input_file}" -extractEmbedded -all:all -FileModifyDate -overwrite_original "{output_file}"'
    subprocess.call(command, shell=True)


def compare_size(input_file, output_file):
    if delete_original_file == 'y':
        # Delete the original file and remove the compressed word from the output file if needed
        original_file = input_file
        # input_size = subprocess.check_output(
        #     f'du -sh "{input_file}"', shell=True).split()[0].decode('utf-8')
        # output_size = subprocess.check_output(
        #     f'du -sh "{output_file}"', shell=True).split()[0].decode('utf-8')
        # Get the size in bytes
        input_size = os.path.getsize(input_file)
        output_size = os.path.getsize(output_file)
        # Convert to MB
        input_size = f'{input_size / 1024 / 1024:.2f}M'
        output_size = f'{output_size / 1024 / 1024:.2f}M'
        print(f'Original file size: {input_size}')
        print(f'Compressed file size: {output_size}')
        input_size = float(input_size.replace('M', ''))
        output_size = float(output_size.replace('M', ''))

        print(f'{input_size} <= {output_size} = {input_size <= output_size}')
        if input_size <= output_size:
            print(
                'Compressed file is bigger than the original file. Original file not deleted')
            # Add not compressed label
            subprocess.call(f'mv "{input_file}" "{
                            input_file[::-1].split(".", 1)[1][::-1]} not compressed.mp4"', shell=True)
            # Delete output file
            subprocess.call(f'rm "{output_file}"', shell=True)
        else:
            subprocess.call(f'rm "{input_file}"', shell=True)
            # if not output_file_provided:
            #     subprocess.call(f'mv "{output_file}" "{
            #                     original_file}"', shell=True)
            print('Original file deleted')
    else:
        print('Original file not deleted')


def is_video_file(file) -> bool:
    return file.endswith('.mp4') or file.endswith('.mkv') or file.endswith('.avi') or file.endswith('.mov') or file.endswith('.wmv')


if os.path.isfile(input_source):
    compress_video(input_file, output_file)
    clone_metadata(input_file, output_file)
    compare_size(input_file, output_file)
else:
    # Get all the files in the directory
    files = os.listdir(input_source)
    videos_to_compress = 0
    videos_size_mb = 0

    for index, file in enumerate(files):
        if 'compressed' in file or '.trashed' in file:
            continue
        if is_video_file(file):
            videos_to_compress += 1
            videos_size_mb += os.path.getsize(
                f'{input_source}/{file}') / 1024 / 1024

    # Remove compressed videos and non-video files from files
    files = [file for file in files if not 'compressed' in file]
    files = [file for file in files if not '.trashed' in file]
    files = [file for file in files if is_video_file(file)]

    print(f'Videos to compress: {videos_to_compress}, Total size: {
          videos_size_mb:.2f} MB / {videos_size_mb / 1024:.2f} GB')

    list_files = input('Do you want to list all the files? (y/n): ')
    if list_files == 'y':
        for index, file in enumerate(files):
            if is_video_file(file):
                video_size = os.path.getsize(
                    f'{input_source}/{file}') / 1024 / 1024
                video_size = f'{video_size:.2f} MB'
                print(f'{index + 1}. {video_size} {file}')

    continue_compression = input('Do you want to continue? (y/n): ')
    if continue_compression != 'y':
        sys.exit()

    # Compress all the files in the directory
    for index, file in enumerate(files):
        if file.endswith('.mp4') or file.endswith('.mkv') or file.endswith('.avi') or file.endswith('.mov') or file.endswith('.wmv'):
            input_file = f'{input_source}/{file}'
            output_file = f'{
                input_source}/{file[::-1].split(".", 1)[1][::-1]} compressed.mp4'
            print(f'Compressing {file}, duration: {
                  get_video_duration(input_file)} seconds')
            compress_video(input_file, output_file)
            clone_metadata(input_file, output_file)
            compare_size(input_file, output_file)
            print(f'Compressed {index + 1}/{len(files)}')

    compressed_videos_size_mb = 0

    compressed_videos = os.listdir(input_source)
    for index, file in enumerate(compressed_videos):
        if 'compressed' in file:
            compressed_videos_size_mb += os.path.getsize(
                f'{input_source}/{file}') / 1024 / 1024

    print('All files compressed')
    print(f'Original size: {
          videos_size_mb:.2f} MB / {videos_size_mb / 1024:.2f} GB')
    print(f'Compressed size: {
          compressed_videos_size_mb:.2f} MB / {compressed_videos_size_mb / 1024:.2f} GB')
    decreased_percentage = (
        1 - (compressed_videos_size_mb / videos_size_mb)) * 100
    print(f'Decreased size: {decreased_percentage:.2f}%')
