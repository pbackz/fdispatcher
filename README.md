# fdispatcher

## Help

```bash
./fdispatcher help                                                                                               (base)
A File dispatcher based on extension

Usage: fdispatcher <COMMAND>

Commands:
mp4   Perform .mp4 file extension move
jpg   Perform .jpg file extension move
png   Perform .png file extension move
mkv   Perform .mkv file extension move
wav   Perform .wav file extension move
pdf   Perform .pdf file extension move
help  Print this message or the help of the given subcommand(s)

Options:
-h, --help  Print help
```

```bash
./fdispatcher help mp4                                                                                           (base)
Perform .mp4 file extension move

Usage: fdispatcher mp4 --source-dir <source-dir> --target-dir <target-dir>

Options:
-s, --source-dir <source-dir>  The source directory where files will be processed recursively
-t, --target-dir <target-dir>  The target directory where files will be moved
-h, --help                     Print help
```

## Usage

```bash
./fdispatcher wav -s ~/tmp/ -t .
Moved "/home/solidsnaakke/tmp/ondeanda.wav" to "./ondeanda.wav"
Moved "/home/solidsnaakke/tmp/lofi001.wav" to "./lofi001.wav"
Moved "/home/solidsnaakke/tmp/musicgen_out2.wav" to "./musicgen_out2.wav"
Moved "/home/solidsnaakke/tmp/musicgen_out3.wav" to "./musicgen_out3.wav"
Moved "/home/solidsnaakke/tmp/reggae_az_sample.wav" to "./reggae_az_sample.wav"
Moved "/home/solidsnaakke/tmp/reggae002.wav" to "./reggae002.wav"
```

```bash
./fdispatcher mp4 -s ~/tmp/ -t .
Moved "/home/solidsnaakke/tmp/travel_trip.mp4" to "./travel_trip.mp4"
Moved "/home/solidsnaakke/tmp/20240201052005.mp4" to "./20240201052005.mp4"
```

```bash
ll fdispatcher/outputs/test/*.png | wc -l
6558

time fdispatcher png -s fdispatcher/outputs/test/ -t ~/test/ 
...
Moved "../article-generator/fdispatcher/outputs/test/20240201044043_000000717.png" to "/home/solidsnaakke/tmp/20240201044043_000000717.png"
Moved "../article-generator/fdispatcher/outputs/test/20240201044043_000000718.png" to "/home/solidsnaakke/tmp/20240201044043_000000718.png"
Moved "../article-generator/fdispatcher/outputs/test/20240201044043_000000719.png" to "/home/solidsnaakke/tmp/20240201044043_000000719.png"

________________________________________________________
Executed in  248.52 millis    fish           external
usr time   19.91 millis    0.00 micros   19.91 millis
sys time  226.75 millis  732.00 micros  226.02 millis
```