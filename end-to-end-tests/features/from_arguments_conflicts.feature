Feature: The from arguments conflict with one another and can not be provided at the same time.


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | from_reference | from_commit_hash                         | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | 1.0.0        |


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-reference is provided as "<from_reference>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | from_reference | from_commit_hash                         | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | 1.0.0        |


  Scenario Outline: You can not provide both a reference and a commit message.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the flag --from-stdin is set and the standard input is "<standard_input>".
    And the argument --from-reference is provided as "<from_reference>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | from_reference | standard_input  | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        | "feat: temp 13" | 1.0.0        |


  Scenario Outline: You can not provide both a reference and a commit message.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    And the flag --from-stdin is set and the standard input is "<standard_input>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | from_reference | standard_input  | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        | "feat: temp 13" | 1.0.0        |


  Scenario Outline: You can not provide both a commit message and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the flag --from-stdin is set and the standard input is "<standard_input>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | standard_input  | from_commit_hash                         | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | "feat: temp 13" | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | 1.0.0        |


  Scenario Outline: You can not provide both a commit message and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the flag --from-stdin is set and the standard input is "<standard_input>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | standard_input  | from_commit_hash                         | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | "feat: temp 13" | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | 1.0.0        |
