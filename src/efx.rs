/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use al::*;

pub const ALC_EFX_MAJOR_VERSION: ::std::os::raw::c_uint = 131073;
pub const ALC_EFX_MINOR_VERSION: ::std::os::raw::c_uint = 131074;
pub const ALC_MAX_AUXILIARY_SENDS: ::std::os::raw::c_uint = 131075;
pub const AL_METERS_PER_UNIT: ::std::os::raw::c_uint = 131076;
pub const AL_DIRECT_FILTER: ::std::os::raw::c_uint = 131077;
pub const AL_AUXILIARY_SEND_FILTER: ::std::os::raw::c_uint = 131078;
pub const AL_AIR_ABSORPTION_FACTOR: ::std::os::raw::c_uint = 131079;
pub const AL_ROOM_ROLLOFF_FACTOR: ::std::os::raw::c_uint = 131080;
pub const AL_CONE_OUTER_GAINHF: ::std::os::raw::c_uint = 131081;
pub const AL_DIRECT_FILTER_GAINHF_AUTO: ::std::os::raw::c_uint = 131082;
pub const AL_AUXILIARY_SEND_FILTER_GAIN_AUTO: ::std::os::raw::c_uint = 131083;
pub const AL_AUXILIARY_SEND_FILTER_GAINHF_AUTO: ::std::os::raw::c_uint =
    131084;
