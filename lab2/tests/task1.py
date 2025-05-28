#!/usr/bin/env python3

import pytest
import subprocess
import shlex


@pytest.mark.parametrize("args, expected_stdout_file, expected_stderr",
                         [
                             ("tests/data/test_file.txt", "tests/data/test_single_file_no_number.txt", ""),
                             ("+2 tests/data/test_file.txt", "tests/data/test_single_file_2_lines.txt", ""),
                             ("+2 tests/data/test_file.txt tests/data/test_file.txt", "tests/data/test_two_files.txt", ""),
                             ("+0 tests/data/test_file.txt", "tests/data/test_empty.txt", ""),
                             ("+300 tests/data/test_file.txt", "tests/data/test_file.txt", ""),
                             ("+300 tests/data/test_empty.txt", "tests/data/test_empty.txt", ""),
                         ])
def test_task1(args, expected_stdout_file, expected_stderr):
    process = subprocess.run(
        [
            "refal/refgo",
            "task1.rsl",
        ]+shlex.split(args),
        capture_output=True,
        text=True,
    )
    with open(expected_stdout_file) as f: expected_stdout = f.read()

    output = process.stdout, process.stderr

    assert output == (expected_stdout, expected_stderr)
