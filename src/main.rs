
mod wallpapers;


#[tokio::main]
async fn main() {
    let  name = wall_prompt();
    // get the home directory
    let home = std::env::var("HOME").unwrap();
    let wallpaper_directory = home + "/.live_wallpapers/";
    let path = wallpaper_directory + &name;
    println!("{:?}", path);
    let fps = ask_fps();
    wallpapers::wallpaper_loop_with_path(&path, fps);
}


// prompt asks user for name of wallpaper and return it
fn wall_prompt() -> String {
    println!("Enter the name of wallpaper");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}

fn ask_fps() -> u64 {
    println!("Enter the fps");
    let mut fps = String::new();
    std::io::stdin().read_line(&mut fps).expect("Failed to read line");
    fps.trim().parse::<u64>().unwrap()
}
