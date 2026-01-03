use file_sorter_lib::utils::collision::CollisionManager;
use std::path::Path;

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

#[test]
fn test_different_folders_no_collision() {
    let mut manager = CollisionManager::new();
    
    let path1 = Path::new("folder1/test.jpg");
    let path2 = Path::new("folder2/test.jpg");
    
    let result1 = manager.get_unique_path(path1);
    let result2 = manager.get_unique_path(path2);
    
    assert_eq!(result1, path1);
    assert_eq!(result2, path2);
}

#[test]
fn test_mixed_extensions() {
    let mut manager = CollisionManager::new();
    
    // test.heic -> test.jpg
    let heic_path = Path::new("test.heic");
    let jpg_output = Path::new("test.jpg");
    
    // Reserve the converted path
    let first = manager.get_unique_path(jpg_output);
    assert_eq!(first, Path::new("test.jpg"));
    
    // Now if we try to convert another test.heic or test.webp to jpg
    let second = manager.get_unique_path(jpg_output);
    assert_eq!(second, Path::new("test-1.jpg"));
}

#[test]
fn test_no_extension() {
    let mut manager = CollisionManager::new();
    let path = Path::new("noext");
    
    let first = manager.get_unique_path(path);
    assert_eq!(first, Path::new("noext"));

    let second = manager.get_unique_path(path);
    assert_eq!(second, Path::new("noext-1"));
}
