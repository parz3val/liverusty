use rustube;


pub async fn _sync_download(url: &str) {
    println!("Downloadeding the video :: {:?}", url);
    let video = rustube::download_best_quality(url).await.unwrap();
    println!("Video downloaded :: {:?}", video);
}

