Feature: Only the first parent of merge commit's are parsed for their Git commit messages.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                          | checkout_commit                          | from_commit_hash                         | from_version | expected_version |
      | https://github.com/dcyou/resume.git | 9015044aba82dbe8aa0119bffd7ea73cad171dd0 | fe14480df04f76e6434d45c762ab087df41b8473 | 1.2.2        | 1.3.1            |
