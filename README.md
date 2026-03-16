# gv2bpmn

gv2bpmn is a tool for converting m4 based DSL for BPMN2.0 (small subset).
It's implemented on top of GraphViz dot.
The conversion goes in several steps:

1.  **Convert m4 input via bpmn.m4 to GraphViz notation:**
    ```bash
    m4 bpmn.m4 example.m4 > example.dot
    ```
2.  **Convert dot file to JSON via dot:**
    ```bash
    dot -Tjson example.dot > example.json
    ```
3.  **Convert JSON to BPMN2.0 XML via Rust tool:**
    ```bash
    # From stdin
    cargo run < example.json > output.bpmn

    # From file
    cargo run -- example.json > output.bpmn
    ```

## Usage

```text
Usage: gv2bpmn [FILE]

Arguments:
  [FILE]  Input Graphviz JSON file. If not provided, reads from stdin.

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Features

- Maps Graphviz nodes to BPMN elements (Start/End events, Tasks, Gateways) based on name prefixes or shapes.
- Preserves layout by converting Graphviz coordinates to BPMN DI format.
- Handles coordinate system transformation (flipping Y-axis).
