use std::path::Path;
use vid2img::FileSource;
use image::{save_buffer};


// This function takes
// video file path string , and folder name
// and saves the frames to the folder
pub fn _vid_2_frames(vid_path: &str, folder_name: &str) {
    let _path = Path::new(vid_path);
    let frame_source = FileSource::new(_path, (200, 200)).unwrap();
    // start counter for name
    for frame in frame_source.into_iter() {
        if let Ok(Some(png_img_data)) = frame {
            // save image into fodler with folder_name/counter.png
            let file_name = String::from(folder_name);
            save_buffer(file_name, &png_img_data,800, 600, image::ColorType::Rgb8).unwrap();
        }
    }
}

