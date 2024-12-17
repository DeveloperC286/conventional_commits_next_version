Feature: Specifies how commits are parsed, acceptable values are 'first' to parse only the first parent of merge commits, or 'all' to parse all parents. commit's are parsed for their Git commit messages.


  Scenario Outline: All the parents of merge commit's are parsed for their Git commit messages.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<default_expected_version>".
    When the argument --history-mode is provided as "all".
    Then the returned version should be "<all_parents_expected_version>".


    Examples:
      | repository                          | checkout_commit                          | commit_hash                              | from_version | default_expected_version | all_parents_expected_version |
      | https://github.com/dcyou/resume.git | 9015044aba82dbe8aa0119bffd7ea73cad171dd0 | fe14480df04f76e6434d45c762ab087df41b8473 | 1.2.2        | 1.3.1                    | 1.3.30                       |


  Scenario Outline: Only the first parent of merge commit's are parsed for their Git commit messages.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --history-mode is provided as "first".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                          | checkout_commit                          | commit_hash                              | from_version | expected_version |
      | https://github.com/dcyou/resume.git | 9015044aba82dbe8aa0119bffd7ea73cad171dd0 | fe14480df04f76e6434d45c762ab087df41b8473 | 1.2.2        | 1.3.1            |


  Scenario Outline: Only the first parent of merge commit's are parsed for their Git commit messages, by default.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                          | checkout_commit                          | commit_hash                              | from_version | expected_version |
      | https://github.com/dcyou/resume.git | 9015044aba82dbe8aa0119bffd7ea73cad171dd0 | fe14480df04f76e6434d45c762ab087df41b8473 | 1.2.2        | 1.3.1            |
