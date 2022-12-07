# Bmp2chr

Cross-platform command line tool to convert an indexed-color Bitmap (.bmp) into CHR data for use in developing Nintento Entertainment System (NES) ROMs.


![](screenshot.png)
<br><br>

# Requirements

Bitmaps should use indexed color mode, be 128px wide, height should be a multiple of 8px.

Pixel values are assigned from palette_index % 4, so multiple 4-color palettes can be used in the same bitmap if you so desire, and it will all get "flattened" to 2bpp pixel data.

The palettes themselves are not imported, so it's up to you to use colors that resemble in-game colors.
<br><br>

# Usage

On the command line:
```bmp2chr mybitmap.bmp```
<br><br>

# Build

Project is written in Rust and should compile for Windows/Linux/Mac and more.

Just run ```cargo build``` to compile or download binaries from releases.