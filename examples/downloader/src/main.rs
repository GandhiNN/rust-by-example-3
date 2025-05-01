use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::{Client, get};
use std::io::{Seek, SeekFrom, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{error::Error, fs::File, time::Instant};

const DOWNLOAD_URL: &str = "https://www.stats.govt.nz/assets/Uploads/Annual-balance-sheets/Annual-balance-sheets-2023-provisional/Download-data/accumulation-accounts-2008-2023-provisional.csv";
const FILE_NAME: &str =
    "/Users/ngakangandhi/study/rust/rust-by-example-3/examples/downloader/download/file.csv";
const CHUNK_SIZE: u64 = 1024 * 8; // each chunk is 8 KB

fn linear_download() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let response = get(DOWNLOAD_URL)?;
    let content = response.bytes()?;

    let mut downloaded_file = File::create(FILE_NAME)?;
    downloaded_file.write_all(&content)?;

    let duration = now.elapsed();
    println!("Downloaded file in {duration:?}");
    Ok(())
}

fn concurrent_download() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    // We create a http client that will be used by all threads
    let client = Client::new();
    let response = client.head(DOWNLOAD_URL).send()?;
    let total_size = response
        .headers()
        .get("content-length")
        .ok_or("Content-Length header missing")?
        .to_str()?
        .parse::<u64>()?;
    println!("file size: {total_size}");

    let file = File::create(FILE_NAME)?;
    file.set_len(total_size)?;

    let file = Arc::new(Mutex::new(file));

    let mut handles = vec![];
    for start in (0..total_size).step_by(CHUNK_SIZE as usize) {
        let end = (start + CHUNK_SIZE - 1).min(total_size - 1);
        let client = client.clone();
        let file = Arc::clone(&file);
        let url = DOWNLOAD_URL.to_string();

        let handle = thread::spawn(move || {
            let response = client
                .get(&url)
                .header("Range", format!("bytes={start}-{end}"))
                .send()
                .map_err(|e| format!("Request failed: {e}"))?
                .bytes()
                .map_err(|e| format!("Failed to read bytes because {e}"))?;

            let mut file = file.lock().map_err(|e| format!("Mutex lock failed: {e}"))?;
            file.seek(SeekFrom::Start(start))
                .map_err(|e| format!("Seek failed: {e}"))?;
            file.write_all(&response)
                .map_err(|e| format!("Write failed: {e}"))?;
            Ok::<(), String>(())
        });
        handles.push(handle);
    }
    let duration = now.elapsed();
    println!("Downloads in chunks took: {duration:?}");

    Ok(())
}

fn download_with_progress_bar() -> Result<(), String> {
    let client = Client::new();
    let vlc_url = "https://get.videolan.org/vlc/3.0.21/macosx/vlc-3.0.21-intel64.dmg";
    let destination = FILE_NAME;

    let total_size = client
        .head(vlc_url)
        .send()
        .map_err(|e| format!("failed to send head request: {e}"))?
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .ok_or("Content-length header missing")?
        .to_str()
        .map_err(|e| format!("invalid content-length header: {e}"))?
        .parse::<u64>()
        .map_err(|e| format!("failed to parse content-length: {e}"))?;

    let file =
        File::create(destination).map_err(|e| format!("failed to create file because {e}"))?;
    file.set_len(total_size)
        .map_err(|e| format!("failed to set file size because: {e}"))?;
    let file = Arc::new(Mutex::new(file));

    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) {msg}")
        .unwrap()
        .progress_chars("#>-")
    );

    let (sender, receiver) = mpsc::channel::<u64>();

    let chunk_size: u64 = 1024 * 1024; // 1MB chunks
    // spawn threads for downloading chunks
    for start in (0..total_size).step_by(chunk_size as usize) {
        let end = (start + chunk_size - 1).min(total_size - 1);
        let client = client.clone();
        let file = Arc::clone(&file);
        let sender = sender.clone();
        let url = vlc_url.to_string();

        thread::spawn(move || {
            match client
                .get(&url)
                .header("Range", format!("bytes={start}-{end}"))
                .send()
                .and_then(|res| res.bytes())
            {
                Ok(response) => {
                    let mut file = file.lock().expect("failed to lock file");
                    file.seek(SeekFrom::Start(start)).expect("seek failed");
                    file.write_all(&response).expect("write failed");

                    sender
                        .send(response.len() as u64)
                        .expect("failed to send progress");
                }
                Err(e) => {
                    eprintln!("failed to download chunk: {e}");
                }
            }
        });
    }

    for downloaded in receiver {
        progress_bar.inc(downloaded);
    }

    progress_bar.finish_with_message("Download complete");

    Ok(())
}

fn main() {
    println!("LINEAR:: starting linear download");
    match linear_download() {
        Ok(()) => {
            println!("LINEAR:: finished linear download successfully");
        }
        Err(e) => {
            eprintln!("LINEAR:: failed to download {FILE_NAME} because {e}");
        }
    }
    println!("CONCURRENT:: starting concurrent download");
    match concurrent_download() {
        Ok(()) => {
            println!("CONCURRENT:: finished concurrent download successfully");
        }
        Err(e) => {
            eprintln!("CONCURRENT:: failed to download {FILE_NAME} because {e}");
        }
    }
    println!("PROGRESS_BAR:: starting dowload with progress bar");
    match download_with_progress_bar() {
        Ok(()) => {
            println!("finished download");
        }
        Err(e) => {
            println!("failed to download with progress bar: {e}");
        }
    }
}
