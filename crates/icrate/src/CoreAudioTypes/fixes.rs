use crate::common::*;
use crate::CoreAudioTypes::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        #[deprecated]
        AVAudioSessionErrorInsufficientPriority =
            AVAudioSessionErrorCodeInsufficientPriority as c_uint,
    }
);
