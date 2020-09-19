Feature: The current version argument supplied is asserted to be equal or greater than the calculated next Semantic Versioning.


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And the --current-version argument is set as "<current_version>".
    And the --batch-commits flag is set.
    And conventional_commits_next_version is executed with the set arguments.
    Then the exit code should be "<expected_exit_code>".

Examples:
    | repository | checkout_commit | from_commit_hash | from_version | current_version | expected_exit_code |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1 | 0.2.1 | 1 |


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And the --current-version argument is set as "<current_version>".
    And conventional_commits_next_version is executed with the set arguments.
    Then the exit code should be "<expected_exit_code>".


Examples:
    | repository | checkout_commit | from_commit_hash | from_version | current_version | expected_exit_code |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1 | 0.2.1 | 1 |
