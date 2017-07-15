### talkingclock
> :clock4: Command line application which says the time.

[![Build Status](https://travis-ci.org/stpettersens/talkingclock.png?branch=master)](https://travis-ci.org/stpettersens/talkingclock)
[![Build status](https://ci.appveyor.com/api/projects/status/1ekayunrux3ise2m?svg=true)](https://ci.appveyor.com/project/stpettersens/talkingclock)

* Written in response to [this /r/dailyprogrammer challenge](https://www.reddit.com/r/dailyprogrammer/comments/6jr76h/20170627_challenge_321_easy_talking_clock).

#### Building from source

Prerequisites to build are:
* [Rust tools](https://www.rust-lang.org) (rustc, cargo, etc).
* [Ruby](https://www.ruby-lang.org), [Rake](https://ruby.github.io/rake/), [gems](https://rubygems.org/pages/download) and [OS gem](https://rubygems.org/gems/os) (optional).
* [UPX](https://upx.github.io) (optional).

Runtime dependencies:
* Eli Fulkerson's [voice.exe](https://elifulkerson.com/projects/commandline-text-to-speech.php) and [sounder.exe](https://www.elifulkerson.com/projects/commandline-wav-player.php) utilities (*Windows*).
* *say* and *ffplay* (*Unices: BSD/Linux/Mac OS X*).

Building:

* `> rake` # or: `cargo build --release`
* `> rake upx` # if you want to compress resultant executable with UPX.
* `> rake install` # to install a global command (must have run `rake upx` first) to invoke the talking clock.

#### Usage

* `talkingclock` # say the current time.
* `talkingclock --time 13:05` # say one oh five / thirteen oh five.
* `talkingclock --help` # see all options and exit.
