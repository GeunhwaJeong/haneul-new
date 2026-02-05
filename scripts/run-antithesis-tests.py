#!/usr/bin/env python3
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

"""
Script to kick off Antithesis tests via the haneul-operations repo.

Requires haneul-operations repo to be locally checked out. If HANEUL_OPS_REPO
environment variable is not set, it defaults to ~/dev/haneul-operations.
"""

import argparse
import json
import os
import subprocess
import sys
import time


def get_git_output(args, cwd=None):
    """Run a git command and return its output."""
    result = subprocess.run(
        ["git"] + args,
        capture_output=True,
        text=True,
        cwd=cwd,
    )
    return result.stdout.strip()


def is_git_dirty(cwd=None):
    """Check if the git repo has uncommitted changes."""
    result1 = subprocess.run(
        ["git", "diff", "--quiet", "--exit-code"],
        cwd=cwd,
    )
    result2 = subprocess.run(
        ["git", "diff", "--cached", "--quiet", "--exit-code"],
        cwd=cwd,
    )
    return result1.returncode != 0 or result2.returncode != 0


def format_cmd_for_output(cmd, description):
    """Format command for output, matching bash script's quoting behavior."""
    parts = []
    for part in cmd:
        if description and part == f"description={description}":
            parts.append(f"description='{description}'")
        else:
            parts.append(part)
    return " ".join(parts)


def main():
    parser = argparse.ArgumentParser(
        description="Run Antithesis tests via haneul-operations workflow",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )

    parser.add_argument(
        "-t", "--test-duration",
        type=float,
        default=0.25,
        help="Antithesis test duration in hours (default: 0.25)",
    )
    parser.add_argument(
        "-s", "--split-version",
        action="store_true",
        help="Split version mode (uses merge-base as primary commit)",
    )
    parser.add_argument(
        "-d", "--description",
        type=str,
        help="Description of the test run",
    )
    parser.add_argument(
        "-u", "--upgrade",
        action="store_true",
        help="Upgrade test type",
    )
    parser.add_argument(
        "-a", "--alt-commit",
        type=str,
        help="Additional sha for split-cluster upgrade test",
    )
    parser.add_argument(
        "-l", "--log-level",
        type=str,
        help="Logging filter for haneul-node (no spaces allowed)",
    )
    parser.add_argument(
        "-T", "--tidehunter-commit",
        type=str,
        help="Tidehunter repo sha (if unset, rocksdb will be used)",
    )
    parser.add_argument(
        "-c", "--cli-commit",
        type=str,
        help="Sha for cli image",
    )
    parser.add_argument(
        "-S", "--stress-commit",
        type=str,
        help="Haneul repo sha for stress image",
    )
    parser.add_argument(
        "-p", "--protocol-override",
        type=str,
        help="Protocol config override (none, testnet, mainnet)",
    )
    parser.add_argument(
        "--test-name",
        type=str,
        help="Name to group test history",
    )
    parser.add_argument(
        "-C", "--haneul-commit",
        type=str,
        help="Haneul repo sha (default: current HEAD)",
    )
    parser.add_argument(
        "-r", "--workflow-ref",
        type=str,
        help="Branch/ref in haneul-operations repo to run workflow from",
    )
    parser.add_argument(
        "-n", "--dry-run",
        action="store_true",
        help="Print the command that would be executed without running it",
    )

    args = parser.parse_args()

    # Determine haneul-operations repo path
    haneul_ops_repo = os.environ.get("HANEUL_OPS_REPO", os.path.expanduser("~/dev/haneul-operations"))

    if not os.path.isdir(haneul_ops_repo):
        print(
            f"haneul-operations repo directory {haneul_ops_repo} does not exist. "
            "Please make sure you have correctly set the haneul-operations repo directory.",
            file=sys.stderr,
        )
        return 1

    # Check if git repo is dirty
    if is_git_dirty():
        print("Warning: git repo is dirty")

    # Validate log level (no spaces)
    if args.log_level and " " in args.log_level:
        print("Error: LOG_LEVEL cannot contain spaces", file=sys.stderr)
        return 1

    # Determine haneul_commit
    haneul_commit = args.haneul_commit
    if not haneul_commit:
        haneul_commit = get_git_output(["rev-parse", "HEAD"])

    # Determine commit and alt_commit based on split_version mode
    if args.split_version:
        if args.alt_commit:
            commit = args.alt_commit
        else:
            commit = get_git_output(["merge-base", "origin/main", "HEAD"])
        alt_commit = haneul_commit
    else:
        commit = haneul_commit
        alt_commit = args.alt_commit

    # Build the gh workflow run command
    cmd = ["gh", "workflow", "run"]

    if args.workflow_ref:
        cmd.extend(["-r", args.workflow_ref])

    cmd.append(".github/workflows/run-antithesis-tests.yaml")
    cmd.extend(["-f", f"haneul_commit={commit}"])
    cmd.extend(["-f", f"test_duration={args.test_duration}"])

    if args.description:
        cmd.extend(["-f", f"description={args.description}"])

    if alt_commit:
        cmd.extend(["-f", f"haneul_commit_alt={alt_commit}"])

    if args.upgrade:
        cmd.extend(["-f", "test_type=upgrade"])

    if args.log_level:
        cmd.extend(["-f", f"rust_log_filter={args.log_level}"])

    if args.tidehunter_commit:
        cmd.extend(["-f", f"tidehunter_commit={args.tidehunter_commit}"])

    if args.cli_commit:
        cmd.extend(["-f", f"haneul_cli_commit={args.cli_commit}"])

    if args.stress_commit:
        cmd.extend(["-f", f"stress_commit={args.stress_commit}"])

    if args.protocol_override:
        cmd.extend(["-f", f"protocol_config_override={args.protocol_override}"])

    if args.test_name:
        cmd.extend(["-f", f"test_name={args.test_name}"])

    # Print the command (format to match bash script's output)
    cmd_str = format_cmd_for_output(cmd, args.description)
    print(f"Running: {cmd_str}")

    if args.dry_run:
        return 0

    # Execute the workflow
    result = subprocess.run(cmd, cwd=haneul_ops_repo)
    if result.returncode != 0:
        return result.returncode

    # Get the GitHub user
    gh_user_result = subprocess.run(
        ["gh", "api", "user"],
        capture_output=True,
        text=True,
        cwd=haneul_ops_repo,
    )
    gh_user = json.loads(gh_user_result.stdout).get("login")

    # Wait for the run to be created
    time.sleep(5)

    # Get the run ID
    run_list_result = subprocess.run(
        [
            "gh", "run", "list",
            "--user", gh_user,
            "--workflow", ".github/workflows/run-antithesis-tests.yaml",
            "--limit", "1",
            "--json", "databaseId",
            "-q", ".[0].databaseId",
        ],
        capture_output=True,
        text=True,
        cwd=haneul_ops_repo,
    )
    run_id = run_list_result.stdout.strip()
    print(f"Run ID: {run_id}")

    # Get the run URL
    run_view_result = subprocess.run(
        [
            "gh", "run", "view", run_id,
            "--json", "url",
            "-q", ".url",
        ],
        capture_output=True,
        text=True,
        cwd=haneul_ops_repo,
    )
    url = run_view_result.stdout.strip()
    print(f"URL: {url}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
