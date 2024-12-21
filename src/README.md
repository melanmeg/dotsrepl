# rust Hint by me.

- Rustでファイルとディレクトリを扱うためのサンプル.
  - シンボリックリンクの作成/解決
  - ファイル/ディレクトリ作成
  - 読み込み/削除

```rust
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // ファイルの作成と書き込み
    let file_path = Path::new("example_file.txt");
    fs::write(file_path, "Hello, Rust!")?;

    // ディレクトリの作成
    let dir_path = Path::new("example_dir");
    fs::create_dir(dir_path)?;

    // シンボリックリンクの作成
    let symlink_path = Path::new("example_symlink");
    fs::symlink(file_path, symlink_path)?;

    // シンボリックリンクの解決
    if let Ok(metadata) = fs::symlink_metadata(symlink_path) {
        if metadata.file_type().is_symlink() {
            let target = fs::read_link(symlink_path)?;
            println!("Symlink points to {:?}", target);
        }
    }

    // ファイルの読み込み
    let content = fs::read_to_string(file_path)?;
    println!("File content: {}", content);

    // ディレクトリ内のファイルをリスト表示
    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        println!("Entry: {:?}", entry.path());
    }

    // シンボリックリンク、ファイル、ディレクトリの削除
    fs::remove_file(file_path)?;
    fs::remove_dir(dir_path)?;
    fs::remove_file(symlink_path)?;

    Ok(())
}
```