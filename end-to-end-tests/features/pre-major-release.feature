Feature: Breaking changes for pre-major release semantic versions only increments the minor expected version.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<post_major_release_from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the --batch-commits flag is set.
    Then the returned version should be "<post_major_release_expected_version>".
    Given the arguments are reset.
    When the argument --from-version is provided as "<pre_major_release_from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the --batch-commits flag is set.
    Then the returned version should be "<pre_major_release_expected_version>".

    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | post_major_release_from_version | post_major_release_expected_version | pre_major_release_from_version | pre_major_release_expected_version |
      | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 1.3.0                           | 1.3.1                               | 0.2.1                          | 0.2.2                              |
      | https://github.com/yargs/yargs.git | b6286d7269f3b20fd00da6e7326d3852a162deea | 395bb67749787d269cabe80ffc3133c2f6958aeb | 1.0.7                           | 2.0.0                               | 0.2.1                          | 0.3.0                              |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<post_major_release_from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the returned version should be "<post_major_release_expected_version>".
    Given the arguments are reset.
    When the argument --from-version is provided as "<pre_major_release_from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the returned version should be "<pre_major_release_expected_version>".

    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | post_major_release_from_version | post_major_release_expected_version | pre_major_release_from_version | pre_major_release_expected_version |
      | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 1.3.0                           | 1.3.1                               | 0.2.1                          | 0.2.2                              |
      | https://github.com/yargs/yargs.git | b6286d7269f3b20fd00da6e7326d3852a162deea | 395bb67749787d269cabe80ffc3133c2f6958aeb | 1.0.7                           | 2.0.2                               | 0.2.1                          | 0.3.2                              |

