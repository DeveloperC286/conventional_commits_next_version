Feature: A commit message can be provided by standard input rather than from a range of commits from Git.


  Scenario Outline:
    Given the context and environment are reset.
    When the argument --from-version is provided as "<from_version>".
    When linting the "<commit_message>".
    Then the returned version should be "<expected_version>".


    Examples:
      | from_version | commit_message                          | expected_version |
      | 16.0.3       | "fix: removing GC mem leak"             | 16.0.4           |
      | 0.0.3        | "feat: new from arg added"              | 0.1.0            |
      | 1.2.0        | "feat!: changing from tag to reference" | 2.0.0            |
