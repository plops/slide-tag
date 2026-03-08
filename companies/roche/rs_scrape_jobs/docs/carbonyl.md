# Integration testing of webserver

The webserver needs a secret to be able to use github oauth. Run it like so:
```
source ~/rs_scrape_github.sh ; cargo run --bin stage10_web --features "db web"
```

Load the secret service account and password from this file:
~/github-service-account.txt

Don't leak these secrets by commiting it to the repository, printing it to logs or writting it in clear text to the website.

Next, start a webbrowser and try the github oauth authorization.

```
/usr/bin/carboxyl http://127.0.0.1:3000
```

This webbrowser is text based so you should be able to use it.
The next section gives details.

# Carbonyl Mouse Input Events

This note documents how to synthesize mouse events for a running `carbonyl` TTY session.

## Protocol used

`carbonyl` enables terminal mouse tracking and accepts **SGR mouse reporting** (DECSET `1006`), visible in startup output like:

- `\x1b[?1003h` (all-motion tracking)
- `\x1b[?1006h` (SGR extended mouse format)

In SGR mode, an event is encoded as:

```text
ESC [ < Cb ; Cx ; Cy M
ESC [ < Cb ; Cx ; Cy m
```

- `M` = button press / wheel event
- `m` = button release
- `Cb` = button/modifier code
- `Cx`, `Cy` = 1-based terminal cell coordinates (column, row)

## Common button codes

- `0` = left button
- `1` = middle button
- `2` = right button
- `64` = wheel up
- `65` = wheel down

Notes:
- For normal click, send press (`M`) then release (`m`) at the same coordinates.
- Wheel events are typically sent as `M` only.
- Modifier bits are added to `Cb` by the terminal protocol, but plain events usually do not need them.

## Examples

## 1) Left click at column 5, row 15

```text
\x1b[<0;5;15M\x1b[<0;5;15m
```

This is the sequence used in-session to open an NPR headline.

## 2) Scroll down at column 40, row 20

```text
\x1b[<65;40;20M
```

## 3) Scroll up at column 40, row 20

```text
\x1b[<64;40;20M
```

## Sending events from this environment

If `carbonyl` is running in an interactive exec session, inject events with `write_stdin` by passing raw escape sequences.

If testing in a local shell/PTY, you can also use `printf`:

```bash
printf '\e[<0;5;15M\e[<0;5;15m'
printf '\e[<65;40;20M'
```

## Practical caveats

- Coordinates are terminal-cell based, not pixel based.
- Resizing the terminal changes click targets.
- UI repaint in this environment may be sparse, so confirm navigation via title/URL changes.