pub const AL_REVERB_DENSITY: ::std::os::raw::c_int = 1;
pub const AL_REVERB_DIFFUSION: ::std::os::raw::c_int = 2;
pub const AL_REVERB_GAIN: ::std::os::raw::c_int = 3;
pub const AL_REVERB_GAINHF: ::std::os::raw::c_int = 4;
pub const AL_REVERB_DECAY_TIME: ::std::os::raw::c_int = 5;
pub const AL_REVERB_DECAY_HFRATIO: ::std::os::raw::c_int = 6;
pub const AL_REVERB_REFLECTIONS_GAIN: ::std::os::raw::c_int = 7;
pub const AL_REVERB_REFLECTIONS_DELAY: ::std::os::raw::c_int = 8;
pub const AL_REVERB_LATE_REVERB_GAIN: ::std::os::raw::c_int = 9;
pub const AL_REVERB_LATE_REVERB_DELAY: ::std::os::raw::c_int = 10;
pub const AL_REVERB_AIR_ABSORPTION_GAINHF: ::std::os::raw::c_int = 11;
pub const AL_REVERB_ROOM_ROLLOFF_FACTOR: ::std::os::raw::c_int = 12;
pub const AL_REVERB_DECAY_HFLIMIT: ::std::os::raw::c_int = 13;
pub const AL_EAXREVERB_DENSITY: ::std::os::raw::c_int = 1;
pub const AL_EAXREVERB_DIFFUSION: ::std::os::raw::c_int = 2;
pub const AL_EAXREVERB_GAIN: ::std::os::raw::c_int = 3;
pub const AL_EAXREVERB_GAINHF: ::std::os::raw::c_int = 4;
pub const AL_EAXREVERB_GAINLF: ::std::os::raw::c_int = 5;
pub const AL_EAXREVERB_DECAY_TIME: ::std::os::raw::c_int = 6;
pub const AL_EAXREVERB_DECAY_HFRATIO: ::std::os::raw::c_int = 7;
pub const AL_EAXREVERB_DECAY_LFRATIO: ::std::os::raw::c_int = 8;
pub const AL_EAXREVERB_REFLECTIONS_GAIN: ::std::os::raw::c_int = 9;
pub const AL_EAXREVERB_REFLECTIONS_DELAY: ::std::os::raw::c_int = 10;
pub const AL_EAXREVERB_REFLECTIONS_PAN: ::std::os::raw::c_int = 11;
pub const AL_EAXREVERB_LATE_REVERB_GAIN: ::std::os::raw::c_int = 12;
pub const AL_EAXREVERB_LATE_REVERB_DELAY: ::std::os::raw::c_int = 13;
pub const AL_EAXREVERB_LATE_REVERB_PAN: ::std::os::raw::c_int = 14;
pub const AL_EAXREVERB_ECHO_TIME: ::std::os::raw::c_int = 15;
pub const AL_EAXREVERB_ECHO_DEPTH: ::std::os::raw::c_int = 16;
pub const AL_EAXREVERB_MODULATION_TIME: ::std::os::raw::c_int = 17;
pub const AL_EAXREVERB_MODULATION_DEPTH: ::std::os::raw::c_int = 18;
pub const AL_EAXREVERB_AIR_ABSORPTION_GAINHF: ::std::os::raw::c_int = 19;
pub const AL_EAXREVERB_HFREFERENCE: ::std::os::raw::c_int = 20;
pub const AL_EAXREVERB_LFREFERENCE: ::std::os::raw::c_int = 21;
pub const AL_EAXREVERB_ROOM_ROLLOFF_FACTOR: ::std::os::raw::c_int = 22;
pub const AL_EAXREVERB_DECAY_HFLIMIT: ::std::os::raw::c_int = 23;
pub const AL_CHORUS_WAVEFORM: ::std::os::raw::c_int = 1;
pub const AL_CHORUS_PHASE: ::std::os::raw::c_int = 2;
pub const AL_CHORUS_RATE: ::std::os::raw::c_int = 3;
pub const AL_CHORUS_DEPTH: ::std::os::raw::c_int = 4;
pub const AL_CHORUS_FEEDBACK: ::std::os::raw::c_int = 5;
pub const AL_CHORUS_DELAY: ::std::os::raw::c_int = 6;
pub const AL_DISTORTION_EDGE: ::std::os::raw::c_int = 1;
pub const AL_DISTORTION_GAIN: ::std::os::raw::c_int = 2;
pub const AL_DISTORTION_LOWPASS_CUTOFF: ::std::os::raw::c_int = 3;
pub const AL_DISTORTION_EQCENTER: ::std::os::raw::c_int = 4;
pub const AL_DISTORTION_EQBANDWIDTH: ::std::os::raw::c_int = 5;
pub const AL_ECHO_DELAY: ::std::os::raw::c_int = 1;
pub const AL_ECHO_LRDELAY: ::std::os::raw::c_int = 2;
pub const AL_ECHO_DAMPING: ::std::os::raw::c_int = 3;
pub const AL_ECHO_FEEDBACK: ::std::os::raw::c_int = 4;
pub const AL_ECHO_SPREAD: ::std::os::raw::c_int = 5;
pub const AL_FLANGER_WAVEFORM: ::std::os::raw::c_int = 1;
pub const AL_FLANGER_PHASE: ::std::os::raw::c_int = 2;
pub const AL_FLANGER_RATE: ::std::os::raw::c_int = 3;
pub const AL_FLANGER_DEPTH: ::std::os::raw::c_int = 4;
pub const AL_FLANGER_FEEDBACK: ::std::os::raw::c_int = 5;
pub const AL_FLANGER_DELAY: ::std::os::raw::c_int = 6;
pub const AL_FREQUENCY_SHIFTER_FREQUENCY: ::std::os::raw::c_int = 1;
pub const AL_FREQUENCY_SHIFTER_LEFT_DIRECTION: ::std::os::raw::c_int = 2;
pub const AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION: ::std::os::raw::c_int = 3;
pub const AL_VOCAL_MORPHER_PHONEMEA: ::std::os::raw::c_int = 1;
pub const AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING: ::std::os::raw::c_int = 2;
pub const AL_VOCAL_MORPHER_PHONEMEB: ::std::os::raw::c_int = 3;
pub const AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING: ::std::os::raw::c_int = 4;
pub const AL_VOCAL_MORPHER_WAVEFORM: ::std::os::raw::c_int = 5;
pub const AL_VOCAL_MORPHER_RATE: ::std::os::raw::c_int = 6;
pub const AL_PITCH_SHIFTER_COARSE_TUNE: ::std::os::raw::c_int = 1;
pub const AL_PITCH_SHIFTER_FINE_TUNE: ::std::os::raw::c_int = 2;
pub const AL_RING_MODULATOR_FREQUENCY: ::std::os::raw::c_int = 1;
pub const AL_RING_MODULATOR_HIGHPASS_CUTOFF: ::std::os::raw::c_int = 2;
pub const AL_RING_MODULATOR_WAVEFORM: ::std::os::raw::c_int = 3;
pub const AL_AUTOWAH_ATTACK_TIME: ::std::os::raw::c_int = 1;
pub const AL_AUTOWAH_RELEASE_TIME: ::std::os::raw::c_int = 2;
pub const AL_AUTOWAH_RESONANCE: ::std::os::raw::c_int = 3;
pub const AL_AUTOWAH_PEAK_GAIN: ::std::os::raw::c_int = 4;
pub const AL_COMPRESSOR_ONOFF: ::std::os::raw::c_int = 1;
pub const AL_EQUALIZER_LOW_GAIN: ::std::os::raw::c_int = 1;
pub const AL_EQUALIZER_LOW_CUTOFF: ::std::os::raw::c_int = 2;
pub const AL_EQUALIZER_MID1_GAIN: ::std::os::raw::c_int = 3;
pub const AL_EQUALIZER_MID1_CENTER: ::std::os::raw::c_int = 4;
pub const AL_EQUALIZER_MID1_WIDTH: ::std::os::raw::c_int = 5;
pub const AL_EQUALIZER_MID2_GAIN: ::std::os::raw::c_int = 6;
pub const AL_EQUALIZER_MID2_CENTER: ::std::os::raw::c_int = 7;
pub const AL_EQUALIZER_MID2_WIDTH: ::std::os::raw::c_int = 8;
pub const AL_EQUALIZER_HIGH_GAIN: ::std::os::raw::c_int = 9;
pub const AL_EQUALIZER_HIGH_CUTOFF: ::std::os::raw::c_int = 10;
pub const AL_EFFECT_FIRST_PARAMETER: ::std::os::raw::c_int = 0;
pub const AL_EFFECT_LAST_PARAMETER: ::std::os::raw::c_int = 32768;
pub const AL_EFFECT_TYPE: ::std::os::raw::c_int = 32769;
pub const AL_EFFECT_NULL: ::std::os::raw::c_int = 0;
pub const AL_EFFECT_REVERB: ::std::os::raw::c_int = 1;
pub const AL_EFFECT_CHORUS: ::std::os::raw::c_int = 2;
pub const AL_EFFECT_DISTORTION: ::std::os::raw::c_int = 3;
pub const AL_EFFECT_ECHO: ::std::os::raw::c_int = 4;
pub const AL_EFFECT_FLANGER: ::std::os::raw::c_int = 5;
pub const AL_EFFECT_FREQUENCY_SHIFTER: ::std::os::raw::c_int = 6;
pub const AL_EFFECT_VOCAL_MORPHER: ::std::os::raw::c_int = 7;
pub const AL_EFFECT_PITCH_SHIFTER: ::std::os::raw::c_int = 8;
pub const AL_EFFECT_RING_MODULATOR: ::std::os::raw::c_int = 9;
pub const AL_EFFECT_AUTOWAH: ::std::os::raw::c_int = 10;
pub const AL_EFFECT_COMPRESSOR: ::std::os::raw::c_int = 11;
pub const AL_EFFECT_EQUALIZER: ::std::os::raw::c_int = 12;
pub const AL_EFFECT_EAXREVERB: ::std::os::raw::c_int = 32768;
pub const AL_EFFECTSLOT_EFFECT: ::std::os::raw::c_int = 1;
pub const AL_EFFECTSLOT_GAIN: ::std::os::raw::c_int = 2;
pub const AL_EFFECTSLOT_AUXILIARY_SEND_AUTO: ::std::os::raw::c_int = 3;
pub const AL_EFFECTSLOT_NULL: ::std::os::raw::c_int = 0;
pub const AL_LOWPASS_GAIN: ::std::os::raw::c_int = 1;
pub const AL_LOWPASS_GAINHF: ::std::os::raw::c_int = 2;
pub const AL_HIGHPASS_GAIN: ::std::os::raw::c_int = 1;
pub const AL_HIGHPASS_GAINLF: ::std::os::raw::c_int = 2;
pub const AL_BANDPASS_GAIN: ::std::os::raw::c_int = 1;
pub const AL_BANDPASS_GAINLF: ::std::os::raw::c_int = 2;
pub const AL_BANDPASS_GAINHF: ::std::os::raw::c_int = 3;
pub const AL_FILTER_FIRST_PARAMETER: ::std::os::raw::c_int = 0;
pub const AL_FILTER_LAST_PARAMETER: ::std::os::raw::c_int = 32768;
pub const AL_FILTER_TYPE: ::std::os::raw::c_int = 32769;
pub const AL_FILTER_NULL: ::std::os::raw::c_int = 0;
pub const AL_FILTER_LOWPASS: ::std::os::raw::c_int = 1;
pub const AL_FILTER_HIGHPASS: ::std::os::raw::c_int = 2;
pub const AL_FILTER_BANDPASS: ::std::os::raw::c_int = 3;
pub const AL_CHORUS_WAVEFORM_SINUSOID: ::std::os::raw::c_int = 0;
pub const AL_CHORUS_WAVEFORM_TRIANGLE: ::std::os::raw::c_int = 1;
pub const AL_CHORUS_MIN_WAVEFORM: ::std::os::raw::c_int = 0;
pub const AL_CHORUS_MAX_WAVEFORM: ::std::os::raw::c_int = 1;
pub const AL_CHORUS_DEFAULT_WAVEFORM: ::std::os::raw::c_int = 1;
pub const AL_CHORUS_MIN_PHASE: ::std::os::raw::c_int = -180;
pub const AL_CHORUS_MAX_PHASE: ::std::os::raw::c_int = 180;
pub const AL_CHORUS_DEFAULT_PHASE: ::std::os::raw::c_int = 90;
pub const AL_FLANGER_WAVEFORM_SINUSOID: ::std::os::raw::c_int = 0;
pub const AL_FLANGER_WAVEFORM_TRIANGLE: ::std::os::raw::c_int = 1;
pub const AL_FLANGER_MIN_WAVEFORM: ::std::os::raw::c_int = 0;
pub const AL_FLANGER_MAX_WAVEFORM: ::std::os::raw::c_int = 1;
pub const AL_FLANGER_DEFAULT_WAVEFORM: ::std::os::raw::c_int = 1;
pub const AL_FLANGER_MIN_PHASE: ::std::os::raw::c_int = -180;
pub const AL_FLANGER_MAX_PHASE: ::std::os::raw::c_int = 180;
pub const AL_FLANGER_DEFAULT_PHASE: ::std::os::raw::c_int = 0;
pub const AL_FREQUENCY_SHIFTER_MIN_LEFT_DIRECTION: ::std::os::raw::c_int = 0;
pub const AL_FREQUENCY_SHIFTER_MAX_LEFT_DIRECTION: ::std::os::raw::c_int = 2;
pub const AL_FREQUENCY_SHIFTER_DEFAULT_LEFT_DIRECTION: ::std::os::raw::c_int =
    0;
