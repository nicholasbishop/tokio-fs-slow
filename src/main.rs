use chrono::Utc;
use std::path::Path;

const ITERATIONS: usize = 100_000;

fn test_sync(path: &Path) {
    let start = Utc::now();
    for _ in 0..ITERATIONS {
        let perms = std::fs::metadata(path).unwrap().permissions();
        std::fs::set_permissions(path, perms).unwrap();
    }
    let end = Utc::now();

    println!("sync duration: {}", end - start);
}

async fn test_async(path: &Path) {
    let start = Utc::now();
    for _ in 0..ITERATIONS {
        let perms = tokio::fs::metadata(path).await.unwrap().permissions();
        tokio::fs::set_permissions(path, perms).await.unwrap();
    }
    let end = Utc::now();

    println!("async duration: {}", end - start);
}

#[tokio::main]
async fn main() {
    let path = Path::new("temp_path");
    std::fs::write(path, "contents").unwrap();

    test_sync(path);
    test_async(path).await;
}
