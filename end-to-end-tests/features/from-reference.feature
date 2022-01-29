Feature: A Git reference can be used to indicate where to start the calculations from.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                         | checkout_commit                          | from_reference | from_version | expected_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | v16.0.3        | 16.0.3       | 16.0.5           |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                             | checkout_commit                          | from_reference    | from_version | expected_version |
	  | https://github.com/Netflix/unleash.git | 238ce019c3a7b3302721fc1ae5b8ad2bdd50a706 | refs/tags/v2.0.0 | 2.0.0 | 2.0.1 |
	  | https://github.com/Netflix/unleash.git | 238ce019c3a7b3302721fc1ae5b8ad2bdd50a706 | tags/v2.0.0 | 2.0.0 | 2.0.1 |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-reference is provided as "<from_reference>".
    Then the error message is "ERROR conventional_commits_next_version::model::commits > Could not find a reference with the name ".


    Examples:
      | repository                         | checkout_commit                          | from_reference | from_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | 16-0-3         | 16.0.3       |
