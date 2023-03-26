# My Rust Crunchyroll Watchdog

![crunchy](https://user-images.githubusercontent.com/128202226/227758163-953d1acc-ac94-4ce9-890b-72288b94b694.png)

### How to Install
    1.) Clone the Repository
    2.) Navigate to the Repository
    3.) Edit main() {...} so that credentials[1] is your Crunchyroll Email
    4.) Edit main() {...} so that credentials[0] is your Crunchyroll Password
    5.) Edit main() {...} to remove credentials_file & credentials
    6.) Save main.rs
    7.) cargo run --release

### Ease of Use
    1.) Complete Installation
    2.) Create a directory $HOME/.rust_binaries
    3.) Copy crunchy/target/release/crunchy into $HOME/.rust_binaries
    4.) Append your $PATH to point to $HOME/.rust_binaries
    5.) Restart your terminal
    6.) In terminal: crunchy

### Auto-Refresh
    1.) Complete Ease of Use
    2.) Update your bash profile to include

        function anime_watch () {
            clear
            crunchy
            sleep 1800
            anime_watch
        }

    3.) Save your bash profile
    4.) Restart Terminal
    5.) In terminal: anime_watch
    NOTE: Ctrl+C to Terminate the Process

### FAQ
    Q: What do the colors mean?
    A: How fresh the release is: Green 1-2 days, Yellow 3-4 days, White 5-6 days, Cyan 7 days, Grey 8-13 days, Red 14 days.

    Q: What does the +/- mean by the time?
    A: - = Countdown, + = Time Over Estimate

    Q: I would like something to be different, how do I leave you a suggestion?
    A: I believe in the power of prayer, and doing it yourself.
        
