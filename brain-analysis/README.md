# Mouse Brain Spatial Analysis

This project uses Python to load and analyze spatial transcriptomics data from a mouse brain experiment.

## Setup

This project uses `uv` for package and environment management.

1.  **Install uv:**
    Follow the instructions at [https://docs.astral.sh/uv/getting-started/installation/](https://docs.astral.sh/uv/getting-started/installation/)

2.  **Create the virtual environment and install dependencies:**
    ```sh
    uv sync
    ```

## Running the Script

To load the data and print its columns, run:

```sh
uv run brain_analysis
```