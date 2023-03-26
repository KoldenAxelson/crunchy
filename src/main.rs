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
    // let a = crunchyroll.media_from_id("GKEH2G428").await?;
    // let b = a.seasons().await?;
    // for c in b {println!("{:?} {:?}", c.id, c.title);}

    let mut shows: Vec<Vec<String>> = Vec::new();

    // "G4PH0WEKE" BLUELOCK
    shows.push(get_info(crunchyroll.clone(), "G6GGCV0QX", "").await?);

    // "GRMG8ZQZR" One Piece
    shows.push(get_info(crunchyroll.clone(), "GYP8PM4KY", "").await?);

    // "GEXH3WKK0" Vinland Sage
    shows.push(get_info(crunchyroll.clone(), "G6K5CN5QV", "Vinland Saga").await?);

    // "GQWH0MP94" Reborn to Master the Blade: From Hero-King to Extraordinary Squire
    shows.push(get_info(crunchyroll.clone(), "GR09CX5QG", "Reborn Sword").await?);

    // "GW4HM75NP" Ice Guy and his Cool Female Colleague
    shows.push(get_info(crunchyroll.clone(), "GY9PC21VE", "Ice Guy").await?);

    // "G0XHWM5Q4" Tomo-chan is a Girl!
    shows.push(get_info(crunchyroll.clone(), "G619CPWZZ", "Tomo is a Girl").await?);

    // "GG5H5X3DE" Buddy Daddies
    shows.push(get_info(crunchyroll.clone(), "G6DQCGP0X", "").await?);

    // "GR09CX545" BOFURI
    shows.push(get_info(crunchyroll.clone(), "GR09CX545", "BOFURI").await?);

    shows.sort_by_key(|show| show[0].parse::<u64>().unwrap());

    for show in shows {
        println!("{}{}{}",show[2],show[1],"\x1b[0m");
    }

    Ok(())
}

async fn get_info(cr: Crunchyroll, season_id: &str, alt_title: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let season: Media<Season> = cr.media_from_id(season_id).await?;
    let episodes = season.episodes().await?;
    let episode: &Media<Episode> = &episodes[episodes.len()-1];

    let now = std::time::SystemTime::now();
    let sec = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() * 1000;
    let time = sec - (episode.metadata.premium_available_date.timestamp_millis() as u64);

    let mut info_string: String = "".to_string();

    // Time Until (Estimate)
    let remaining_days  =  (604_800_000 - time) / 86_400_000;
    let remaining_hours = ((604_800_000 - time) % 86_400_000) / 3_600_000;
    let remaining_mins  = ((604_800_000 - time) %  3_600_000) /    60_000;
    if remaining_days <= 7 {
        info_string.push_str(
            &format!("-{:0>2}:{:0>2}:{:0>2} ",
                remaining_days,
                remaining_hours,
                remaining_mins));
    } else {
        let time_over     = time - 604_800_000;
        let elapsed_days  = time_over / 86_400_000;
        let elapsed_hours = time_over % 86_400_000 / 3_600_000;
        let elapsed_mins  = time_over %  3_600_000 /    60_000;
        info_string.push_str(
            &format!("+{:0>2}:{:0>2}:{:0>2} ",
                elapsed_days,
                elapsed_hours,
                elapsed_mins));
    }

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

    let color: String = match time / 86_400_000 {
        0 => "\x1b[38;5;2m".to_string(),    // Green
        1 => "\x1b[38;5;2m".to_string(),    // Green
        2 => "\x1b[38;5;3m".to_string(),    // Yellow
        3 => "\x1b[38;5;3m".to_string(),    // Yellow
        4 => "\x1b[38;5;15m".to_string(),   // White
        5 => "\x1b[38;5;15m".to_string(),   // White
        6 => "\x1b[38;5;15m".to_string(),   // White
        7 => "\x1b[38;5;14m".to_string(),   // Cyan
        8 => "\x1b[38;5;240m".to_string(),  // Grey
        9 => "\x1b[38;5;240m".to_string(),  // Grey
        10=> "\x1b[38;5;240m".to_string(),  // Grey
        11=> "\x1b[38;5;240m".to_string(),  // Grey
        12=> "\x1b[38;5;240m".to_string(),  // Grey
        13=> "\x1b[38;5;240m".to_string(),  // Grey
        _ => "\x1b[38;5;160m".to_string(),  // Red
    };

    let time_string: String = time.to_string();

    let return_vec: Vec<String> = vec![time_string, info_string, color];

    Ok(return_vec)
}
