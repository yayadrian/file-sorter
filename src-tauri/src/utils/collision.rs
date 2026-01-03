use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Manages filename collisions by tracking used paths and adding numeric suffixes
pub struct CollisionManager {
    used_paths: HashMap<String, usize>,
}

impl CollisionManager {
    pub fn new() -> Self {
        Self {
            used_paths: HashMap::new(),
        }
    }

    /// Get a unique output path, adding numeric suffix if needed
    /// Example: IMG_1.jpg -> IMG_1.jpg, IMG_1-1.jpg, IMG_1-2.jpg, etc.
    pub fn get_unique_path(&mut self, desired_path: &Path) -> PathBuf {
        let path_str = desired_path.to_string_lossy().to_string();
        
        // Check if this exact path is already used
        if !self.used_paths.contains_key(&path_str) {
            self.used_paths.insert(path_str, 1);
            return desired_path.to_path_buf();
        }

        // Path is used, find next available suffix
        let stem = desired_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        
        let extension = desired_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let parent = desired_path.parent().unwrap_or(Path::new(""));

        // Try suffixes until we find an unused path
        for i in 1..10000 {
            let new_name = if extension.is_empty() {
                format!("{}-{}", stem, i)
            } else {
                format!("{}-{}.{}", stem, i, extension)
            };

            let new_path = parent.join(&new_name);
            let new_path_str = new_path.to_string_lossy().to_string();

            if !self.used_paths.contains_key(&new_path_str) {
                self.used_paths.insert(new_path_str, 1);
                return new_path;
            }
        }

        // Fallback (should never reach here)
        desired_path.to_path_buf()
    }

    /// Reserve a path without returning it (for files we're copying as-is)
    pub fn reserve_path(&mut self, path: &Path) {
        let path_str = path.to_string_lossy().to_string();
        self.used_paths.insert(path_str, 1);
    }
}

impl Default for CollisionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_collision() {
        let mut manager = CollisionManager::new();
        let path = Path::new("test.jpg");
        let result = manager.get_unique_path(path);
        assert_eq!(result, path);
    }

    #[test]
    fn test_collision_adds_suffix() {
        let mut manager = CollisionManager::new();
        let path = Path::new("test.jpg");
        
        let first = manager.get_unique_path(path);
        assert_eq!(first, Path::new("test.jpg"));

        let second = manager.get_unique_path(path);
        assert_eq!(second, Path::new("test-1.jpg"));

        let third = manager.get_unique_path(path);
        assert_eq!(third, Path::new("test-2.jpg"));
    }

    #[test]
    fn test_collision_with_folders() {
        let mut manager = CollisionManager::new();
        let path = Path::new("folder/subfolder/test.jpg");
        
        let first = manager.get_unique_path(path);
        assert_eq!(first, Path::new("folder/subfolder/test.jpg"));

        let second = manager.get_unique_path(path);
        assert_eq!(second, Path::new("folder/subfolder/test-1.jpg"));
    }
}
