# compy-cli

A command line interface to compress video files using ffmpeg.

## Usage

You can specify the input and output files as arguments:

```bash
compy-cli [options] <input> <output>
```

Or you can specify only the input file. In this case, the output file will be the same as the input file, but with the `compressed` suffix:

```bash
compy-cli [options] <input>
```

## Examples

```bash
compy-cli video.mp4 compressed.mp4
```

```bash
compy-cli video.mp4
```

## Options

| Option | Description |
| --- | --- |
| `-h, --help` | output usage information |
| `-V, --version` | output the version number |
| `-v, --verbose` | verbose output |

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.