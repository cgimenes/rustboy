# Rustboy

Fourth attempt of writing a Gameboy emulator.

Everything started when I read the [Coffee GB Blog](https://blog.rekawek.eu/2017/02/09/coffee-gb). It motivated me to build my own. And so I started in 2018 the [neutronstar-gb](https://github.com/cgimenes/neutronstar-gb), a Gameboy emulator in Java.

After some time I stopped working on it for more than 2 years, but meanwhile I kept it on my mind. Java never felt "low-level" enough for an emulator, so I figured a rewrite would double as a chance to learn another language.

So, in 2020, I started [gomenes-boy](https://github.com/cgimenes/gomenes-boy), a rewrite in Go. It didn't survive for long, and I don't remember why.

After years on hold, I decided to work on this project again. My motivation returned after watching a lot of [Tsoding](https://www.youtube.com/tsoding) and [Tsoding Daily](https://www.youtube.com/@TsodingDaily). Naturally, that meant yet another rewrite: [thatone](https://github.com/cgimenes/thatone). It had a cool feature though: hot reloading. Whenever I changed files, it recompiled and swapped a dynamic library in memory.

Now, riding the current wave of Rust hype, I decided to finally finish this project and use it as an excuse to learn Rust.

## References

- https://gekkio.fi/files/gb-docs/gbctr.pdf
- https://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM
- https://www.youtube.com/watch?v=HyzD8pNlpwI
- https://github.com/mpostaire/gbmulator
- https://aselker.github.io/gameboy-sound-chip/
- https://gbdev.io/pandocs
- https://gbdev.gg8.se/wiki/articles/Sound_Controller
- https://aselker.github.io/gameboy-sound-chip/
