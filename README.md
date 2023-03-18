# My Crunchyroll Watchdog

### Warning
If you are attempting to run this code yourself..
    - You might have to change the seasons to watch the animes you want
    - Credentials will not work for anyone else running this, change login_with_credentials("Crunchyroll Email", "Crunchyroll Password")

### How to Find Your Show
    1.) Navigate to your show on Crunchyroll
    2.) Look at the URL
    3.) Find the 9-digit show identifier
    4.) Add this to the code

    // Get Seasons
    // let a = crunchyroll.media_from_id("Your Show's 9-digit id").await?;
    // let b = a.seasons().await?;
    // for c in b {println!("{:?} {:?}", c.id, c.title);}

    5.) Run the code
    6.) Find the 9-digit Season Id you want to watch
    7.) Make a watchdog entry in the code
    
