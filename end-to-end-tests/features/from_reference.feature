Feature: A Git reference can be provided as an argument to indicate where to start taking the range of commits from till HEAD to calculate the next semantic version.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                         | checkout_commit                          | reference | from_version | expected_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | v16.0.3   | 16.0.3       | 16.0.5           |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<full_reference>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".
    Given the arguments are reset.
    When linting from the "<partial_reference>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".
    Given the arguments are reset.
    When linting from the "<short_reference>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                             | checkout_commit                          | full_reference   | from_version | expected_version | partial_reference | short_reference |
      | https://github.com/Netflix/unleash.git | 238ce019c3a7b3302721fc1ae5b8ad2bdd50a706 | refs/tags/v2.0.0 | 2.0.0        | 2.0.1            | tags/v2.0.0       | v2.0.0          |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    And the argument --from-version is provided as "<from_version>".
    Then their is a could not find reference "<reference>" error.


    Examples:
      | repository                         | checkout_commit                          | reference | from_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | 16-0-3    | 16.0.3       |
