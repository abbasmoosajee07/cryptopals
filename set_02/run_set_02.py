
#!/usr/bin/env python3
import os
from pathlib import Path
from challenge_utils import ChallengeBenchmarks

if __name__ == "__main__":

    script_dir = Path(__file__).parent.resolve()
    project_root = script_dir.parent
    os.chdir(project_root)
    selected_dir = script_dir

    config_file = "cryptopals_set_02.json"
    PROBLEMS_TO_RUN = list(range(9, 17))  # Problems 1-25

    analyzer = ChallengeBenchmarks(
        base_dir = selected_dir,
        config_file = config_file,
    )

    results = analyzer.analyze(
        problems_to_run= PROBLEMS_TO_RUN,  # Problems 1-25
        iterations=1,
        save_results=False,
        custom_dir= selected_dir / "analysis"
    )

    print("\nAnalysis complete!")
    print(results.head(25))