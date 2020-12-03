Feature: The input argument reference can be used instead of the commit hash.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                         | checkout_commit                          | from_reference | from_version | expected_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | v16.0.3        | 16.0.3       | 16.0.5           |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                             | checkout_commit                          | from_reference    | from_version | expected_version |
      | https://github.com/tinacms/tinacms.git | e4bb0aac1e16e9423ec9caac701077a732bcd98e | refs/tags/v0.34.0 | 0.34.0       | 0.35.0           |
      | https://github.com/tinacms/tinacms.git | e4bb0aac1e16e9423ec9caac701077a732bcd98e | tags/v0.34.0      | 0.34.0       | 0.35.0           |


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the error message is either "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments" or "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments".


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | from_reference | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0        | 1.0.0        |


  Scenario Outline: You must provide either a reference or a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-version is provided as "<from_version>".
    Then the error message is "error: The following required arguments were not provided:".


    Examples:
      | repository                         | checkout_commit                          | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 1.0.0        |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the error message is "ERROR conventional_commits_next_version::git > Could not find a reference with the name ".


    Examples:
      | repository                         | checkout_commit                          | from_reference | from_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | 16-0-3         | 16.0.3       |
