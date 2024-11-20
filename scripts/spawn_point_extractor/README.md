# spawn_point_extractor

Extracts spawn points from the `main_zone.tscn` file of WEBFISHING.

## Usage

To install dependencies:

```bash
bun install
```

To run:

Move `main_zones.tscn` to the root directory of this project. Running the script will read this file
then write the spawn points to `spawn_points.json`.

Note: This script uses `tscn2json`. This dependency does not handle some props in an unmodified
`main_zones.tscn` correctly. If you get errors, try removing any props that have `{` or `}`
characters in them.

```bash
bun run index.ts
```

