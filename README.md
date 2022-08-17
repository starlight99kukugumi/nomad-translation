# About /!\

Translation file for [Nomad Sculpt](https://nomadsculpt.com/).

Translation is in progress, and will be done by a translation agency.
Do not waste time doing it.
The repository will probably stay public, mostly for per-language 3d technical terms.

# Testing

For iOS and Android, the file should be named `debug.strings`.
For the web version, it simply needs to end with `.strings`.

- iOS: copy the file in `Nomad/debug.strings` and restart the app
- Android: copy the file in `Android/data/com.stephaneginier.nomad/files/debug.strings` and restart the app
- For the [Web version](https://stephaneginier.com/archive/nomad_demo/), simply drag & drop the file in the page

# Documentation

Should be straightforward.
Empty strings `""` will use english as a fallback.

# Note
Emojis not supported.

# Traditional Chinese

```
opencc -i locales/simplified-chinese.strings -o locales/traditional-chinese.strings -c s2twp
opencc -i description/simplified-chinese.txt -o description/traditional-chinese.txt -c s2twp
```

# Integration

```
crowdin download && ./../../build/nomad a
crowdin upload
crowdin upload translations --auto-approve-imported --import-eq-suggestions
```
