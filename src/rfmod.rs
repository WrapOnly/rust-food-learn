/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FmodEx library, is provided by FIRELIGHT TECHNOLOGIES.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
rust-fmod
=========

This is a rust binding for __FMOD__, the library developped by FIRELIGHT TECHNOLOGIES.

__FMOD__ website : www.fmod.org


##Installation

You must install on your computer the __FMOD__ library which is used for the binding.

To build it, please use :

```Shell
> make
```

This command build __rfmod__, the examples, and the documentation.

You can build them separatly too.

```Shell
> make rfmod
> make examples
> make doc
```

##Short example

Here is a short example on how to create a file and to play it :

```Rust
#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::*;
use rfmod::*;
use std::os;

fn main() {
    let fmod = match FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("Error code : {}", e);
        }
    };

    match fmod.init() {
       enums::Result::Ok => {}
        e => {
            fmod.release();
            panic!("FmodSys.init failed : {}", e);
        }
    };

    let mut sound = match fmod.create_sound(StrBuf::from_str("music.mp3"), None, None) {
                      enums::Result::Ok(s) => s,
                      Err(err) => {panic!("Error code : {}", err);},
                    };

    match sound.play_to_the_end() {
       enums::Result::Ok => {println!("Ok !");}
        err => {panic!("Error code : {}", err);}
    };
}
```

For a more complete example : https://github.com/GuillaumeGomez/rust-music-player

##License

    Copyright (c) 2014 Guillaume Gomez

    The license of this project is the same of the FMOD non-commercial use.
    Please refer to it. Here is the website for FMOD : http://www.fmod.org/

#Notes

 * Members marked with [r] mean the variable is modified by FMOD and is for reading purposes only. Do not change this value.
 * Members marked with [w] mean the variable can be written to. The user can set the value.


Here is the list of all modules :
!*/

#![crate_name = "rfmod"]
#![desc = "Rust binding for FMOD"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

#![feature(globs)]

extern crate libc;

pub use channel::{Channel, FmodSpeakerMixOptions, FmodReverbChannelProperties};
pub use fmod_sys::{FmodSys, FmodGuid, FmodSoftwareFormat, FmodAdvancedSettings, FmodOutputHandle, FmodCreateSoundexInfo, FmodMemoryUsageDetails, FmodUserData};
pub use sound::{Sound, FmodTag, FmodSyncPoint};
pub use channel_group::{ChannelGroup};
pub use sound_group::SoundGroup;
pub use dsp::{Dsp, DspParameterDesc, DspDescription, DspState};
pub use dsp_connection::DspConnection;
pub use reverb::Reverb;
pub use reverb_properties::ReverbProperties;
pub use vector::FmodVector;
pub use geometry::Geometry;
pub use file::FmodFile;

mod ffi;
mod sound;
mod channel;
mod channel_group;
mod sound_group;
mod fmod_sys;
mod dsp;
mod dsp_connection;
mod geometry;
mod vector;
mod reverb;
mod reverb_properties;
mod file;
pub mod enums;
pub mod types;
pub mod callbacks;
pub mod error;


#[cfg(target_os = "linux")]
mod platform {
    #[cfg(target_arch="x86")]
    #[link(name = "fmodex")] extern{}
    #[cfg(target_arch="x86_64")]
    #[link(name = "fmodex64")] extern{}
}

#[cfg(target_os = "macos")]
mod platform {
    #[link(name = "fmodex")] extern{}
}
