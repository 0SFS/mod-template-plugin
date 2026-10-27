# 0SFS plugin mod template

The default plugin is Rust, built for `wasm32-unknown-unknown`, with no host imports and a 16 MiB memory cap. TypeScript is only a last resort: set `execution` to `js` and a `js.path` if you cannot ship WASM. A JavaScript mod is unsandboxed and does not start from a shared `?mods=` link.

1. Fork this template.
2. Edit `src/lib.rs` and `public/0sfs-mod.json`.
3. `cargo build --release --target wasm32-unknown-unknown`
4. `mkdir -p public && cp target/wasm32-unknown-unknown/release/osfs_mod.wasm public/mod.wasm`
5. `node scripts/validate-mod.mjs`
6. Push, enable GitHub Pages from `public/`, and add `owner/repo` in the Mods tab.

## Mod

This repository is a 0SFS mod. `public/0sfs-mod.json` is the manifest. GitHub Pages serves the `public/` directory at the site root.

Fork the repository, edit the manifest and the files it names, push, then add the GitHub URL in the simulator's Mods tab.

Topics: `0sfs-mod` and `0sfs-plugin`.

## Community

- [Code of conduct](https://github.com/0SFS/.github/blob/main/CODE_OF_CONDUCT.md)
- [Contributing](https://github.com/0SFS/.github/blob/main/CONTRIBUTING.md)
- [License](LICENSE)
- [Security](https://github.com/0SFS/.github/blob/main/SECURITY.md)
