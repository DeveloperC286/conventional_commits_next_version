Feature: The singular Git commit message to use in the calculation of the next semantic version.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-message is provided as "<from_commit_message>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                         | checkout_commit                          | from_commit_message                     | from_version | expected_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | "fix: removing GC mem leak"             | 16.0.3       | 16.0.4           |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | "feat: new from arg added"              | 16.0.3       | 16.1.0           |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | "feat!: changing from tag to reference" | 16.0.3       | 17.0.0           |
