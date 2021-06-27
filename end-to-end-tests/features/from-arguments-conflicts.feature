Feature: The from arguments conflict with one another and can not be provided at the same time.


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the error message is either "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments" or "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments".


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | from_reference | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0        | 1.0.0        |


  Scenario Outline: You can not provide both a reference and a commit message.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-message is provided as "<from_commit_message>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the error message is either "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments" or "error: The argument '--from-commit-message <from-commit-message>' cannot be used with one or more of the other specified arguments".


    Examples:
      | repository                         | checkout_commit                          | from_commit_message | from_reference | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | "feat: temp 13"     | v15.4.0        | 1.0.0        |


  Scenario Outline: You can not provide both a commit message and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-message is provided as "<from_commit_message>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the error message is either "error: The argument '--from-commit-message <from-commit-message>' cannot be used with one or more of the other specified arguments" or "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments".


    Examples:
      | repository                         | checkout_commit                          | from_commit_message | from_commit_hash                         | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | "feat: temp 13"     | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | 1.0.0        |


  Scenario Outline: You must provide one of the from arguments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    Then the error message is "error: The following required arguments were not provided:".


    Examples:
      | repository                         | checkout_commit                          | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 1.0.0        |