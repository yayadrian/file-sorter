# Test Fixtures

This directory contains minimal test zip files for integration testing.

## Files

- `basic.zip` - Contains 2 JPEGs and 1 PNG for basic passthrough testing
- `nested_folders.zip` - Contains images in nested folder structure (a/b/c/)
- `collision.zip` - Contains files that would collide after conversion (img.jpg + img.heic)
- `nested_zip.zip` - Contains a nested zip file to test that it's properly ignored

## Image Files

The test images are minimal placeholder files:
- `test1.jpg`, `test2.jpg` - Minimal JPEG markers
- `test1.png` - Minimal PNG marker

These are suitable for testing file handling logic without requiring large image files in the repository.

## Usage

Run tests with:
```bash
cd src-tauri
cargo test
```

The test fixtures are automatically used by the integration tests.
