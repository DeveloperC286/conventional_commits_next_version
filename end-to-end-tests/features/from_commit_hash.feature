Feature: A Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD to calculate the next semantic version.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --current-version is provided as "<current_version>".
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


    Examples:
      | repository                                   | checkout_commit                          | commit_hash                              | from_version | current_version |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | 5afe20347dd3ae954b31707a67f381f87920797f | 4.1.2        | 4.1.3           |
      | https://gitlab.com/dmfay/massive-js.git      | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 2.0.1        | 2.1.0           |


  Scenario Outline: When you provide an invalid commit hash a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then their is a could not find commit hash "<commit_hash>" error.


    Examples:
      | repository                                    | checkout_commit                          | commit_hash                              | from_version |
      | https://github.com/SergioBenitez/Rocket.git   | 549c9241c41320fc5af76b53c2ffc3bd8db88f8c | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 | 1.0.4        |
      | https://github.com/electron/electron.git      | 8798571a77a4d2a7e073b046d2e8b56caa4d1e68 | a115eaa633856eb0b09f4019952f866e6b4ef96d | 0.2.12       |
      | https://gitlab.com/gitlab-org/release-cli.git | 451e0773944e47a4e2678c67691a69cf8934e76e | 1260e8a74de5c29c85ffd4e2b91632236ea55c3a | 3.2.0        |