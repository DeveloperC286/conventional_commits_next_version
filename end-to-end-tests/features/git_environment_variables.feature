Feature: Git environment variables are respected and used instead of using the current working directory.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --current-version is provided as "<current_version>".
    Then the current version assertion passes.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the current version assertion passes.


    Examples:
      | repository                                   | checkout_commit                          | commit_hash                              | from_version | current_version |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | 5afe20347dd3ae954b31707a67f381f87920797f | 4.1.2        | 4.2.7           |
      | https://gitlab.com/dmfay/massive-js.git      | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 2.0.1        | 3.1.0           |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --current-version is provided as "<current_version>".
    Then the current version assertion fails.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the current version assertion fails.


    Examples:
      | repository                                   | checkout_commit                          | commit_hash                              | from_version | current_version |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | 5afe20347dd3ae954b31707a67f381f87920797f | 4.1.2        | 4.1.3           |
      | https://gitlab.com/dmfay/massive-js.git      | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 2.0.1        | 2.1.0           |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                         | checkout_commit                          | reference | from_version | expected_version |
      | https://github.com/yargs/yargs.git | 95a4a0ac573cfe158e6e4bc8c8682ebd1644a198 | v16.0.3   | 16.0.3       | 16.0.5           |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                             | checkout_commit                          | reference        | from_version | expected_version |
      | https://github.com/Netflix/unleash.git | 238ce019c3a7b3302721fc1ae5b8ad2bdd50a706 | refs/tags/v2.0.0 | 2.0.0        | 2.0.1            |
      | https://github.com/Netflix/unleash.git | 238ce019c3a7b3302721fc1ae5b8ad2bdd50a706 | tags/v2.0.0      | 2.0.0        | 2.0.1            |
