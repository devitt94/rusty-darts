import argparse
import csv

from rusty_darts import (
    BULLSEYE,
    TREBLE_20,
    TREBLE_19,
    TREBLE_18,
    TREBLE_17,
    TREBLE_16,
    TREBLE_15,
    TREBLE_14,
    simulate,
)


if __name__ == '__main__':
    
    argparse = argparse.ArgumentParser(
        description="Simulate darts throws at different aim points and dispersions"
    )
    argparse.add_argument(
        "--n-sims",
        type=int,
        default=200000,
        help="Number of simulations to run for each aim point and dispersion",
    )
    argparse.add_argument(
        "--min-dispersion",
        nargs="+",
        type=float,
        default=2.5,
        help="Values of dispersion to simulate",
    )
    argparse.add_argument(
        "--max-dispersion",
        nargs="+",
        type=float,
        default=50.0,
        help="Values of dispersion to simulate",
    )
    argparse.add_argument(
        "--dispersion-step",
        nargs="+",
        type=float,
        default=2.5,
        help="Values of dispersion to simulate",
    )
    argparse.add_argument(
        "--results-file",
        type=str,
        default="results.csv",
        help="File to write results to",
    )
    args = argparse.parse_args()

    AIM_POINTS = {
        "bullseye": BULLSEYE,
        "treble_20": TREBLE_20,
        "treble_19": TREBLE_19,
        "treble_18": TREBLE_18,
        "treble_17": TREBLE_17,
        "treble_16": TREBLE_16,
        "treble_15": TREBLE_15,
        "treble_14": TREBLE_14,
    }


    writer = csv.writer(open(args.results_file, "w"))
    writer.writerow(["aim_point", "dispersion", "average_score", "std_dev"])
    for name, aim_point in AIM_POINTS.items():
        dispersion = args.min_dispersion
        while dispersion <= args.max_dispersion:
            print(f"Simulating {args.n_sims} throws at {name} with dispersion {dispersion}mm")
            result = simulate(args.n_sims, dispersion, aim_point)
            writer.writerow([name, dispersion, result.average_score, result.std_dev])
            print(f"Result: {result}")

            dispersion += args.dispersion_step

