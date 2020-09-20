Feature: The commit message's increments are batched together and the largest increment determines the next calculated Semantic Versioning.


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And the --batch-commits flag is set.
    And conventional_commits_next_version is executed with the set arguments.
    Then the returned version should be "<expected_version>".


Examples:
    | repository | checkout_commit | from_commit_hash | from_version | expected_version |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1 | 0.2.2 |
