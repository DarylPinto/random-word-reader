use crate::utils::{get_words, ChannelMessage};
use rand::seq::SliceRandom;
use std::{error::Error, path::PathBuf, sync::mpsc::Receiver};
use tts::Tts;

// Read a random word from the selected category forever
// until a signal is recieved from channel_receiver to stop
pub fn speak(
    categories: Vec<PathBuf>,
    selected_category: Option<PathBuf>,
    interval: u64,
    channel_receiver: Receiver<ChannelMessage>,
) -> Result<(), Box<dyn Error>> {
    let mut speaker = Tts::default()?;
    let delay = if interval < 1 { 1 } else { interval };

    let word_pool = get_words(categories, selected_category)?;

    loop {
        let choice = word_pool.choose(&mut rand::thread_rng());
        if let Some(word) = choice {
            speaker.speak(word, true)?;
        }
        if let Ok(message) = channel_receiver.try_recv() {
            match message {
                ChannelMessage::StopTalking => break,
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(delay));
    }

    Ok(())
}
