use libc::*;
use crate::{
    abstracts::VectorLayout,
    frame::VideoFrame,
    base::*,
};

#[repr(i32)]
pub enum VideoFrameType {
    EmptyFrame = 0,
    // Wire format for MultiplexEncodedImagePacker seems to depend on
    // numerical values of these constants.
    VideoFrameKey = 3,
    VideoFrameDelta = 4,
}

#[repr(C)]
struct RawParameter {
    key:   *const c_char,
    value: *const c_char,
}

impl From<(&str, &str)> for RawParameter {
    fn from(value: (&str, &str)) -> Self {
        Self {
            key:   to_c_str(value.0).unwrap(),
            value: to_c_str(value.1).unwrap(),
        }
    }
}

#[repr(C)]
struct RawCodecSettings {
    // TODO(nisse): Change to int, for consistency.
    width:                       u16,
    height:                      u16,
    start_bitrate:               u32, // kilobits/sec.
    max_bitrate:                 u32, // kilobits/sec.
    min_bitrate:                 u32, // kilobits/sec.
    max_framerate:               u32,
    // This enables/disables encoding and sending when there aren't multiple
    // simulcast streams,by allocating 0 bitrate if inactive.
    active:                      bool,
    qp_max:                      u32,
    number_of_simulcast_streams: u8,
    expect_encode_from_texture:  bool,
    // Legacy Google conference mode flag for simulcast screenshare
    legacy_conference_mode:      bool,
    loss_notification:           bool,
    number_of_cores:             i32,
    max_payload_size:            usize,
}

#[repr(C)]
struct RawVideoEncoder {
    encoder: *const c_void,
}

#[repr(C)]
struct RawVideoEncoderFactory {
    factory: *const c_void,
}

pub struct VideoEncoderAdapter {}

pub trait VideoEncoderExt: Send {
    /// Initialize the encoder with the information from the codecSettings
    ///
    /// Input:
    ///          - codec_settings    : Codec settings
    ///          - settings          : Settings affecting the encoding itself.
    /// Input for deprecated version:
    ///          - number_of_cores   : Number of cores available for the encoder
    ///          - max_payload_size  : The maximum size each payload is allowed
    ///            to have. Usually MTU - overhead.
    ///
    /// Return value                  : Set bit rate if OK
    ///                                 <0 - Errors:
    ///                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
    ///                                  WEBRTC_VIDEO_CODEC_ERR_SIZE
    ///                                  WEBRTC_VIDEO_CODEC_MEMORY
    ///                                  WEBRTC_VIDEO_CODEC_ERROR
    fn init(&mut self, settings: ());
    /// Sets rate control parameters: bitrate, framerate, etc. These settings
    /// are instantaneous (i.e. not moving averages) and should apply from
    /// now until the next call to set_rates().
    fn set_rates(&mut self, parameters: ());
    /// Encode an image (as a part of a video stream). The encoded image
    /// will be returned to the user through the encode complete callback.
    ///
    /// Input:
    ///          - frame             : Image to be encoded
    ///          - frame_types       : Frame type to be generated by the
    ///            encoder.
    ///
    /// Return value                 : WEBRTC_VIDEO_CODEC_OK if OK
    ///                                <0 - Errors:
    ///                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
    ///                                  WEBRTC_VIDEO_CODEC_MEMORY
    ///                                  WEBRTC_VIDEO_CODEC_ERROR
    fn encode(
        &mut self,
        adapter: &mut VideoEncoderAdapter,
        frame: &VideoFrame,
        types: &[VideoFrameType],
    );
}

pub struct VideoEncoder {
    // ptr: *const RawVideoEncoder,
    ext: Box<dyn VideoEncoderExt>,   
}

impl VideoEncoder {
    pub fn new<T>(name: &str, pars: &[(&str, &str)], ext: T) -> Self 
    where
        T: VideoEncoderExt + 'static
    {
        Self {
            ext: Box::new(ext),
        }
    }
}

pub struct VideoEncoderFactory {
    ptr: *const RawVideoEncoderFactory,
}

impl VideoEncoderFactory {
    
}
