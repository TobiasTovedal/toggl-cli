# toggl-cli

CLI for [Toggl time track](https://toggl.com/track/). To simplify my life since I am unfortunately required to time track some projects.

The only possible action is to add an entry with description to a project for a predefined duration starting _now_.

Accepts input on format `<PROJECT> <DURATION_IN_MINUTES> <DESCRIPTION>`. For example

```shell
toggl-cli useful-project 60 "Time well spent"
```

## Installing / Getting started

1. Add your `API_KEY` to `config.rs`
2. Add projects with id's to project vector in `config.rs`
3. Run `cargo build`
4. Add generated binary to `$PATH`
