import os
import subprocess
from argparse import ArgumentParser
from logging import INFO, Formatter, Logger, StreamHandler, getLogger
from multiprocessing import Pool
from pathlib import Path

VIS_CMD = ["cargo", "run", "--release", "--bin", "vis"]


def setup_logger(name: str) -> Logger:
    logger = getLogger(name)
    logger.setLevel(INFO)

    ch = StreamHandler()
    ch.setLevel(INFO)
    ch_formatter = Formatter(
        "%(asctime)s - %(name)s - %(levelname)s - %(message)s", "%H:%M:%S"
    )
    ch.setFormatter(ch_formatter)

    logger.addHandler(ch)
    return logger


def run_test(exe_in_out_file: tuple[Path, Path, Path]) -> None:
    exe_file, in_file, out_file = exe_in_out_file
    out_file.touch()
    res = subprocess.run(exe_file.as_posix(), stdin=in_file.open(), capture_output=True)
    out_file.write_bytes(res.stdout)


def eval_result(tool_inout_file: tuple[Path, Path, Path]) -> int:
    tool_path, in_file, out_file = tool_inout_file
    cmd = VIS_CMD + [in_file.as_posix(), out_file.as_posix()]
    res = subprocess.run(cmd, capture_output=True, cwd=tool_path)
    return int(res.stdout.decode("utf-8").split()[-1])


if __name__ == "__main__":
    arg_parser = ArgumentParser()
    arg_parser.add_argument("exe", metavar="X", help="Executable")
    arg_parser.add_argument("tool_dir", metavar="TOOL", help="Path to local tester directory")
    arg_parser.add_argument("in_dir", metavar="IN", help="Path to input directory")
    arg_parser.add_argument("out_dir", metavar="OUT", help="Path to output directory")
    arg_parser.add_argument("--verbose", action="store_true", help="Show detailed test result")
    args = arg_parser.parse_args()

    lg = setup_logger("runner")

    exe_path = Path(args.exe)
    tool_path = Path(args.tool_dir).resolve()

    in_dir = Path(args.in_dir)
    out_dir = Path(args.out_dir)
    if not out_dir.is_dir():
        out_dir.mkdir()

    in_files = sorted(in_dir.iterdir())
    in_out_files = [(in_file, out_dir / in_file.name) for in_file in in_files]
    exe_in_out_files = [(exe_path, in_file, out_file) for in_file, out_file in in_out_files]
    tool_in_out_files = [(tool_path, in_file, out_file) for in_file, out_file in in_out_files]
    n = len(in_files)

    # Limit the number of processes to max(cpu_count - CPU_RESERVE, 1),
    # to make room for processes which have nothing to do with this test.
    # If this approach is not taken, test scores will be unexpectedly affected.
    CPU_RESERVE = 4
    cpu_count = os.cpu_count()
    assert cpu_count is not None
    test_processes_count = max(cpu_count - CPU_RESERVE, 1)
    pool = Pool(processes=test_processes_count)

    lg.info(f"#test_cases = {n}")
    lg.info(f"Running tests... (#processes = {test_processes_count})")
    pool.map(run_test, exe_in_out_files)
    lg.info("Evaluating results...")
    scores = pool.map(eval_result, tool_in_out_files)

    if args.verbose:
        for score, (in_file, out_file) in zip(scores, in_out_files):
            lg.info(f"`{in_file}` => `{out_file}`: score = {score}")
    total = sum(scores)

    max_score, (max_in, max_out) = max(zip(scores, in_out_files))
    min_score, (min_in, min_out) = min(zip(scores, in_out_files))
    lg.info("----- TEST SUMMARY -----")
    lg.info(f"SUM = {total}")
    lg.info(f"AVG = {total / n}")
    lg.info(f"MAX = {max_score} (`{max_in}` => `{max_out}`)")
    lg.info(f"MIN = {min_score} (`{min_in}` => `{min_out}`)")
