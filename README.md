# 🐶 Git Dog

**A pixel-art coding companion that grows with your GitHub activity.**

Git Dog is an original open-source Shiba/Corgi-style puppy, written in **Rust**. Our first milestone draws a little orange-and-cream dog with a floppy ear, curled tail, a red coding bandana, a doghouse, grassy backyard, toys, and a contribution-inspired board.

> **Milestone 001 is a functional prototype, not the final approved sprite.** It generates an animated GIF and still PNG. Real GitHub activity syncing, XP, multiple breeds, accessories, and automatic README updates come in later milestones.

## Run it

Install stable Rust and Cargo, then run:

```bash
cargo run --release -- --output dist/git-dog.gif --png dist/git-dog.png
```

Your GIF and PNG appear in `dist/` (ignored by Git). The workflow also creates a **git-dog-preview** artifact with both outputs, downloadable from a GitHub Actions run.

```bash
cargo run --release -- --output dist/idle.gif --mood idle
cargo run --release -- --output dist/sleepy.gif --mood sleepy
cargo run --release -- --output dist/playful.gif --mood playful --frames 32 --scale 3
cargo test
```

## Arguments

| Flag | Meaning |
| --- | --- |
| `--output` | Animated GIF file (default: `dist/git-dog.gif`) |
| `--png` | Optional still PNG file |
| `--mood` | `playful`, `idle`, or `sleepy` |
| `--frames` | 2–96 GIF frames (default 24) |
| `--scale` | Nearest-neighbor pixel multiplier 1–5 (default 3) |
| `--help` | CLI usage |

The drawing engine uses a logical **256×144 RGBA framebuffer**, loops its tail movement and expressions, and scales without smoothing. Rendering is deterministic for a given frame, mood, and total frame count. The generator does **not** access your GitHub account or need any tokens yet.

## Roadmap

- [x] Initialize Rust crate and CLI
- [x] Deterministic self-contained pixel renderer
- [x] Export looping GIF and PNG previews
- [x] Three moods, independent tail animation, tests
- [x] GitHub Actions build, lint, tests, and preview artifact
- [ ] Replace prototype shapes with a polished layered sprite faithful to the approved concept
- [ ] Add richer blinking, walk, bark, and sleeping animations
- [ ] Integrate GitHub activity API with rate-limit/error handling
- [ ] Add XP, levels, streaks, accessories, and evolution
- [ ] Schedule GitHub Actions to regenerate the README pet
- [ ] Add more breeds after the first puppy is polished

## Guiding principles

Git Dog's artwork, code, and animation system are our own. The approved concept image is our visual reference; nothing from Git Koi or Git Bonsai is copied. Taking a break never makes your puppy sick or causes it to die.

## License

MIT — see [LICENSE](LICENSE).
