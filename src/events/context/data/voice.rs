use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
/// Audio data from all users in a voice channel, fired every 20ms.
///
/// Valid audio data (`Some(audio)` where `audio.len >= 0`) contains up to 20ms of 16-bit stereo PCM audio
/// at 48kHz, using native endianness. Songbird will not send audio for silent regions, these should
/// be inferred using [`SpeakingUpdate`]s (and filled in by the user if required using arrays of zeroes).
///
/// [`Config::playout_buffer_length`]: crate::Config::playout_buffer_length
pub struct VoiceTick {
    /// Decoded voice data and source packets sent by each user.
    pub speaking: HashMap<u32, VoiceData>,

    /// Set of all SSRCs currently known in the call who aren't included in [`Self::speaking`].
    pub silent: HashSet<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
/// Voice packet and audio data for a single user, from a single tick.
pub struct VoiceData {
    /// RTP packet clocked out for this tick.
    ///
    /// If `None`, then the packet was lost, and [`Self::decoded_voice`] may include
    /// around one codec delay's worth of audio.
    pub packet: Option<RtpData>,
    /// PCM audio obtained from a user.
    ///
    /// Valid audio data (`Some(audio)` where `audio.len >= 0`) typically contains 20ms of 16-bit stereo PCM audio
    /// at 48kHz, using native endianness. Channels are interleaved (i.e., `L, R, L, R, ...`).
    ///
    /// This value will be `None` if Songbird is not configured to decode audio.
    pub decoded_voice: Option<Vec<i16>>,
}
