"""
Tennessee Eastman Process — simulation log plotter.

Usage (from the analysis/ directory):
    uv run python -m tep_analysis.plot
    uv run python -m tep_analysis.plot --csv ../tennessee-eastman-service/simulation_log.csv
"""

import argparse
import sys
from pathlib import Path

import pandas as pd
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

# ── default CSV path (relative to this file → repo root) ──────────────────────
_DEFAULT_CSV = Path(__file__).parent.parent.parent / \
    "tennessee-eastman-service" / "simulation_log.csv"

# ── ISD / alarm thresholds ────────────────────────────────────────────────────
ISD = {
    "XMEAS(7)":  (None, 3000.0),   # Reactor Pressure  kPa  — shut >3000
    "XMEAS(9)":  (None,  175.0),   # Reactor Temp      °C   — shut >175
    "XMEAS(8)":  (10.0,   90.0),   # Reactor Level     %    — shut <10 or >90
    "XMEAS(12)": (10.0,   90.0),   # Sep Level         %    — shut <10 or >90
    "XMEAS(15)": (10.0,   90.0),   # Stripper Level    %    — shut <10 or >90
}

# ── panel layout ──────────────────────────────────────────────────────────────
PANELS = [
    # (title, ylabel, [(col, label, color), ...], y_limits_or_None)
    (
        "Reactor Pressure",
        "kPa",
        [("XMEAS(7)", "Reactor P", "#e05c5c")],
        (None, None),
    ),
    (
        "Reactor Temperature",
        "°C",
        [("XMEAS(9)", "Reactor T", "#e08a3c")],
        (None, None),
    ),
    (
        "Reactor Level",
        "%",
        [("XMEAS(8)", "Reactor Lv", "#5c9ee0")],
        (0, 100),
    ),
    (
        "Sep & Stripper Levels",
        "%",
        [
            ("XMEAS(12)", "Sep Lv",      "#5cb85c"),
            ("XMEAS(15)", "Stripper Lv", "#9b59b6"),
        ],
        (0, 100),
    ),
    (
        "Recycle & Purge Flow",
        "kscmh / kscmh",
        [
            ("XMEAS(5)",  "Recycle Flow", "#1abc9c"),
            ("XMEAS(10)", "Purge Rate",   "#e74c3c"),
        ],
        (None, None),
    ),
    (
        "Purge Valve (MV)",
        "%",
        [("XMV(6)", "Purge Valve", "#e74c3c")],
        (0, 100),
    ),
    (
        "Sep & Stripper Underflow (MV)",
        "%",
        [
            ("XMV(7)",  "Sep Underflow",      "#3498db"),
            ("XMV(8)",  "Stripper Product",   "#2ecc71"),
        ],
        (0, 100),
    ),
    (
        "max |dx/dt|  (deriv norm)",
        "state units / h",
        [("deriv_norm", "deriv_norm", "#95a5a6")],
        (None, None),
    ),
]


def _add_thresholds(ax, col: str) -> None:
    if col not in ISD:
        return
    lo, hi = ISD[col]
    kw = dict(linestyle="--", linewidth=0.8, alpha=0.7)
    if hi is not None:
        ax.axhline(hi, color="#e05c5c", **kw, label=f"ISD >{hi}")
    if lo is not None:
        ax.axhline(lo, color="#e05c5c", **kw, label=f"ISD <{lo}")


def plot(csv_path: Path) -> None:
    print(f"Loading {csv_path} …")
    df = pd.read_csv(csv_path)
    print(f"  {len(df)} rows  |  t = {df['t_h'].min():.4f} … {df['t_h'].max():.4f} h")

    t = df["t_h"]
    n_panels = len(PANELS)
    ncols = 2
    nrows = (n_panels + 1) // ncols

    fig, axes = plt.subplots(nrows, ncols, figsize=(14, nrows * 3),
                             sharex=True, constrained_layout=True)
    fig.suptitle("Tennessee Eastman Process — Simulation Log", fontsize=13, fontweight="bold")

    flat_axes = axes.flatten()

    for idx, (title, ylabel, series, ylim) in enumerate(PANELS):
        ax = flat_axes[idx]
        for col, label, color in series:
            if col not in df.columns:
                continue
            ax.plot(t, df[col], label=label, color=color, linewidth=1.0)
            _add_thresholds(ax, col)

        ax.set_title(title, fontsize=9, fontweight="bold")
        ax.set_ylabel(ylabel, fontsize=8)
        ax.tick_params(labelsize=7)
        ax.xaxis.set_major_formatter(ticker.FormatStrFormatter("%.3f"))
        if ylim != (None, None):
            lo, hi = ylim
            current_lo, current_hi = ax.get_ylim()
            ax.set_ylim(lo if lo is not None else current_lo,
                        hi if hi is not None else current_hi)
        if len(series) > 1 or any(col in ISD for col, _, _ in series):
            ax.legend(fontsize=7, loc="upper right")
        ax.grid(True, linewidth=0.4, alpha=0.5)

    # hide any spare axes
    for idx in range(n_panels, len(flat_axes)):
        flat_axes[idx].set_visible(False)

    # shared x label on bottom row
    for ax in flat_axes[(nrows - 1) * ncols:]:
        ax.set_xlabel("Simulated time (h)", fontsize=8)

    out = csv_path.with_suffix(".png")
    fig.savefig(out, dpi=150)
    print(f"Saved → {out}")
    plt.show()


def main() -> None:
    parser = argparse.ArgumentParser(description="Plot TEP simulation_log.csv")
    parser.add_argument(
        "--csv",
        type=Path,
        default=_DEFAULT_CSV,
        help=f"Path to simulation_log.csv (default: {_DEFAULT_CSV})",
    )
    args = parser.parse_args()

    if not args.csv.exists():
        print(f"ERROR: CSV not found: {args.csv}", file=sys.stderr)
        sys.exit(1)

    plot(args.csv)


if __name__ == "__main__":
    main()
