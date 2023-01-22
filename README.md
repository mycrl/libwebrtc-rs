<h1 align="center">
    libwebrtc-rs
</h1>
<div align="center">
    <strong>Rust ❤️ WebRTC</strong>
    </br>
    <span>Facilitating high-level interactions between Rust and WebRTC</span>
</div>
</br>
<div align="center">
    <img src="https://img.shields.io/github/languages/top/mycrl/libwebrtc-rs"/>
    <img src="https://img.shields.io/github/license/mycrl/libwebrtc-rs"/>
    <img src="https://img.shields.io/github/issues/mycrl/libwebrtc-rs"/>
    <img src="https://img.shields.io/github/stars/mycrl/libwebrtc-rs"/>
</div>
<br/>
<br/>


The rust high-level abstraction binding of Google WebRTC [M99](https://groups.google.com/g/discuss-webrtc/c/Yf6c3HW4N3k/m/3SC_Hy15BQAJ). With WebRTC, you can add real-time communication capabilities to your application that works on top of an open standard. It supports video, voice, and generic data to be sent between peers, allowing developers to build powerful voice- and video-communication solutions.


### Quick start

Add the following to your Cargo.toml:

```toml
batrachia = "0.1"
```

There are simple example in the [example](https://github.com/mycrl/webrtc-rs-example) repo.

### Building

#### Automatic

The batrachia crate will automatically find the precompiled static library files in the git libwebrtc-rs repo release.

#### Manual

A set of environment variables can be used to point batrachia towards. They will override the automatic detection logic.

| key                 | description                                                                              |
|---------------------|------------------------------------------------------------------------------------------|
| WEBRTC_LIBRARY_PATH | webrtc static library path, this will skip downloading and use your static library.      |
| SYS_LIBRARY_PATH    | batrachiatc static library path, this will skip downloading and use your static library. |


### License
[GPL](./LICENSE) Copyright (c) 2022 Mr.Panda.
