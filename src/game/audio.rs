use std::error::Error;
use std::io::Cursor;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, StreamError};

pub const UI_SELECT_EFFECT: &[u8] = include_bytes!("../../assets/audio/ui_select.ogg");
pub const UI_ERROR_EFFECT: &[u8] = include_bytes!("../../assets/audio/ui_error.ogg");
pub const UI_DIALOG_OPEN_EFFECT: &[u8] = include_bytes!("../../assets/audio/ui_dialog_open.ogg");

pub const BOOK_OPEN_EFFECT: &[u8] = include_bytes!("../../assets/audio/book_open.ogg");
pub const BOOK_FLIP_EFFECT: &[u8] = include_bytes!("../../assets/audio/book_flip.ogg");

pub const UNDO_REDO_EFFECT: &[u8] = include_bytes!("../../assets/audio/undo_redo.ogg");

pub const SECRET_FOUND_EFFECT: &[u8] = include_bytes!("../../assets/audio/secret_found.ogg");
pub const NO_PATH_EFFECT: &[u8] = include_bytes!("../../assets/audio/no_path.ogg");
pub const LEVEL_COMPLETE_EFFECT: &[u8] = include_bytes!("../../assets/audio/level_complete.ogg");
pub const LEVEL_PACK_COMPLETE_EFFECT: &[u8] = include_bytes!("../../assets/audio/level_pack_complete.ogg");
pub const LEVEL_RESET: &[u8] = include_bytes!("../../assets/audio/level_reset.ogg");
pub const STEP_EFFECT: &[u8] = include_bytes!("../../assets/audio/step.ogg");

pub struct AudioHandler {
    _stream: OutputStream,

    stream_handle: OutputStreamHandle,
}

impl AudioHandler {
    pub fn new() -> Result<Self, StreamError> {
        let output_stream = OutputStream::try_default();
        let (_stream, stream_handle) = output_stream?;

        Ok(Self {
            _stream,

            stream_handle,
        })
    }

    pub fn play_sound_effect(&self, sound_effect: &'static [u8]) -> Result<(), Box<dyn Error>> {
        let cursor = Cursor::new(sound_effect);
        let source = Decoder::new(cursor)?;

        self.stream_handle.play_raw(source.convert_samples())?;

        Ok(())
    }
}