pub const AL_FREQUENCY_SHIFTER_DIRECTION_DOWN: ::std::os::raw::c_int = 0;
pub const AL_FREQUENCY_SHIFTER_DIRECTION_UP: ::std::os::raw::c_int = 1;
pub const AL_FREQUENCY_SHIFTER_DIRECTION_OFF: ::std::os::raw::c_int = 2;
pub const AL_FREQUENCY_SHIFTER_MIN_RIGHT_DIRECTION: ::std::os::raw::c_int = 0;
pub const AL_FREQUENCY_SHIFTER_MAX_RIGHT_DIRECTION: ::std::os::raw::c_int = 2;
pub const AL_FREQUENCY_SHIFTER_DEFAULT_RIGHT_DIRECTION: ::std::os::raw::c_int
          =
    0;
pub const AL_VOCAL_MORPHER_MIN_PHONEMEA: ::std::os::raw::c_int = 0;
pub const AL_VOCAL_MORPHER_MAX_PHONEMEA: ::std::os::raw::c_int = 29;
pub const AL_VOCAL_MORPHER_DEFAULT_PHONEMEA: ::std::os::raw::c_int = 0;
pub const AL_VOCAL_MORPHER_MIN_PHONEMEA_COARSE_TUNING: ::std::os::raw::c_int =
    -24;
