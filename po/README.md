# Dawn UI translations

Dawn v1 ships English. All static screen text uses Slint's `@tr` syntax.
The runtime initializes gettext at `/usr/share/locale`; Slint uses the Cargo
package name `frontend` as its domain (`frontend.mo`). This language is
independent of the locale selected for the installed system.

To refresh the template, install the extractor matching Slint:

```sh
cargo install slint-tr-extractor --version 1.18.1 --locked
slint-tr-extractor --default-domain frontend --package-name Dawn --package-version 0.1.0 \
  -o po/frontend.pot frontend/ui/*.slint frontend/ui/screens/*.slint
```

Future translations can be compiled with `msgfmt`, then installed as
`/usr/share/locale/<language>/LC_MESSAGES/frontend.mo`. Branding text and
slide captions belong to the branding folder and are loaded verbatim.
