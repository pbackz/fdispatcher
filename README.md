# fdispatcher

<img src="fd-logo.png" alt="wianalytics logo" width="200" height="200" style="display: block; margin-left: auto; margin-right: auto;">

- [fdispatcher](#fdispatcher)
   * [Demo](#demo)
   * [Disclaimer ⚠️](#disclaimer-)
   * [Installation](#installation)
      + [Get binary](#get-binary)
         - [x86_64](#x86_64)
      + [Build from source](#build-from-source)
      + [Archlinux AUR](#archlinux-aur)
         - [yaourt / yay / paru](#yaourt-yay-paru)
   * [Usage](#usage)
   * [Help](#help)
   * [List of supported extensions](#list-of-supported-extensions)

## Demo


## Disclaimer ⚠️

The use of this tool is at your own risk. Being as fast as possible, if the wrong extension is requested at the root (/), it could cause problems restoring files to their original location. Therefore, use it with caution.

This is why this tool provides the use of a subcommand for each supported extension rather than going through a global command that would have a '--extension' parameter.

A --dry-run option will be implemented soon.

## Installation

### Get binary

#### x86_64

```bash
wget https://github.com/pbackz/fdispatcher/releases/download/v${version}/fd_x86_64
chmod +x fd_x86_64 && mv fd_x86_64 /usr/local/bin/fd
```

### Build from source

(Need `cargo` dependency)

```bash
git clone https://github.com/pbackz/fdispatcher.git
cd fdispatcher
cargo build --release
mv target/release/fd /usr/local/bin/
```

### Archlinux AUR

#### yaourt / yay / paru

```bash
yay fdispatcher

2 aur/fdispatcher-bin 0.3.0-2 (+0 0.00)
An easy-to-use file dispatcher based on extension

1 aur/fdispatcher-git 0.3.0-1 (+0 0.00) (Installé)
An easy-to-use file dispatcher based on extension
==> Paquets à installer (ex. 1 2 3, 1-3 or ^4)
```

## Usage

```bash
fd wav -s ~/tmp/ -t .

Moved "/home/solidsnaakke/tmp/ondeanda.wav" to "./ondeanda.wav"
Moved "/home/solidsnaakke/tmp/lofi001.wav" to "./lofi001.wav"
Moved "/home/solidsnaakke/tmp/musicgen_out2.wav" to "./musicgen_out2.wav"
Moved "/home/solidsnaakke/tmp/musicgen_out3.wav" to "./musicgen_out3.wav"
Moved "/home/solidsnaakke/tmp/reggae_az_sample.wav" to "./reggae_az_sample.wav"
Moved "/home/solidsnaakke/tmp/reggae002.wav" to "./reggae002.wav"
```

```bash
fd mp4 -s ~/tmp/ -t media-mp4/

Moved "/home/solidsnaakke/tmp/20240201092606.mp4" to "media-mp4/20240201092606.mp4"
Moved "/home/solidsnaakke/tmp/20240201092749.mp4" to "media-mp4/20240201092749.mp4"
Moved "/home/solidsnaakke/tmp/20240201182450_FILM_x2.mp4" to "media-mp4/20240201182450_FILM_x2.mp4"
Moved "/home/solidsnaakke/tmp/20240201182450_FILM_x2.mp4.temp.mp4" to "media-mp4/20240201182450_FILM_x2.mp4.temp.mp4"
Moved "/home/solidsnaakke/tmp/20240201185618.mp4" to "media-mp4/20240201185618.mp4"
Moved "/home/solidsnaakke/tmp/20240201185618_FILM_x2.mp4" to "media-mp4/20240201185618_FILM_x2.mp4"
Moved "/home/solidsnaakke/tmp/20240201210746_FILM_x2.mp4" to "media-mp4/20240201210746_FILM_x2.mp4"
Moved "/home/solidsnaakke/tmp/20240201214928.mp4" to "media-mp4/20240201214928.mp4"
Moved "/home/solidsnaakke/tmp/20240201214928_FILM_x2.mp4" to "media-mp4/20240201214928_FILM_x2.mp4"
Moved "/home/solidsnaakke/tmp/20240201224842.mp4" to "media-mp4/20240201224842.mp4"
Moved "/home/solidsnaakke/tmp/20240201224842_FILM_x2.mp4" to "media-mp4/20240201224842_FILM_x2.mp4"
Moved "/home/solidsnaakke/tmp/20240202005846.mp4" to "media-mp4/20240202005846.mp4"
Moved "/home/solidsnaakke/tmp/20240202005846_FILM_x2.mp4" to "media-mp4/20240202005846_FILM_x2.mp4"
```

```bash
ll ~/tmp/*.png | wc -l
14324


time fd png -t ~/tmp/ -s media/

...
...
Moved "/home/solidsnaakke/tmp/20240202005846_000000497.png" to "media/20240202005846_000000497.png"
Moved "/home/solidsnaakke/tmp/20240202005846_000000498.png" to "media/20240202005846_000000498.png"
Moved "/home/solidsnaakke/tmp/20240202005846_000000499.png" to "media/20240202005846_000000499.png"

________________________________________________________
Executed in  472.22 millis    fish           external
usr time   24.89 millis    0.00 micros   24.89 millis
sys time  442.69 millis  706.00 micros  441.98 millis
```

## Help

```bash
fd help mp4
Perform .mp4 file extension move

Usage: fd mp4 --source-dir <source-dir> --target-dir <target-dir>

Options:
-s, --source-dir <source-dir>  The source directory where files will be processed recursively
-t, --target-dir <target-dir>  The target directory where files will be moved
-h, --help                     Print help
```

```bash
fd help
A File dispatcher based on extension

Usage: fd <COMMAND>

Commands:
mp4         Perform .mp4 file extension move
jpg         Perform .jpg file extension move
png         Perform .png file extension move
mkv         Perform .mkv file extension move
wav         Perform .wav file extension move
pdf         Perform .pdf file extension move
...
xml         Perform .xml file extension move
xsl         Perform .xsl file extension move
xpm         Perform .xpm file extension move
zip         Perform .zip file extension move
help        Print this message or the help of the given subcommand(s)

Options:
-h, --help  Print help
```

## List of supported extensions

| Extension  | Supported |
|------------|:---------:|
| mp4        | [✅]       |
| jpg        | [✅]       |
| png        | [✅]       |
| mkv        | [✅]       |
| wav        | [✅]       |
| pdf        | [✅]       |
| asm        | [✅]       |
| blend      | [✅]       |
| bmp        | [✅]       |
| bz2        | [✅]       |
| c          | [✅]       |
| class      | [✅]       |
| conf       | [✅]       |
| cfg        | [✅]       |
| cpp        | [✅]       |
| cc         | [✅]       |
| css        | [✅]       |
| csv        | [✅]       |
| db         | [✅]       |
| deb        | [✅]       |
| desktop    | [✅]       |
| diff       | [✅]       |
| dtd        | [✅]       |
| gif        | [✅]       |
| gz         | [✅]       |
| h          | [✅]       |
| html       | [✅]       |
| htm        | [✅]       |
| jar        | [✅]       |
| java       | [✅]       |
| kwd        | [✅]       |
| ksp        | [✅]       |
| kss        | [✅]       |
| log        | [✅]       |
| m3u        | [✅]       |
| m4a        | [✅]       |
| m4p        | [✅]       |
| md5        | [✅]       |
| md5sums    | [✅]       |
| mo         | [✅]       |
| mpg        | [✅]       |
| mpeg       | [✅]       |
| ogg        | [✅]       |
| patch      | [✅]       |
| php        | [✅]       |
| phps       | [✅]       |
| phtml      | [✅]       |
| pl         | [✅]       |
| pls        | [✅]       |
| pov        | [✅]       |
| properties | [✅]       |
| ps         | [✅]       |
| py         | [✅]       |
| pyo        | [✅]       |
| pyc        | [✅]       |
| rdf        | [✅]       |
| rs         | [✅]       |
| go         | [✅]       |
| rpm        | [✅]       |
| rtf        | [✅]       |
| sh         | [✅]       |
| so         | [✅]       |
| tar        | [✅]       |
| tgz        | [✅]       |
| ttf        | [✅]       |
| txt        | [✅]       |
| xbel       | [✅]       |
| xsd        | [✅]       |
| xml        | [✅]       |
| xsl        | [✅]       |
| xpm        | [✅]       |
| zip        | [✅]       |
| help       | [✅]       |
