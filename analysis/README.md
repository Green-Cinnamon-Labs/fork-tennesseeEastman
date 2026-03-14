# TEP Analysis Package

This folder contains the Python package `tep-analysis`.

## Why this package exists

`tep-analysis` exists to provide a clean, reproducible way to analyze and visualize simulation outputs from the Tennessee Eastman service.

Main reasons:
- Keep plotting and data analysis logic separate from the Rust simulation service.
- Provide a Python CLI command (`plot`) for quick graph generation.
- Ensure dependency isolation through Poetry, so everyone uses the same package versions.

## Prerequisites

Run these commands from the `analysis/` directory.

- Install dependencies:

```bash
poetry install
```

- (Optional) confirm which virtual environment Poetry will use:

```bash
poetry env info --path
```

## Commands

### 1. Generate the plot (default CSV path)

```bash
poetry run plot
```

This uses the script entry point defined in `pyproject.toml`:
- `plot = "tep_analysis.plot:main"`

### 2. Generate the plot with an explicit CSV file

```bash
poetry run plot --csv ../tennessee-eastman-service/simulation_log.csv
```

### 3. Equivalent Python module command

```bash
poetry run python -m tep_analysis.plot
```

## How to verify it is using the Poetry virtual environment

Use:

```bash
poetry run python -c "import sys; print(sys.executable)"
```

The output Python path should point to Poetry's virtual environment (not a global system Python).

## Output

The plot image is saved next to the CSV file, with `.png` extension.

Example:
- `../tennessee-eastman-service/simulation_log.csv`
- `../tennessee-eastman-service/simulation_log.png`
