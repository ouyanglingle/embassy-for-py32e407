## Generate embassy-stm32 structure diagram

Inputs already on disk (read-only):
- `D:\embassy-for-py32e407\参考设计\embassy-stm32-0.6.0\graph.json` (193 nodes, 375 edges, 35 group paths)
- `D:\embassy-for-py32e407\参考设计\embassy-stm32-0.6.0\json2drawio_layout.py` (graph.json → coordinate-less .drawio XML with nested swimlane containers)

Outputs to produce in the same directory:
- `D:\embassy-for-py32e407\参考设计\embassy-stm32-0.6.0\embassy-stm32-structure.drawio`
- `D:\embassy-for-py32e407\参考设计\embassy-stm32-0.6.0\embassy-stm32-structure.drawio.png` (embedded XML, repaired IEND)

Layout engine: drawio ELK `--layout` pass via `horizontalFlow` (fallback `verticalFlow`). Graphviz `dot` is unavailable on this system, and `autolayout.py` hard-exits without it, so this is the only path that matches both the existing `json2drawio_layout.py` script's design and the local tooling.

Steps:
1. `python json2drawio_layout.py graph.json -o embassy-stm32-structure.layout.drawio`
2. `"C:\Program Files\draw.io\draw.io.exe" -x -f xml --layout horizontalFlow -o embassy-stm32-structure.drawio embassy-stm32-structure.layout.drawio`
3. Preview PNG (no `-e`, width 2000): `draw.io.exe -x -f png --width 2000 -o embassy-stm32-structure.preview.png embassy-stm32-structure.drawio`; vision self-check for overlap/clipping; up to 2 fix-and-retry rounds
4. Final PNG with embedded XML: `draw.io.exe -x -f png -e -s 2 -o embassy-stm32-structure.drawio.png embassy-stm32-structure.drawio`; immediately run `python C:\Users\ouyan\.zcode\skills\drawio-skill\scripts\repair_png.py embassy-stm32-structure.drawio.png` to fix draw.io's truncated IEND chunk
5. Clean up intermediates (`embassy-stm32-structure.layout.drawio`, `embassy-stm32-structure.preview.png`) so only the two deliverables remain

Fallbacks: if the drawio CLI crashes/hangs, deliver the .drawio XML only or generate a `https://app.diagrams.net/` browser URL via `scripts/encode_drawio_url.py --edit`. If `horizontalFlow` produces an unwieldy layout, retry once with `verticalFlow` (single rerun, no script change).