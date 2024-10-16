# toggl-cli

CLI for [Toggl time track](https://toggl.com/track/). To simplify my life since I am unfortunately required to time track some projects.

Add an entry with description to a project for a duration starting _now_.

Accepts input on format `<PROJECT> <DURATION_IN_MINUTES> <DESCRIPTION>`. For example

```shell
toggl-cli useful-project 60 "Time well spent"
```

## Installing / Getting started

1. Add your Toggle API key as an environment variable called `TOGGL_API_KEY` in your shell
2. Add projects with id's to project vector in `config.rs`
3. Run `cargo build`
4. Add generated binary to `$PATH`
