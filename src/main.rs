use std::{
    env::{self, args},
    fs::read_dir,
    path::{Path, PathBuf},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    let target_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    list_dir(&target_path)?;

    Ok(())
}

fn list_dir(path: &Path) -> std::io::Result<()> {
    let entries = read_dir(path)?;

    let mut item_list = Vec::new();

    for entry in entries {
        let entry = entry?;
        item_list.push(entry);
    }

    item_list.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

        b_is_dir
            .cmp(&a_is_dir)
            .then_with(|| a.file_name().cmp(&b.file_name()))
    });

    for entry in item_list {
        let metadata = entry.metadata()?;
        let file_name = entry.file_name();
        let filename_str = file_name.to_string_lossy();

        let file_type = if metadata.is_dir() { "DIR" } else { "FILE" };

        println!("{:<30} {:<10}", filename_str, file_type);
    }
    Ok(())
}