pub const AL_VOCAL_MORPHER_MAX_PHONEMEA_COARSE_TUNING: ::std::os::raw::c_int =
    24;
pub const AL_VOCAL_MORPHER_DEFAULT_PHONEMEA_COARSE_TUNING:
          ::std::os::raw::c_int =
    0;
pub const AL_VOCAL_MORPHER_MIN_PHONEMEB: ::std::os::raw::c_int = 0;
pub const AL_VOCAL_MORPHER_MAX_PHONEMEB: ::std::os::raw::c_int = 29;
pub const AL_VOCAL_MORPHER_DEFAULT_PHONEMEB: ::std::os::raw::c_int = 10;
pub const AL_VOCAL_MORPHER_MIN_PHONEMEB_COARSE_TUNING: ::std::os::raw::c_int =
    -24;
pub const AL_VOCAL_MORPHER_MAX_PHONEMEB_COARSE_TUNING: ::std::os::raw::c_int =
    24;
pub const AL_VOCAL_MORPHER_DEFAULT_PHONEMEB_COARSE_TUNING:
          ::std::os::raw::c_int =
    0;
pub const AL_VOCAL_MORPHER_PHONEME_A: ::std::os::raw::c_int = 0;
pub const AL_VOCAL_MORPHER_PHONEME_E: ::std::os::raw::c_int = 1;
pub const AL_VOCAL_MORPHER_PHONEME_I: ::std::os::raw::c_int = 2;
pub const AL_VOCAL_MORPHER_PHONEME_O: ::std::os::raw::c_int = 3;
pub const AL_VOCAL_MORPHER_PHONEME_U: ::std::os::raw::c_int = 4;
pub const AL_VOCAL_MORPHER_PHONEME_AA: ::std::os::raw::c_int = 5;
pub const AL_VOCAL_MORPHER_PHONEME_AE: ::std::os::raw::c_int = 6;
pub const AL_VOCAL_MORPHER_PHONEME_AH: ::std::os::raw::c_int = 7;
pub const AL_VOCAL_MORPHER_PHONEME_AO: ::std::os::raw::c_int = 8;
pub const AL_VOCAL_MORPHER_PHONEME_EH: ::std::os::raw::c_int = 9;
pub const AL_VOCAL_MORPHER_PHONEME_ER: ::std::os::raw::c_int = 10;
pub const AL_VOCAL_MORPHER_PHONEME_IH: ::std::os::raw::c_int = 11;
pub const AL_VOCAL_MORPHER_PHONEME_IY: ::std::os::raw::c_int = 12;
pub const AL_VOCAL_MORPHER_PHONEME_UH: ::std::os::raw::c_int = 13;
pub const AL_VOCAL_MORPHER_PHONEME_UW: ::std::os::raw::c_int = 14;
pub const AL_VOCAL_MORPHER_PHONEME_B: ::std::os::raw::c_int = 15;
pub const AL_VOCAL_MORPHER_PHONEME_D: ::std::os::raw::c_int = 16;
pub const AL_VOCAL_MORPHER_PHONEME_F: ::std::os::raw::c_int = 17;
pub const AL_VOCAL_MORPHER_PHONEME_G: ::std::os::raw::c_int = 18;
pub const AL_VOCAL_MORPHER_PHONEME_J: ::std::os::raw::c_int = 19;
pub const AL_VOCAL_MORPHER_PHONEME_K: ::std::os::raw::c_int = 20;
pub const AL_VOCAL_MORPHER_PHONEME_L: ::std::os::raw::c_int = 21;
pub const AL_VOCAL_MORPHER_PHONEME_M: ::std::os::raw::c_int = 22;
pub const AL_VOCAL_MORPHER_PHONEME_N: ::std::os::raw::c_int = 23;
pub const AL_VOCAL_MORPHER_PHONEME_P: ::std::os::raw::c_int = 24;
pub const AL_VOCAL_MORPHER_PHONEME_R: ::std::os::raw::c_int = 25;
pub const AL_VOCAL_MORPHER_PHONEME_S: ::std::os::raw::c_int = 26;
pub const AL_VOCAL_MORPHER_PHONEME_T: ::std::os::raw::c_int = 27;
pub const AL_VOCAL_MORPHER_PHONEME_V: ::std::os::raw::c_int = 28;
pub const AL_VOCAL_MORPHER_PHONEME_Z: ::std::os::raw::c_int = 29;
pub const AL_VOCAL_MORPHER_WAVEFORM_SINUSOID: ::std::os::raw::c_int = 0;
pub const AL_VOCAL_MORPHER_WAVEFORM_TRIANGLE: ::std::os::raw::c_int = 1;
pub const AL_VOCAL_MORPHER_WAVEFORM_SAWTOOTH: ::std::os::raw::c_int = 2;
pub const AL_VOCAL_MORPHER_MIN_WAVEFORM: ::std::os::raw::c_int = 0;
pub const AL_VOCAL_MORPHER_MAX_WAVEFORM: ::std::os::raw::c_int = 2;
pub const AL_VOCAL_MORPHER_DEFAULT_WAVEFORM: ::std::os::raw::c_int = 0;
pub const AL_PITCH_SHIFTER_MIN_COARSE_TUNE: ::std::os::raw::c_int = -12;
pub const AL_PITCH_SHIFTER_MAX_COARSE_TUNE: ::std::os::raw::c_int = 12;
pub const AL_PITCH_SHIFTER_DEFAULT_COARSE_TUNE: ::std::os::raw::c_int = 12;
pub const AL_PITCH_SHIFTER_MIN_FINE_TUNE: ::std::os::raw::c_int = -50;
pub const AL_PITCH_SHIFTER_MAX_FINE_TUNE: ::std::os::raw::c_int = 50;
pub const AL_PITCH_SHIFTER_DEFAULT_FINE_TUNE: ::std::os::raw::c_int = 0;
pub const AL_RING_MODULATOR_SINUSOID: ::std::os::raw::c_int = 0;
pub const AL_RING_MODULATOR_SAWTOOTH: ::std::os::raw::c_int = 1;
pub const AL_RING_MODULATOR_SQUARE: ::std::os::raw::c_int = 2;
pub const AL_RING_MODULATOR_MIN_WAVEFORM: ::std::os::raw::c_int = 0;
pub const AL_RING_MODULATOR_MAX_WAVEFORM: ::std::os::raw::c_int = 2;
pub const AL_RING_MODULATOR_DEFAULT_WAVEFORM: ::std::os::raw::c_int = 0;
pub const AL_COMPRESSOR_MIN_ONOFF: ::std::os::raw::c_int = 0;
pub const AL_COMPRESSOR_MAX_ONOFF: ::std::os::raw::c_int = 1;
pub const AL_COMPRESSOR_DEFAULT_ONOFF: ::std::os::raw::c_int = 1;
pub type LPALGENEFFECTS =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALsizei,
                                               arg2: *mut ALuint)>;
