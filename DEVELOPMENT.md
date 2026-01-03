# Development Notes

## Project Structure

This is a Tauri v2 desktop application with:
- **Frontend**: Preact + TypeScript + Vite
- **Backend**: Rust with image processing capabilities

## Key Implementation Details

### Image Processing Pipeline
1. Zip files are processed sequentially from a queue
2. Each image is evaluated: copy as-is (JPEG/PNG/GIF) or convert to JPEG
3. Folder structure is preserved in the output zip
4. Filename collisions are handled with numeric suffixes (-1, -2, etc.)

### State Management
- `ProcessorState` manages the job queue and cancellation flags
- Jobs are processed one at a time to avoid resource contention
- Progress events are emitted via Tauri's event system

### Error Handling
- Fail-fast approach: any error aborts the current job
- Temp files are cleaned up automatically via RAII (Drop trait)
- No partial output zips are created on failure

### Future Enhancements
- Parallel image conversion (with configurable thread pool)
- Better animated WebP detection (parse WebP headers)
- Full EXIF preservation using exiftool integration
- Drag reordering of queue items in UI
- Pause/resume functionality

## Development Commands

```bash
# Start dev server
./dev.sh
# or
npm run tauri dev

# Run tests
cd src-tauri && cargo test

# Build for production
npm run tauri build

# Generate icons (requires source icon)
npm run tauri icon path/to/icon.png
```

## Dependencies Note

### libheif on macOS
```bash
brew install libheif libde265 x265
```

### libheif on Windows
Use vcpkg as documented in README.md

## Code Architecture

```
src-tauri/src/
├── main.rs              - Entry point
├── lib.rs               - Module exports
├── commands.rs          - Tauri command handlers
├── report.rs            - Report JSON generation
├── processor/
│   ├── mod.rs           - State management
│   ├── queue.rs         - Queue processor
│   ├── zip_handler.rs   - Zip read/write
│   ├── image_converter.rs - Image format conversion
│   └── temp_manager.rs  - Temp file lifecycle
└── utils/
    ├── collision.rs     - Filename collision handling
    └── metadata.rs      - EXIF utilities
```

## Testing Strategy

- **Unit tests**: Collision logic, report generation
- **Integration tests**: Temp management, file structure
- **Test fixtures**: Minimal zip files for CI/CD

Test fixtures are kept small (<1KB) to avoid bloating the repository.
