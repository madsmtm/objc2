use crate::MIDIProtocolID;

#[allow(non_upper_case_globals)]
impl MIDIProtocolID {
    /// MIDI 1.0.
    #[doc(alias = "kMIDIProtocol_1_0")]
    pub const Protocol_1_0: Self = Self(1);
    /// MIDI 2.0.
    #[doc(alias = "kMIDIProtocol_2_0")]
    pub const Protocol_2_0: Self = Self(2);
}
