use crate::handlers::cmd_handler::run_cmd;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct Context {
    home_dir: PathBuf,
    entries: fs::ReadDir,
}

pub fn link_to_homedir(path: String, force_flag: bool, backup_flag: bool, link_flag: bool) {
    println!("install dotfiles to homedir...\n");
    let context = context_info(path.clone()).unwrap();
    let home_dir: PathBuf = context.home_dir;

    delete_files_if_exist(backup_flag);
    create_dotbackup(backup_flag);
    backing_up(path.clone(), backup_flag, &home_dir);
    wrap_setup_dotfiles(path.clone(), force_flag, backup_flag, link_flag, &home_dir);
}

pub fn context_info(path: String) -> Result<Context, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let entries = fs::read_dir(&path).map_err(|e| format!("Failed to read directory: {}", e))?;

    Ok(Context { home_dir, entries })
}

pub fn delete_files_if_exist(backup_flag: bool) {
    // Delete files if they already exist at the backup destination
    let backup_path = format!("{}/.dotbackup", std::env::var("HOME").unwrap_or_default());
    if backup_flag && Path::new(&backup_path).exists() {
        println!(
            "Backup file already exists at {}. Removing it...\n",
            backup_path
        );
        fs::remove_dir_all(&backup_path).expect("Failed to remove existing backup file");
    }
}

pub fn create_dotbackup(backup_flag: bool) {
    // Create dotbackup
    if backup_flag {
        println!("backup dotfiles...");
        let backup_path = format!("{}/.dotbackup", std::env::var("HOME").unwrap_or_default());
        if !std::path::Path::new(&backup_path).exists() {
            println!("{} not found. Auto Make it.\n", backup_path);
            if let Err(err) = fs::create_dir(&backup_path) {
                eprintln!("Failed to create directory: {}", err);
            }
        } else {
            println!("{} already exists.\n", backup_path);
        }
    }
}

pub fn is_symlink(symlink_path: &Path) -> bool {
    match fs::symlink_metadata(symlink_path) {
        Ok(metadata) => metadata.file_type().is_symlink(),
        Err(_) => false,
    }
}

pub fn setup_dotfiles(
    force_flag: bool,
    link_flag: bool,
    home_dir: PathBuf,
    file_name: &str,
    file_path: PathBuf,
    dest_path: PathBuf,
    result_is_symlink: bool,
) {
    // Setup dotfiles
    if (!dest_path.exists() || force_flag) && !result_is_symlink {
        if link_flag {
            println!("link to homedir {}...", dest_path.display());
            let cmd = format!("ln -vsnf {} {}", file_name, home_dir.to_string_lossy());
            run_cmd(&cmd, false, None);
        } else {
            println!("copy to homedir {}...", dest_path.display());
            let cmd = format!(
                "cp -vaf {} {}",
                file_path.to_string_lossy(),
                home_dir.to_string_lossy()
            );
            run_cmd(&cmd, false, None);
        }
    } else {
        println!(" already exist {}...", dest_path.display());
    }
}

pub fn backing_up(path: String, backup_flag: bool, home_dir: &PathBuf) {
    // Backup files if backup_flag is true
    let context = context_info(path).unwrap();
    for entry in context.entries {
        match entry {
            Ok(entry) => {
                let file_name: String = entry.file_name().to_string_lossy().to_string();
                let dest_path: &PathBuf = &home_dir.as_path().join(&file_name);
                let symlink_path: &Path = Path::new(&dest_path);
                let result_is_symlink: bool = is_symlink(symlink_path);

                // Skip .git, .github, other than dotfiles
                let file_name = entry.file_name().to_string_lossy().into_owned();
                if file_name == ".git" || file_name == ".github" || !file_name.starts_with('.') {
                    continue;
                }

                // Backup files if backup_flag is true
                if backup_flag && (dest_path.exists() || result_is_symlink) {
                    println!("Backing up {}...", dest_path.display());
                    let backup_path = home_dir.join(".dotbackup").join(file_name);
                    fs::rename(&dest_path, &backup_path).expect("Failed to move file to backup");
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
            }
        }
    }
}

pub fn wrap_setup_dotfiles(
    path: String,
    force_flag: bool,
    backup_flag: bool,
    link_flag: bool,
    home_dir: &PathBuf,
) {
    let context = context_info(path).unwrap();

    if link_flag && force_flag {
        println!("ln command cannot be forced to overwrite.",);
    }

    for entry in context.entries {
        if let Ok(entry) = entry {
            let file_name: String = entry.file_name().to_string_lossy().to_string();
            let file_path: PathBuf = entry.path();
            let dest_path: &PathBuf = &home_dir.as_path().join(&file_name);
            let symlink_path: &Path = Path::new(&dest_path);
            let result_is_symlink: bool = is_symlink(symlink_path);

            // Skip .git, .github, other than dotfiles
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name == ".git"
                || file_name == ".github"
                || file_name == ".gitignore"
                || !file_name.starts_with('.')
            {
                continue;
            }

            // Backup files if backup_flag is true
            if backup_flag && (dest_path.exists() || result_is_symlink) {
                println!("Backing up {}...", dest_path.display());
                let backup_path = home_dir.join(".dotbackup").join(file_name);
                fs::rename(&dest_path, &backup_path).expect("Failed to move file to backup");
            }

            let file_name = entry.file_name().to_string_lossy().to_string();
            if !link_flag || !force_flag {
                setup_dotfiles(
                    force_flag,
                    link_flag,
                    home_dir.clone(),
                    file_name.as_str(),
                    file_path.clone(),
                    dest_path.clone(),
                    result_is_symlink,
                );
            }
        }
    }
}
