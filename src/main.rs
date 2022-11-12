
mod wallpapers;
mod youtube;
mod vid_mess;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // Take input from the user for the wall_prompt function and store name in name
    let  name = wall_prompt();
    // let path = "/Volumes/ExternalDisk/programming/gotomate/vault/";
    // make path from the string name
    let path = "/Volumes/ExternalDisk/programming/gotomate/vault/".to_string() + &name;
    // use the path to start 60 fps loop

    // print the path
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