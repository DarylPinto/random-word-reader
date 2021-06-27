use crate::utils::{self, ChannelMessage};
use rand::seq::SliceRandom;
use std::{error::Error, path::PathBuf, sync::mpsc};
use tts::Tts;

// Read a random word from the selected category forever
// until a signal is received from channel_receiver to stop
pub fn speak(
    categories: Vec<PathBuf>,
    selected_category: Option<PathBuf>,
    interval: u64,
    channel_receiver: mpsc::Receiver<ChannelMessage>,
) -> Result<(), Box<dyn Error>> {
    let mut speaker = Tts::default()?;
    let delay = if interval < 1 { 1 } else { interval };

    let word_pool = utils::get_words(categories, selected_category)?;

    loop {
        let choice = word_pool.choose(&mut rand::thread_rng());
        if let Some(word) = choice {
            speaker.speak(word, true)?;
        }
        if channel_receiver.try_recv().is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(delay));
    }

    Ok(())
}
