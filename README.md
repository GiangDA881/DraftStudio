# Draft Studio

Ready-to-edit CapCut PC projects from your clips, photos, voiceovers and music.

English · [Tiếng Việt](README.vi.md)

> **Unofficial.** Not affiliated with, endorsed by or sponsored by CapCut or ByteDance; "CapCut" is
> their trademark and only names the editor these drafts open in. The CapCut draft format is not
> documented: it was worked out from drafts CapCut writes itself, so a CapCut update can break it.
> MIT licensed, see [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Build ready-to-edit **CapCut PC projects** from photos, video clips, voiceovers and music, from a
desktop app, a command line or an AI agent over MCP. Open CapCut and the timeline is already there:
cuts, Ken Burns moves, titles, lower thirds, motion graphics, music that ducks under the voice.

## What it does

- **Main track**: photos and clips back to back, with Ken Burns zoom and pans written as CapCut
  keyframes.
- **Narration sections**: give each voiceover its clips and photos; their lengths are fitted to the
  voice automatically and every section cut lands exactly on its voice start (details below).
- **Graphics** drawn by [VectorCraft](https://github.com/storytold/vectorcraft) as transparent PNGs
  (20 templates): `lower_third`, `chapter_title`, `quote_card`, `location_tag`, `headline_bar`,
  `subtitle`, `fact_box`, `subscribe`, `end_card`, `car_spec_hud`, `vs_compare`, `breaking_banner`,
  `price_tag`, `neon_title`, `step_callout`, `stat_bar`, `social_handle`, `minimal_corner`,
  `top_list_badge`, `cinematic_letterbox`.
- **Motion** rendered by [EffectCraft](https://github.com/storytold/effectcraft) as alpha video
  (ProRes 4444 `.mov` or WebM): 70 motion, kinetic text, transition & background presets (`fade`,
  `slide_left`, `slide_right`, `rise`, `pop`, `blur_in`, `wipe`, `drift`, `glow_pop`, `light_sweep`,
  `typewriter`, `neon_hud`, …) plus 307 After Effects layer effects (`ec.*`).
- **FilmCraft FX**: 112 video transitions, 26 Creative Look LUTs & Lumetri presets, 139 video effects
  & animated presets, and 61 audio effects & transitions.
- **Music**: loops to the end, fades in and out, ducks under the voice with volume keyframes.
- **CapCut integration**: the draft appears at the top of CapCut's Projects tab; rebuilding a draft
  with the same name moves the old one to a backup folder (CapCut keeps a live copy and would ignore
  an overwrite). Fixes the `\\?\` path prefix that makes CapCut crash or report missing media.
- **Preview**: [FilmCraft](https://github.com/storytold/filmcraft) can render the same cut as a
  low-res MP4, without opening CapCut.
- **Agents**: an MCP server exposes the same build function the UI uses, so Claude Desktop, Cursor
  or any MCP client can assemble a CapCut project from a JSON spec, and the desktop app shows that
  build live while it happens.

## Requirements

- Windows 10/11 and **CapCut PC** (desktop). Drafts follow the format of CapCut PC 9.6 for Windows.
- Optional, for graphics, motion and the MP4 preview: the three craft CLIs
  (`vectorcraft-cli.exe`, `effectcraft-cli.exe`, `filmcraft-cli.exe`). Without them you still get
  scenes, Ken Burns, voice and music.
- Optional: `ffmpeg` / `ffprobe` on `PATH` (cover frame from a video, media probing fallback).
- The interface and messages are in Vietnamese for now.

## Install

**Prebuilt**: download the release zip, unzip anywhere. It holds `DraftStudio.exe` (desktop
app), `capcut_draft_app.exe` (CLI + MCP server) and the three craft CLIs in `crafts/` (their
licenses in `crafts/licenses/`), so graphics, motion and the MP4 preview work right away. A zip
without them (the CI build) takes them in that same `crafts/` folder, or anywhere on `PATH`.

**From source** (Rust stable):

```bash
cargo test -p capcut-core
cargo build --release -p capcut-studio -p capcut-draft-cli
```

That needs Visual Studio Build Tools (MSVC). Without MSVC, use the GNU toolchain and a portable
MinGW: `rustup toolchain install stable-x86_64-pc-windows-gnu`,
`winget install BrechtSanders.WinLibs.POSIX.MSVCRT`, then `source env.sh` in Git Bash before the
commands above. GNU builds land in `target/x86_64-pc-windows-gnu/release/` and need
`WebView2Loader.dll` next to `DraftStudio.exe`.

**Craft engines**: build each one from its repository (`cargo build --release -p vectorcraft-cli`,
and likewise for effectcraft and filmcraft). The app looks for each CLI, first match wins: the path
set in Settings → `VECTORCRAFT_CLI` / `EFFECTCRAFT_CLI` / `FILMCRAFT_CLI` environment variables →
`crafts/` next to the app → next to the app → `PATH` → a craft engines folder (Settings, or the
`CRAFT_ENGINES_DIR` environment variable) holding the cloned repositories.

## Use it

### Desktop app

Run `DraftStudio.exe`. Add scenes or narration sections, graphics, motion and music, preview
the timeline, then **Xuất sang CapCut** (export). `DraftStudio.exe serve` opens the same UI in
a browser at http://127.0.0.1:8797. If CapCut was open while you exported, quit it completely and
start it again: CapCut rewrites its project list when it exits.

### Command line

```bash
capcut_draft_app status                       # CapCut folder, craft engines, ffmpeg
capcut_draft_app create --spec examples/narrated_sections.json
capcut_draft_app slideshow --name "Trip" --images C:/media/photos --voice C:/media/voice.mp3 --bgm C:/media/music.mp3
capcut_draft_app list | inspect "Draft name" | repair "Draft name"
capcut_draft_app templates                    # graphic templates + motion presets
capcut_draft_app graphic --template lower_third --field "name=Seneca" --field "role=Philosopher"
capcut_draft_app motion --preset slide_left --template lower_third --duration 5
capcut_draft_app probe C:/media/clip.mp4
```

### MCP (AI agents)

```json
{ "mcpServers": { "draft-studio": { "command": "C:\\path\\to\\capcut_draft_app.exe", "args": ["mcp"] } } }
```

Tools: `capcut_get_status`, `capcut_list_drafts`, `capcut_get_draft`, `capcut_live_edit`,
`capcut_build_project`, `capcut_create_draft` (v1 field names), `capcut_quick_slideshow`,
`capcut_repair_draft`, `craft_list_templates`, `vectorcraft_render_graphic`,
`effectcraft_render_motion`, `filmcraft_probe`.

### Watch an agent build live

Keep Draft Studio open while an agent works and every MCP tool call shows up as it happens.

- An **Agent** chip in the top bar pulses while a call runs. The **Agent** tab on the left lists the
  agent's recent calls with a live, terminal-style log of each.
- A build (`capcut_build_project`, `capcut_create_draft`, `capcut_quick_slideshow`) puts the agent's
  spec on the stage and timeline, read-only. Overlays fill in as VectorCraft and EffectCraft finish
  them (WebM motion plays; a ProRes `.mov` shows its graphic, browsers cannot play ProRes), and the
  FilmCraft MP4 plays as soon as it is rendered.
- Your open project is saved first and left alone: **Quay lại dự án** goes back to it,
  **Mở thành dự án** saves the agent's spec as a new studio project you can edit.
- The studio switches to a build when it starts, unless you are typing or a dialog is open (then you
  get a notice with a **Xem** button). Settings → *Xem agent làm việc* turns that off, and has an
  option, off by default, to **open the studio when an agent starts a build**: the MCP server then
  starts `DraftStudio.exe` from its own folder if no studio answers, or the open window comes to the
  front.
- A studio opened in the middle of a build still gets the whole call so far. With the studio closed
  nothing changes for the agent: events go out fire-and-forget from a background thread (local HTTP,
  short timeouts, a bounded queue), and the build log still goes to stderr.

Under the hood the MCP server posts events to `POST /api/activity` on the studio port (8797, or
`CAPCUT_STUDIO_PORT`) and every open UI follows `GET /api/activity/stream` (Server-Sent Events); see
`crates/core/src/activity.rs` and `apps/studio/src/server.rs`.

## Project spec

A JSON file (or the `spec` argument of `capcut_build_project`). Times are in seconds; paths are
absolute. Examples: [narrated_sections.json](examples/narrated_sections.json),
[slideshow.json](examples/slideshow.json) (their paths are placeholders).

| Field | Meaning |
|---|---|
| `project_name` | Draft name in CapCut (no `/ \ : * ? " < > \|`) |
| `width`, `height`, `fps` | Canvas, default 1920×1080 at 30 |
| `sections` | Narrated blocks, laid out first: `{voice, scenes, gap_after_sec, desc}` |
| `scenes` | Main-track clips after the sections (or alone): `path`, `duration_sec`, `trim_start_sec`, `ken_burns`, `zoom_start`, `zoom_end`, `pan` (none/left/right/up/down), `volume`, `desc` |
| `voiceovers` | Free voiceovers: `path`, `start_sec` (default: right after the previous one), `volume` |
| `fit_scenes_to_voice` | Photos in `scenes` without a duration share the narration left after the sections |
| `overlays` | `kind`: `file` (`path`) / `graphic` (`graphic.template`, `graphic.fields`, `accent`, `font`) / `motion` (`motion.preset`, from a graphic or `path`); `start_sec`, `section`, `duration_sec`, `x`, `y`, `scale`, `opacity`, `fade_in_sec`, `fade_out_sec`. Footage and pictures (`file`, e.g. B-roll) stack below graphics and motion, so titles stay on top |
| `bgm` | `path`, `volume` (0.3), `fade_in_sec`, `fade_out_sec`, `loop_to_fill`, `duck_under_voice`, `duck_ratio` (0.4) |
| `register_to_capcut` | Put the draft on CapCut's home screen (default true) |
| `output_dir` | Draft folder; default inside CapCut's draft root |
| `film_preview` | Also render an MP4 preview with FilmCraft |

### Narration sections

```json
"sections": [
  {"voice": {"path": "C:/media/voice/01.mp3"}, "scenes": [{"path": "C:/media/clips/a.mp4"}, {"path": "C:/media/clips/b.mp4"}]},
  {"voice": {"path": "C:/media/voice/02.mp3"}, "gap_after_sec": 0.5,
   "scenes": [{"path": "C:/media/photos/portrait.jpg", "duration_sec": 6, "ken_burns": true}, {"path": "C:/media/clips/c.mp4"}]}
],
"overlays": [{"kind": "motion", "section": 1, "start_sec": 0.6, "graphic": {"template": "lower_third", "fields": {"name": "Alfred Adler"}}, "motion": {"preset": "slide_left"}}]
```

- Sections start at 0 and follow each other: section N+1 starts exactly when voice N ends, plus
  `gap_after_sec` (a pause covered by section N's pictures). Each boundary is a cut on the main
  track, aligned to the microsecond with the voice start.
- Inside a section, scenes with `duration_sec` keep it (a video is capped at its length). The others
  share the rest: **photos stretch to fill the slack**, **videos are trimmed proportionally from the
  tail** and never run past their file (after `trim_start_sec`). While videos are being trimmed,
  each free photo keeps at least 2 s.
- Too little footage for a voice is an error that names the section and the missing seconds, never
  a black gap. Fix it with another clip, a photo (photos stretch) or by dropping a photo's
  `duration_sec`.
- A section's voice takes no `start_sec`. An overlay with `section` (counted from 0) times its
  `start_sec` from that section's start, so graphics stay on their line when a voice file changes.
- Top-level `scenes` and `voiceovers` still work as before and play after the last section.

## Where files go

| What | Where |
|---|---|
| Drafts | `%LOCALAPPDATA%\CapCut\User Data\Projects\com.lveditor.draft\<project_name>` (override with `output_dir` or `CAPCUT_DRAFT_ROOT`) |
| Generated graphics, motion, previews | `studio_assets\` next to the app (Settings → assets folder). CapCut links these files: keep them. |
| Settings, saved studio projects | `%APPDATA%\CapCutDraftStudio\` |
| Studio port | 8797 (`CAPCUT_STUDIO_PORT`) |

## Known limits

- Windows only; the macOS draft folder is wired up but untested.
- Unofficial draft format: test after CapCut updates.
- Graphics, motion and the preview need the craft CLIs.
- Vietnamese interface.

## Source layout

```
crates/core     draft writer, spec, section fitting, draft library, craft bridges, graphic templates, MCP,
                live activity feed to the studio (activity.rs)
apps/cli        capcut_draft_app.exe (CLI + MCP server)
apps/studio     DraftStudio.exe: axum server (API + embedded UI) in a Tauri window; ui/ is plain HTML/CSS/JS
examples        public example specs (placeholder paths)
```

The studio timeline (`apps/studio/ui/app.js`, `compute()` / `fitSection`) mirrors the Rust layout
(`crates/core/src/build.rs`, `fit_section`); change both together.

## License

MIT, see [LICENSE](LICENSE). Third-party components and their licenses:
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
