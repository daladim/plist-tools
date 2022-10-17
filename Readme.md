# plist-convert

This crate works around a bug in Traktor 3.6.1 (and at lest iTunes 12.8 on Windows)

### How to use

1. Download [`itunes-library-convert.exe` using this link](https://github.com/daladim/plist-tools/releases/latest/download/itunes-library-convert.exe)
2. in a command prompt, run
`itunes-library-convert.exe "C:\Users\my_username\Music\iTunes\iTunes Music Library.xml" "C:\Users\my_username\Music\iTunes\iTunes Music Library for Traktor.xml"`<br/>Of course, you should edit the paths to point at your actual iTunes XML library file.
3. Note: by default, only the errors are shown. If there is no output, this means this has worked. In case you want more output, run `set RUST_LOG=info` before step 2.
4. in Traktor settings, in `File management` section, choose your newly created XML file in the `iTunes/Music` field (this must be done once only)
5. start Traktor and enjoy!

Step 2 must be done everytime you change your iTunes Library.<br/>
Step 4 needs to be done only once.

This may not work for you guys outside of Europe (using a different encoding than Latin-15). If so, leave an issue and I'll add support for more encodings.

![command prompt](/img/cmd.png)


### Problem

See [this thread](https://community.native-instruments.com/discussion/5526/bug-in-tp3-3-6-0-325-duplicate-entries-in-collection/p4) for more info.<br/>
Traktor loads `iTunes Library.xml` to locate songs from iTunes.
This file is a `plist` file, where paths are encoded in UTF-8 (and percent-escaped, fwiw).

For some reason, at least on my computer (where Windows is in French), Traktor would decode paths as Latin-15 (ISO_8859_15).
Obviously, Traktor would fail at locating these files.


### Solution

This project reads the plist XML, decodes strings from UTF-8 and re-encodes them in Latin-15 before saving to a new XML file.
It is not possible to do so for every track (e.g. tracks that contain Greek, Cyrillic, etc. characters that do not fit into Latin-15). These tracks are left as-is.
