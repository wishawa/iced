## Direct Wgpu

A demonstration of how to render using wgpu directly from an iced widget.
This is different from the `integration_wgpu` example because here,
we render from inside an iced widget,
respecting iced layout and event handling system and all that,
while in `integration_wgpu`, we render iced on top of our wgpu code.

The __[`main`]__ file contains all the rust code of the example.
The shader code is in __[`shader/`]__

<div align="center">
  <a href="https://gfycat.com/SmugLawfulImago">
    <img src="https://thumbs.gfycat.com/SmugLawfulImago-small.gif">
  </a>
</div>

You can run it with `cargo run`:
```
cargo run --package direct_wgpu
```

[`main`]: src/main.rs
