#![cfg(feature = "CoreAudioTypes")]

use icrate::CoreAudioTypes::*;

#[test]
fn generated_four_char_code_is_correct() {
    // 'lpcm' == 1819304813
    assert_eq!(kAudioFormatLinearPCM, 1819304813);
}
