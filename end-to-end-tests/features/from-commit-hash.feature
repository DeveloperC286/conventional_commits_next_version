Feature: A Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD to lint.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --current-version is provided as "<current_version>".
    Then the current version assertion passes.


    Examples:
      | repository                                   | checkout_commit                          | from_commit_hash                         | from_version | current_version |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | 5afe20347dd3ae954b31707a67f381f87920797f | 4.1.2        | 4.2.7           |
      | https://gitlab.com/dmfay/massive-js.git      | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 2.0.1        | 3.1.0           |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --current-version is provided as "<current_version>".
    Then the current version assertion fails.


    Examples:
      | repository                                   | checkout_commit                          | from_commit_hash                         | from_version | current_version |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | 5afe20347dd3ae954b31707a67f381f87920797f | 4.1.2        | 4.1.3           |
      | https://gitlab.com/dmfay/massive-js.git      | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 2.0.1        | 2.1.0           |


  Scenario Outline: When you provide an invalid commit hash a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a could not find commit hash "<from_commit_hash>" error.


    Examples:
      | repository                                    | checkout_commit                          | from_version | from_commit_hash                         |
      | https://github.com/SergioBenitez/Rocket.git   | 549c9241c41320fc5af76b53c2ffc3bd8db88f8c | 1.0.4        | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 |
      | https://github.com/electron/electron.git      | 8798571a77a4d2a7e073b046d2e8b56caa4d1e68 | 0.2.12       | a115eaa633856eb0b09f4019952f866e6b4ef96d |
      | https://gitlab.com/gitlab-org/release-cli.git | 451e0773944e47a4e2678c67691a69cf8934e76e | 3.2.0        | 1260e8a74de5c29c85ffd4e2b91632236ea55c3a |