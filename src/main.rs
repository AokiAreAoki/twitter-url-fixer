extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use clipboard_master::{CallbackResult, Master, ClipboardHandler};
use regex::{Captures, Regex};
use std::time::Duration;
use std::{io, thread::sleep, time};

struct Handler;

#[macro_use]
extern crate lazy_static;

const REPLACE_DELAY: Duration = time::Duration::from_millis(150);

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        lazy_static! {
            static ref TWITTER_RE: Regex = match Regex::new(
                r"(?P<p>https?://)(?:\w+\.)?(?:(?:vxtwitter|twitter|x)\.com)(?P<u>/[\w+@_-]+/status/\w+)(\?\S+)?"
            ) {
                Ok(re) => re,
                Err(err) => panic!("{}", err),
            };
            static ref DISCORD_RE: Regex = match Regex::new(
                r"(?P<link>https?://(?:\w+\.)?discordapp\.(?:com|net)/attachments/[/\w._-]+)(\?\S+)?"
            ) {
                Ok(re) => re,
                Err(err) => panic!("{}", err),
            };
        }

        sleep(REPLACE_DELAY);

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

        match ctx.get_contents() {
            Err(_) => {
                return CallbackResult::Stop;
            }
            Ok(string) => {
                let mut fixed_string = string.clone();

                fixed_string = TWITTER_RE
                    .replace_all(fixed_string.as_str(), |cap: &Captures| {
                        let protocol = cap.get(1).unwrap().as_str();
                        let uri = cap.get(2).unwrap().as_str();

                        format!("{}vxtwitter.com{}", protocol, uri)
                    })
                    .into_owned();

                // fixed_string = DISCORD_RE
                //     .replace_all(fixed_string.as_str(), |cap: &Captures| {
                //         cap.get(1).unwrap().as_str().to_owned()
                //     })
                //     .into_owned();

                if string == fixed_string {
                    return CallbackResult::Stop;
                }

                if let Err(err) = ctx.set_contents(fixed_string.clone()) {
                    println!("Error setting clipboard: {}", err);
                    return CallbackResult::Stop;
                }

                println!("Replace:");
                println!("- {}", string);
                println!("+ {}", fixed_string);

                CallbackResult::Next
            }
        }
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn error_callback(error: io::Error) -> CallbackResult {
    println!("Error listening to clipboard: {}", error);
    CallbackResult::Stop
}

fn main() {
    let delay = time::Duration::from_millis(100);

    println!("Listening to clipboard");
    loop {
        let _ = Master::new(Handler).run();
        sleep(delay);
    }
}
