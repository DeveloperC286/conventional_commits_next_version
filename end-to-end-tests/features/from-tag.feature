Feature: The argument --from-tag can be used instead of --from-commit-hash.


Scenario Outline: You can not provide both --from-tag and --from-commit-hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-tag is provided as "<from_tag>".
    Then the error message is either "error: The argument '--from-tag <from-tag>' cannot be used with one or more of the other specified arguments" or "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments".


Examples:
    | repository                         | checkout_commit                          | from_commit_hash                         | from_tag | from_version |
    | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0  | 1.0.0 |


Scenario Outline: You must provide either --from-tag or --from-commit-hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    Then the error message is "error: The following required arguments were not provided:".


Examples:
    | repository                                    | checkout_commit                          | from_version |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 1.0.0 |


Scenario Outline: You can use the --from-tag argument instead of the --from-commit-hash argument.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-tag is provided as "<from_tag>".
    And the argument --current-version is provided as "<current_version>".
    Then the conventional_commits_next_version assertion passes.

Examples:
    | repository                                 | checkout_commit                          | from_tag | from_version | current_version |
    | https://github.com/yargs/yargs.git         | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | v16.0.3  | 16.0.3       | 16.0.5 |


Scenario Outline: When you provide an invalid tag a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-tag is provided as "<from_tag>".
    And the argument --current-version is provided as "<current_version>".
    Then the error message is "ERROR conventional_commits_next_version::git > Could not find a tag with the name '16-0-3'.".

Examples:
    | repository                                 | checkout_commit                          | from_tag | from_version | current_version |
    | https://github.com/yargs/yargs.git         | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | 16-0-3   | 16.0.3       | 16.0.5 |
