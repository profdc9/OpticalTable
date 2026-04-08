# Optical Table

**[Launch the app →](https://profdc9.github.io/OpticalTable/)**

Optical Table is a ray optics simulator ported from a Java applet originally written by Daniel L. Marks in 1996–2004. It runs entirely in the browser with no dependencies.

## Features

- Interactive 2D optical bench with drag-and-drop optics placement
- Ray tracing through lenses, mirrors, prisms, diffraction gratings, and ABCD elements
- Gaussian beam propagation and wavefront analysis
- Optical path length (OPL), group velocity dispersion (GVD), and Seidel aberration panels
- Wavefront pupil map with Strehl ratio estimate
- Built-in scenes: telescope, Snell's law, diffraction grating, curved mirror, TIR, birefringent crystal, Ti:sapphire laser cavity
- Save and load scenes in a plain-text format compatible with the original Java applet
- Native desktop app via [Tauri](https://tauri.app/) (Windows) with native file dialogs

## Running

Open `public/optical_table_22.html` directly in a browser, or visit the hosted version at:

**https://profdc9.github.io/OpticalTable/**

To serve locally (required for URL scene loading):

```
npx serve public
```

## Desktop App (Windows)

Requires [Rust](https://rustup.rs/) and the Tauri CLI:

```
npm install
cargo tauri build
```

The installer is produced at `src-tauri/target/release/bundle/`.

## License

Copyright © 1997–2004, 2026 Daniel L. Marks

This software is provided under the [zlib license](https://opensource.org/licenses/Zlib). See the About box in the application for the full license text.
