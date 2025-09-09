from pathlib import Path
from challenge_utils import ScriptBuilder

# Constants
PROBLEM_NO = 9
CHALLENGE = 2
CHOSEN_LANGUAGE = "rust"

AUTHOR = "Abbas Moosajee"

CONFIG_DICT = {
    2 : ("set_02", "cryptopals_set_02.json"),
    # 3 : ("set_03", "cryptopals_set_03.json"),
    # 4 : ("set_04", "cryptopals_set_04.json"),
    # 5 : ("set_05", "cryptopals_set_05.json"),
    # 6 : ("set_06", "cryptopals_set_06.json"),
    # 7 : ("set_07", "cryptopals_set_07.json"),
    # 8 : ("set_08", "cryptopals_set_08.json"),
    }

def main() -> None:
    """Main function to create challenge files."""

    repo_dir = Path(__file__).parent
    folder, config_file = CONFIG_DICT[CHALLENGE]
    challenge_dir = repo_dir / folder

    try:
        builder = ScriptBuilder(AUTHOR, challenge_dir, config_file)

        filepath = builder.create_files(
            prob_no=PROBLEM_NO,
            language=CHOSEN_LANGUAGE,
            txt_files=1,
        )

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()