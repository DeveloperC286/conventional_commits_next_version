Feature: A repository using Conventional Commits 1.0.0 can apply the commit messages in a consecutive order to determine the next Semantic Versioning 2.0.0, then compare the current version argument to ensure it is equal or larger than the determine Semantic Versioning.


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And the --current-version argument is set as "<current_version>".
    And conventional_commits_next_version is executed with the set arguments.
    Then the returncode should be "<expected_returncode>".


Examples:
    |            repository              |              checkout_commit             |               from_commit_hash               | from_version | current_version | expected_returncode |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd |   262e796329f4a09083ca07e49f926ae43ac850e9   |    0.2.1     |       0.2.1     |          1          |
    | https://github.com/yargs/yargs.git | 027a6365b737e13116811a8ef43670196e1fa00a |   1f26de809432be9cc6f4f185629f6e5d13236598   |    0.2.3     |       0.4.0     |          0          |
    | https://github.com/yargs/yargs.git | 18b0b752424bf560271e670ff95a0f90c8386787 |   ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6   |    1.0.3     |       1.1.0     |          1          |
    | https://github.com/yargs/yargs.git | 2fed2a7c58de1d7c60858c0e8ff24421609e0dc4 |   6014e39bca3a1e8445aa0fb2a435f6181e344c45   |    1.0.3     |       1.0.3     |          0          |
    | https://github.com/yargs/yargs.git | b9409da199ebca515a848489c206b807fab2e65d |   6e4bc2aa0a6c5560928296f18e008a15b2aaf335   |    1.1.3     |       2.0.1     |          0          |
    | https://github.com/yargs/yargs.git | 0aaa68bf36d35c697426de4dfe2e4e12128c2dc0 |   927810c7615912fb77a160273b2d6a946e9737b8   |    9.4.0     |      10.2.0     |          0          |