pub type LPALDELETEEFFECTS =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALsizei,
                                               arg2: *const ALuint)>;
pub type LPALISEFFECT =
    ::std::option::Option<extern "C" fn(arg1: ALuint) -> ALboolean>;
pub type LPALEFFECTI =
    ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum,
                                        arg3: ALint)>;
pub type LPALEFFECTIV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *const ALint)>;
pub type LPALEFFECTF =
    ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum,
                                        arg3: ALfloat)>;
pub type LPALEFFECTFV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *const ALfloat)>;
pub type LPALGETEFFECTI =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALint)>;
pub type LPALGETEFFECTIV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALint)>;
pub type LPALGETEFFECTF =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALfloat)>;
pub type LPALGETEFFECTFV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALfloat)>;
pub type LPALGENFILTERS =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALsizei,
                                               arg2: *mut ALuint)>;
pub type LPALDELETEFILTERS =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALsizei,
                                               arg2: *const ALuint)>;
pub type LPALISFILTER =
    ::std::option::Option<extern "C" fn(arg1: ALuint) -> ALboolean>;
pub type LPALFILTERI =
    ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum,
                                        arg3: ALint)>;
pub type LPALFILTERIV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *const ALint)>;
pub type LPALFILTERF =
    ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum,
                                        arg3: ALfloat)>;
