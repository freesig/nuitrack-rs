# {{crate}}

{{readme}}

# nuitrack-rs
A Rust crate giving access to the Nuitrack SDK

## Installation Guide
__Ubuntu 18.04__
1. Download the SDK from the [Nuitrack website](https://nuitrack.com). 
_If you are short on space you only need the NuitrackSDK/Nuitrack folder (you can delete the rest of the folders)_
2. Unzip it the SDK somewhere eg. `~/nuitrack/` 
3. Follow the ubuntu instractions [here](http://download.3divi.com/Nuitrack/doc/Installation_page.html)
_Make sure to get `libpng12-0`. It's avalible [here](https://packages.ubuntu.com/xenial/amd64/libpng12-0/download) 
if you can't find in apt-get_
4. Set the environment variable `NUI_SDK_DIR` to you sdk root directory. 
eg. `NUI_SDK_DIR = /home/user/nuitrack` _note not `/home/user/nuitrack/Nuitrack`
5. Then you should be able to build your project with `cargo build --release`.
To test try `cargo run --release --example skeleton` with a camera attached.
_Please open an issue if you get stuck on any step_

__Orbtec Astra__
I've only tested this with the Orbec Astra. 
To install the Orbtec SDK follow [these instructions.](https://astra-wiki.readthedocs.io/en/latest/installation.html)
