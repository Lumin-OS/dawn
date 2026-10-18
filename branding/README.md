# Runtime branding

Dawn reads `/etc/dawn/installer.toml` to choose a folder under
`/usr/share/dawn/branding/`. Set `DAWN_BRANDING_DIR` to use a different
folder during development; launching again loads it without recompiling.

Each folder contains `branding.toml` (see `luminos/`), `logo.svg`,
`icon.svg`, and optional `welcome.md`. The accent colors the Welcome and
Installing headings. Images are loaded at runtime, never embedded.

Optional slides are PNG files in `slides/`, in filename order. Use
zero-padded names such as `01.png`, `02.png`. An adjacent `01.txt` supplies
a plain-text caption. Slides advance every eight seconds on Installing;
opening the log or the stop confirmation pauses rotation. Without slides,
Dawn shows the usual progress and log. Invalid slide images report a
branding error rather than silently producing blank slides.

The two shipped slides are generated geometric illustrations with English
captions; a distributor can replace both images and text.
