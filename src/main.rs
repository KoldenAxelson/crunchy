use anyhow::Result;
use crunchyroll_rs::{Crunchyroll, Episode, Season};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Can't Post my Login Credentials bc of Degens on the Internet
    let read_to_string = fs::read_to_string("/Library/.my_credentials")?;
    let credentials_file = read_to_string;
    let credentials : Vec<&str> = credentials_file.rsplit(",").collect();

    let crunchyroll = Crunchyroll::builder()
        .login_with_credentials(credentials[1], credentials[0])
        .await?;

    // let token = Crunchyroll::session_token(&crunchyroll).await;
    // println!("{:?}", token);

    // Get Seasons
    // let a: Series = crunchyroll.media_from_id("G79H23G0D").await?;
    // let b = a.seasons().await?;
    // for mut c in b {
    //     println!("{:?} Season {:?} | {:?}",c.title,c.season_number,c.id);
    //     let d = c.versions().await?;
    //     for e in d {println!("{:?} {:?}", e.id, e.title);}
    // }

    // Episode Details from Season
    // let a: Season = crunchyroll.media_from_id("GY2PCVE0G").await?;
    // let b = a.episodes().await?;
    // for c in b {
    //     println!("{:?} {:?} {:?}", c.id, c.title, c.audio_locale);
    // }

    // Waiting for new Dr. Stone Seasons
    // let check_series = crunchyroll.media_from_id("GYEXQKJG6").await?;
    // let check_seasons = check_series.seasons().await?;
    // let check_exclude = vec![
    //     // Season ID's
    // ];
    // for check_season in check_seasons {
    //     if !check_exclude.contains(&check_season.id.as_ref()) {
    //         println!("\x1b[38;5;13m{:?} {:?}\x1b[0m", check_season.id, check_season.title);
    //     }
    // }

    let mut shows: Vec<Vec<String>> = Vec::new();

    // "GRMG8ZQZR" One Piece
    shows.push(get_info(crunchyroll.clone(), "GYP8PM4KY", "", false).await?);

    // "G24H1N3MP" Mushoku Tensei: Jobless Reincarnation
    shows.push(get_info(crunchyroll.clone(), "G6NQCJ9P1", "Mushoku Tensei", true).await?);

    // "GYZJ43JMR" That Time I Got Reincarnated as a Slime
    shows.push(get_info(crunchyroll.clone(), "GRZXCMZ37", "As a Slime", true).await?);

    // "GP5HJ84QX" A Condition Called Love 
    shows.push(get_info(crunchyroll.clone(), "GRDQCG50Z", "Condition: Love", true).await?);

    // "G6NQ5DWZ6" My Hero Academia
    shows.push(get_info(crunchyroll.clone(), "G6DQCG592", "My Hero", true).await?);

    // "GYE5K3GQR" Konosuba
    shows.push(get_info(crunchyroll.clone(), "GY2PCVE0G", "Konosuba", true).await?);

    // "GJ0H7QGME" Viral Hit
    shows.push(get_info(crunchyroll.clone(), "G68VCPZNJ", "Viral Hit", true).await?);

    // "G3KHEVDPE" Wind Breaker 
    shows.push(get_info(crunchyroll.clone(), "G6X0C458K", "Windbreaker", true).await?);

    // "GG5H5XQ7D" Kaiju No. 8 
    shows.push(get_info(crunchyroll.clone(), "GY5VCDQKW", "Kaiju #8", false).await?);

    // Hiatus
    // "GEXH3WKK0" Vinland Sage
    // "GDKHZEP8W" MASHLE: MAGIC AND MUSCLES
    // "GVDHX8Q71" Why Raeliana Ended Up at the Duke's Mansion
    // "GNVHKNPQ7" My Love Story with Yamada-kun at Lv999
    // "GYEXQKJG6" DR STONE
    // "GY5P48XEY" Demon Slayer: Kimetsu no Yaiba
    // "G0XHWM0D3" Trapped in a Dating Sim! 
    // "GVDHX8504" Reborn as a Vending Machine
    // "GEXH3W2Z7" Reign of the Seven Spellblades
    // "G9VHN9P43" Horimiya: Missing Piecies
    // "G5PHNM7E2" Sugar Apple Fairy Tale 
    // "G0XHWM52V" Sacrificial Princess and the King of Beasts
    // "GEXH3W2V7" Sign of Affection
    // "GDKHZEJ0K" Solo Leveling
    // "G0XHWM1EK" Wrong Way to Use Healing Magic
    // "G3KHEVDJ7" Apothecary Diaries
    // "G79H23Z8P" Shangri-La Frontier
    // "GG5H5XQX4" Frieren: Beyond Journey's End

    shows.sort_by_key(|show| show[0].parse::<u64>().unwrap());

    for show in shows {
        let show_check: bool = show[3].parse().unwrap();
        if !show_check {continue;}
        println!("{}{}{}",show[2],show[1],"\x1b[0m");
    }

    Ok(())
}

async fn get_info(cr: Crunchyroll, season_id: &str, alt_title: &str, dub: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let season: Season = cr.media_from_id(season_id).await?;
    let episodes = season.episodes().await?;
    let mut episode: &Episode = &episodes[episodes.len()-1];
    if dub {
        for n in 0..episodes.len()-1 {
            if format!("{:?}",episodes[n].audio_locale).eq("en_US") || episodes[n].is_dubbed {
                episode = &episodes[n];
            }
        }
    }

    let now       = std::time::SystemTime::now();
    let sec: i64  = (now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() * 1000) as i64;
    let mut time: i64 = sec - (episode.premium_available_date.timestamp_millis() as i64);
    if dub && !format!("{:?}",episode.audio_locale).eq("en_US") {time -= 604_800_000 * 2;}

    let mut info_string: String = "".to_string();

    // Time Until (Estimate)
    let remaining_days  =  (604_800_000 - time) / 86_400_000;
    let remaining_hours = ((604_800_000 - time) % 86_400_000) / 3_600_000;
    let remaining_mins  = ((604_800_000 - time) %  3_600_000) /    60_000;
    if remaining_days <= 7 && (604_800_000 - time) >= 0 {
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

    info_string.push_str(&format!("{} ",episode.premium_available_date.format("%a")));

    if alt_title == "" {
        info_string.push_str(&format!("{: <15}",episode.series_title));
    } else {
        info_string.push_str(&format!("{: <15}",alt_title));
    }
    info_string.push_str("  ");

    info_string.push_str(&format!("S{:0>2}",episode.season_number));
    info_string.push_str(&format!("E{:0>4}",episode.sequence_number));
    info_string.push_str("  ");

    info_string.push_str(&format!("{}", match episode.is_dubbed {true => "EN", false => "JP"}));
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

    let dub_matched = (dub && episode.is_dubbed) || !dub;
    let is_expired = remaining_days < -30;
    let will_show = format!("{:?}",dub_matched && !is_expired);

    let return_vec: Vec<String> = vec![time_string, info_string, color, will_show];

    Ok(return_vec)
}