pub type LPALFILTERFV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *const ALfloat)>;
pub type LPALGETFILTERI =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALint)>;
pub type LPALGETFILTERIV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALint)>;
pub type LPALGETFILTERF =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALfloat)>;
pub type LPALGETFILTERFV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALfloat)>;
pub type LPALGENAUXILIARYEFFECTSLOTS =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALsizei,
                                               arg2: *mut ALuint)>;
pub type LPALDELETEAUXILIARYEFFECTSLOTS =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALsizei,
                                               arg2: *const ALuint)>;
pub type LPALISAUXILIARYEFFECTSLOT =
    ::std::option::Option<extern "C" fn(arg1: ALuint) -> ALboolean>;
pub type LPALAUXILIARYEFFECTSLOTI =
    ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum,
                                        arg3: ALint)>;
pub type LPALAUXILIARYEFFECTSLOTIV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *const ALint)>;
pub type LPALAUXILIARYEFFECTSLOTF =
    ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum,
                                        arg3: ALfloat)>;
pub type LPALAUXILIARYEFFECTSLOTFV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *const ALfloat)>;
pub type LPALGETAUXILIARYEFFECTSLOTI =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALint)>;
pub type LPALGETAUXILIARYEFFECTSLOTIV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALint)>;
pub type LPALGETAUXILIARYEFFECTSLOTF =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALfloat)>;
pub type LPALGETAUXILIARYEFFECTSLOTFV =
    ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum,
                                               arg3: *mut ALfloat)>;
