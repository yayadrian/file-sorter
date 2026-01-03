# File Sorter - Zip Image Converter

A production-ready desktop application that processes zip files containing images, converting various image formats to JPEG and maintaining the original folder structure.

## Features

- 🖼️ **Multiple Format Support**: Converts HEIC, HEIF, WebP, TIFF, BMP, AVIF to JPEG
- 📦 **Batch Processing**: Queue multiple zip files for sequential processing
- 🎯 **Smart Handling**: Preserves PNG, JPEG, and GIF files as-is; keeps animated formats
- 📁 **Structure Preservation**: Maintains original folder hierarchy in output
- 🔄 **Collision Management**: Automatically handles filename conflicts
- 📊 **Detailed Reports**: Generates JSON report for each processed zip
- 🚫 **Fully Offline**: No network calls, all processing is local
- 🖥️ **Cross-Platform**: Works on macOS and Windows 11

## Privacy & Security

**This application is fully offline.** All file processing happens locally on your machine. No files are uploaded to any server, and no telemetry data is collected. Your images stay private and secure on your device.

## Installation

### macOS

1. Download the `.dmg` file from the releases page
2. Open the DMG and drag File Sorter to your Applications folder
3. Double-click to launch
4. If macOS prevents opening (unsigned app), go to System Preferences > Security & Privacy and click "Open Anyway"

### Windows

1. Download the `.msi` installer from the releases page
2. Run the installer
3. Follow the installation wizard
4. Launch File Sorter from the Start Menu

## Usage

1. **Add Files**: Drag and drop zip files onto the app, or click to choose files
2. **Auto-Processing**: Processing starts automatically once files are added
3. **Monitor Progress**: Watch real-time progress with file counts and current operations
4. **Cancel Anytime**: Stop the current job with the Cancel button
5. **Access Output**: Completed zips are saved to your Downloads folder with a "Show in Folder" button

### Output

For each input zip, the app creates:
- **Output zip**: Named `<original>-converted.zip` in your Downloads folder
- **report.json**: Included in the output zip root with processing details

## Supported Formats

### Input Formats

| Format | Action | Notes |
|--------|--------|-------|
| JPEG (.jpg, .jpeg) | Copy as-is | No re-encoding |
| PNG (.png) | Copy as-is | Transparency preserved |
| GIF (.gif) | Copy as-is | Animated GIFs preserved |
| HEIC/HEIF | Convert to JPEG | EXIF metadata preserved |
| WebP | Convert to JPEG or copy | Animated WebP kept as-is |
| TIFF/TIF | Convert to JPEG | EXIF metadata preserved |
| BMP | Convert to JPEG | No EXIF metadata |
| AVIF | Convert to JPEG | Partial metadata support |

### Conversion Settings

- **JPEG Quality**: 95 (high quality)
- **Transparency Handling**: Composited onto white background
- **Metadata**: EXIF preserved where possible (HEIC, TIFF, WebP)

## Building from Source

### Prerequisites

- **Node.js** (v18 or later)
- **Rust** (latest stable)
- **npm** or **yarn**

#### Platform-Specific Requirements

**macOS:**
```bash
# Install libheif via Homebrew
brew install libheif libde265 x265
```

**Windows:**
```bash
# Install vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
./bootstrap-vcpkg.sh

# Install libheif
./vcpkg install libheif
```

### Development

```bash
# Clone the repository
git clone https://github.com/yayadrian/file-sorter.git
cd file-sorter

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Building for Production

```bash
# Build for current platform
npm run tauri build

# Output locations:
# macOS: src-tauri/target/release/bundle/dmg/
# Windows: src-tauri/target/release/bundle/msi/
```

## Automated Builds & Releases (GitHub Actions)

This repo includes a GitHub Actions workflow that automatically builds platform installers and attaches them to GitHub Releases:

- Workflow file: `.github/workflows/build-and-release.yml`
- On every push to `main`: builds for macOS/Windows/Linux and updates a prerelease named `nightly`
- On version tags (`v*`, e.g. `v1.0.1`): builds for macOS/Windows/Linux and creates a release for that tag

### Creating a versioned release

1. Bump the version in `package.json` and `src-tauri/tauri.conf.json` (and `src-tauri/Cargo.toml` if you want it to match).
2. Create and push a tag:

```bash
git tag v1.0.1
git push origin v1.0.1
```

3. Wait for the workflow to finish, then download the installers from the GitHub Releases page.

### Code Signing (Optional but Recommended)

**macOS:**
```bash
# Set environment variables before building
export APPLE_CERTIFICATE=<base64-cert>
export APPLE_CERTIFICATE_PASSWORD=<password>
export APPLE_ID=<your-apple-id>
export APPLE_TEAM_ID=<team-id>
```

**Windows:**
```bash
# Use signtool or configure in tauri.conf.json
```

## Architecture

```
┌─────────────────────────────────────┐
│  Frontend (Preact + TypeScript)    │
│  - Drag & Drop UI                   │
│  - Queue Management                 │
│  - Progress Display                 │
└────────────┬────────────────────────┘
             │ Tauri IPC
┌────────────▼────────────────────────┐
│  Tauri v2 (Rust Backend)           │
│  - Zip Processing                   │
│  - Image Conversion                 │
│  - File Management                  │
│  - Report Generation                │
└─────────────────────────────────────┘
```

## Metadata Preservation Limitations

| Format | EXIF Preserved | Notes |
|--------|---------------|-------|
| HEIC/HEIF | ✅ Yes | Full EXIF via libheif |
| TIFF | ✅ Yes | Full EXIF support |
| WebP | ⚠️ Partial | XMP/EXIF if present in source |
| BMP | ❌ N/A | BMP format has no EXIF |
| AVIF | ⚠️ Partial | Depends on encoder support |

All converted images are encoded as JPEG with quality 95. Metadata preservation is attempted but may not be complete for all formats.

## Testing

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run specific test suite
cargo test collision_tests
cargo test report_tests
```

Test fixtures are located in `src-tauri/tests/fixtures/`.

## Troubleshooting

### "No image files found in zip"
- Ensure your zip contains supported image formats
- Check that images aren't nested inside another zip (nested zips are ignored)

### Conversion errors
- Some HEIC files may require iOS-specific codecs
- Very large images may cause memory issues (processing is single-threaded)

### Output not appearing
- Check your Downloads folder
- Ensure you have write permissions to Downloads
- Look for `<filename>-converted.zip` or `<filename>-converted-1.zip` if collision occurred

### macOS "App is damaged" error
- This occurs with unsigned apps
- Go to System Preferences > Security & Privacy > General
- Click "Open Anyway" for File Sorter

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## Known Limitations

- **Single-threaded**: Image conversion is sequential (future enhancement: parallel processing)
- **Memory**: Very large images (>100MB) loaded entirely into memory during conversion
- **Animated WebP detection**: Currently simplified; may not detect all animated WebP files correctly
- **EXIF preservation**: Not all metadata fields preserved for all formats

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Image processing powered by [image-rs](https://github.com/image-rs/image)
- HEIC support via [libheif-rs](https://github.com/Cykooz/libheif-rs)
- UI built with [Preact](https://preactjs.com/)

---

**Made with ❤️ for Harmony and her very specific zip file requirements**  
