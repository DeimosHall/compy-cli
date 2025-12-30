# compy-cli

## 1. Overview

`compy-cli` is a command-line interface tool to compress videos using an opinionated FFmpeg command template. It can compress a single video or a bunch of them if a directory is passed through arguments.

## 2. User Interface (CLI)

The tool utilizes a standard POSIX-compliant flag structure.

| Argument/Flag   | Short | Long     | Required | Description                                                         |
| --------------- | ----- | -------- | -------- | ------------------------------------------------------------------- |
| `INPUT`         | N/A   | N/A      | Yes      | Path to a single video or a directory.                              |
| Delete original | -d    | --delete | No       | If set, delete original file only after successful compression.     |
| Dry Run / List  | -l    | --list   | No       | Lists the files to be processed and exits without compressing.      |
 
## 2.2 Usage examples

Compress a single video.

```bash
compy video.mp4
```

Compress all the videos within a directory, delete original files if success.

```bash
compy my_videos/ --delete
```

List all the videos that can be processed by the tool within a directory. Don't compress any video.

```bash
compy my_videos/ --list
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.