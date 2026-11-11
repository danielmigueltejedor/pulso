# Pulso app icon

**Figure:** Eadweard Muybridge. He studied motion frame by frame, which is what a motion graphics tool does.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Portrait of Eadweard Muybridge, pioneer of early action photography.png" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Portrait_of_Eadweard_Muybridge,_pioneer_of_early_action_photography.png) (public domain). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

**Linux:** the hicolor icons (`hicolor/scalable` and every PNG size) put the tile on the Nodalix
app grid, like the Colloid icons around it: a 224 px tile inside a 256 px canvas (16 px margin),
a 51.5 px corner radius and a soft shadow. The scalable SVG is made by `tools/nodalia-app-icon.py`
in [nodalix-icon-theme](https://github.com/danielmigueltejedor/nodalix-icon-theme) and
`packaging/icons.sh` renders the PNG sizes from it. macOS, the Windows `.ico` and the
1024 px render keep the full-bleed tile. The window and taskbar icon on Windows is the 256 px
hicolor PNG, so it has the Linux margin and shadow.

## Files

| File | What |
|---|---|
| `effectcraft.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `effectcraft-small.svg` | lighter 512 px version; also the web favicon where there is one |
| `effectcraft-1024.png` | 1024 px render |
| `effectcraft-macos-512.png` | runtime Dock icon on macOS, on Apple's 824/1024 grid |
| `effectcraft.icns` | macOS icon (Apple grid) |
| `effectcraft.ico` | Windows icon, 16–256 px; embedded in `effectcraft.exe` by `apps/effectcraft/build.rs` |
| `hicolor/<n>x<n>/apps/ai.storyteller.effectcraft.png` | Linux icon theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.effectcraft.svg` | Linux scalable icon |

App ID: `ai.storyteller.effectcraft` (Wayland app ID, `packaging/linux/ai.storyteller.effectcraft.desktop`).

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `effectcraft.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
