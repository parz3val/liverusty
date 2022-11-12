use wallpaper;

// Wallpaper utilities

// Wallpaper loop with path and fps
// files are 001,002, and so on
// path/to/folder/001.jpg
pub fn wallpaper_loop_with_path(path: &str, fps: u64) {

    println!("Changing wallpaper from {:?} with fps {:?}", path, fps);
    let mut i = 1;
    loop {
        let file_name = format!("{:03}", i);
        let file_path = format!("{}/{}.jpg", path, file_name);
        // copy file path into a new file_path_name
        let file_path_name = file_path.clone();
        if let Ok(file) = std::fs::File::open(file_path) {
            // print file in console
            println!("{:?}", file);
            change_wallpaper_from_path(&file_path_name);
        }
        i += 1;
        std::thread::sleep(std::time::Duration::from_millis(1000 / fps));
    }
}

fn change_wallpaper_from_path(path: &str) {
    wallpaper::set_from_path(path).unwrap();
}