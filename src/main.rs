use anyhow::Result;
use crunchyroll_rs::{Crunchyroll, Media, Episode, Season};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Can't Post my Login Credentials bc of Degens on the Internet
    let credentials_file = fs::read_to_string("/home/user/.lib/credentials")?;
    let credentials : Vec<&str> = credentials_file.rsplit(",").collect();

    let crunchyroll = Crunchyroll::builder()
        .login_with_credentials(credentials[1], credentials[0])
        .await?;

    // Get Seasons
    // let a = crunchyroll.media_from_id("GEXH3WKK0").await?;
    // let b = a.seasons().await?;
    // for c in b {println!("{:?} {:?}", c.id, c.title);}

    // "G6NQ5DWZ6" My Hero Academia
    get_info(crunchyroll.clone(), "GRVNC2JD0", "My Hero").await?;

    // "G4PH0WEKE" BLUELOCK
    get_info(crunchyroll.clone(), "G6GGCV0QX", "").await?;

    // "GRMG8ZQZR" One Piece
    get_info(crunchyroll.clone(), "GYP8PM4KY", "").await?;

    // "GEXH3WKK0" Vinland Sage
    get_info(crunchyroll.clone(), "G6K5CN5QV", "Vinland Saga").await?;

    // "GQWH0MP94" Reborn to Master the Blade: From Hero-King to Extraordinary Squire
    get_info(crunchyroll.clone(), "GR09CX5QG", "Reborn Sword").await?;

    // "GW4HM75NP" Ice Guy and his Cool Female Colleague
    get_info(crunchyroll.clone(), "GY9PC21VE", "Ice Guy").await?;

    // "G0XHWM5Q4" Tomo-chan is a Girl!
    get_info(crunchyroll.clone(), "G619CPWZZ", "Tomo is a Girl").await?;

    // "GG5H5X3DE" Buddy Daddies
    get_info(crunchyroll.clone(), "G6DQCGP0X", "").await?;

    Ok(())
}

async fn get_info(cr: Crunchyroll, season_id: &str, alt_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let season: Media<Season> = cr.media_from_id(season_id).await?;
    let episodes = season.episodes().await?;
    let episode: &Media<Episode> = &episodes[episodes.len()-1];

    let now = std::time::SystemTime::now();
    let sec = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() * 1000;
    let time = sec - (episode.metadata.premium_available_date.timestamp_millis() as u64);

    let mut info_string: String = "".to_string();

    info_string.push_str(&format!("{:0>2} ",time / 86_400_000));

    if alt_title == "" {
        info_string.push_str(&format!("{: <15}",episode.metadata.series_title));
    } else {
        info_string.push_str(&format!("{: <15}",alt_title));
    }
    info_string.push_str("  ");

    info_string.push_str(&format!("S{:0>2}",episode.metadata.season_number));
    info_string.push_str(&format!("E{:0>4}",episode.metadata.episode_number));
    info_string.push_str("  ");

    info_string.push_str(&format!("{:?}",episode.title));

    match time / 86_400_000 {
        0 => print!("\x1b[38;5;2m"),    // Green
        1 => print!("\x1b[38;5;2m"),    // Green
        2 => print!("\x1b[38;5;3m"),    // Yellow
        3 => print!("\x1b[38;5;3m"),    // Yellow
        4 => print!("\x1b[38;5;15m"),   // White
        5 => print!("\x1b[38;5;15m"),   // White
        6 => print!("\x1b[38;5;15m"),   // White
        7 => print!("\x1b[38;5;14m"),   // Cyan
        8 => print!("\x1b[38;5;240m"),  // Grey
        9 => print!("\x1b[38;5;240m"),  // Grey
        10=> print!("\x1b[38;5;240m"),  // Grey
        11=> print!("\x1b[38;5;240m"),  // Grey
        12=> print!("\x1b[38;5;240m"),  // Grey
        13=> print!("\x1b[38;5;240m"),  // Grey
        _ => print!("\x1b[38;5;160m"),  // Red
    }

    println!("{}", info_string);

    print!("\x1b[0m");

    Ok(())
}