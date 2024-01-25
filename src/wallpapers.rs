use wallpaper;

// Wallpaper utilities

// Wallpaper loop with path and fps
// files are 001,002, and so on
// path/to/folder/001.jpg
pub fn wallpaper_loop_with_path(path: &str, fps: u64) {

    println!("Changing wallpaper from {:?} with fps {:?}", path, fps);
    let mut i = 1;
    let frame_count = std::fs::read_dir(path).unwrap().count();

    loop {
        let file_name = format!("{:03}", i);
        let file_path = format!("{}/{}.png", path, file_name);
        // copy file path into a new file_path_name
        let file_path_name = file_path.clone();
        if let Ok(file) = std::fs::File::open(file_path) {
            // print file in console
            //change_wallpaper_from_path(&file_path_name);
            change_wallpaper_from_file(&file_path_name);
        }

        i += 1;
        if i > frame_count - 1 {
            i = 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000 / fps));
    }
}

fn _change_wallpaper_from_path(path: &str) {
    wallpaper::set_from_path(path).unwrap();
}

fn change_wallpaper_from_file(path: &str) {
    // create a hsetroot -fill path command
    std::process::Command::new("hsetroot")
        .arg("-fill")
        .arg(path)
        .output()
        .expect("failed to execute process");
}
