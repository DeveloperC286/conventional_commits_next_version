Feature: conventional_commits_next_version can operate with the Git environment variables and not just when the current working directory is a Git repository.


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the GIT_DIR environment variable is set to the cloned repository and the working directory is changed.
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And conventional_commits_next_version is executed with the set arguments.
    Then the returned version should be "<expected_version>".


Examples:
    |            repository              |              checkout_commit             |               from_commit_hash               | from_version | expected_version |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd |   262e796329f4a09083ca07e49f926ae43ac850e9   |    0.2.1     |       0.2.2      |
    | https://github.com/yargs/yargs.git | 027a6365b737e13116811a8ef43670196e1fa00a |   1f26de809432be9cc6f4f185629f6e5d13236598   |    0.2.3     |       0.3.0      |
    | https://github.com/yargs/yargs.git | 18b0b752424bf560271e670ff95a0f90c8386787 |   ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6   |    1.0.3     |       1.1.1      |
    | https://github.com/yargs/yargs.git | 2fed2a7c58de1d7c60858c0e8ff24421609e0dc4 |   6014e39bca3a1e8445aa0fb2a435f6181e344c45   |    1.0.3     |       1.0.3      |
    | https://github.com/yargs/yargs.git | b9409da199ebca515a848489c206b807fab2e65d |   6e4bc2aa0a6c5560928296f18e008a15b2aaf335   |    1.1.3     |       2.0.0      |
    | https://github.com/yargs/yargs.git | 0aaa68bf36d35c697426de4dfe2e4e12128c2dc0 |   927810c7615912fb77a160273b2d6a946e9737b8   |    9.4.0     |      10.0.0      |
    | https://github.com/yargs/yargs.git | 6014e39bca3a1e8445aa0fb2a435f6181e344c45 |   705384762919641fe9d4c0967452a292f5f52c6d   |     1.2.0    |       1.4.0      |


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the GIT_DIR environment variable is set to the cloned repository and the working directory is changed.
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And the --current-version argument is set as "<current_version>".
    And conventional_commits_next_version is executed with the set arguments.
    Then the exit code should be "<expected_exit_code>".


Examples:
    |            repository              |              checkout_commit             |               from_commit_hash               | from_version | current_version | expected_exit_code |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd |   262e796329f4a09083ca07e49f926ae43ac850e9   |    0.2.1     |       0.2.1     |          1         |
    | https://github.com/yargs/yargs.git | 027a6365b737e13116811a8ef43670196e1fa00a |   1f26de809432be9cc6f4f185629f6e5d13236598   |    0.2.3     |       0.4.0     |          0         |
    | https://github.com/yargs/yargs.git | 18b0b752424bf560271e670ff95a0f90c8386787 |   ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6   |    1.0.3     |       1.1.0     |          1         |
    | https://github.com/yargs/yargs.git | 2fed2a7c58de1d7c60858c0e8ff24421609e0dc4 |   6014e39bca3a1e8445aa0fb2a435f6181e344c45   |    1.0.3     |       1.0.3     |          0         |
    | https://github.com/yargs/yargs.git | b9409da199ebca515a848489c206b807fab2e65d |   6e4bc2aa0a6c5560928296f18e008a15b2aaf335   |    1.1.3     |       2.0.1     |          0         |
    | https://github.com/yargs/yargs.git | 0aaa68bf36d35c697426de4dfe2e4e12128c2dc0 |   927810c7615912fb77a160273b2d6a946e9737b8   |    9.4.0     |      10.2.0     |          0         |
    | https://github.com/yargs/yargs.git | 3af7f04cdbfcbd4b3f432aca5144d43f21958c39 |   705384762919641fe9d4c0967452a292f5f52c6d   |    3.0.0     |       3.0.0     |          1         |


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the GIT_DIR environment variable is set to the cloned repository and the working directory is changed.
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And the --batch-commits flag is set.
    And conventional_commits_next_version is executed with the set arguments.
    Then the returned version should be "<expected_version>".


Examples:
    |            repository              |              checkout_commit             |               from_commit_hash                | from_version | expected_version |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd |   262e796329f4a09083ca07e49f926ae43ac850e9    |     0.2.1    |       0.2.2      |
    | https://github.com/yargs/yargs.git | 027a6365b737e13116811a8ef43670196e1fa00a |   1f26de809432be9cc6f4f185629f6e5d13236598    |     0.2.3    |       0.3.0      |
    | https://github.com/yargs/yargs.git | 18b0b752424bf560271e670ff95a0f90c8386787 |   ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6    |     1.0.3    |       1.1.0      |
    | https://github.com/yargs/yargs.git | 2fed2a7c58de1d7c60858c0e8ff24421609e0dc4 |   6014e39bca3a1e8445aa0fb2a435f6181e344c45    |     1.0.3    |       1.0.3      |
    | https://github.com/yargs/yargs.git | b80bcf73533cedeb0118fc9b54a9899c89b98344 |   8aae3332251d09fa136db17ef4a40d83fa052bc4    |     1.1.3    |       1.1.4      |
    | https://github.com/yargs/yargs.git | 932cd1177e93f5cc99edfe57a4028e30717bf8fb |   b7722a624424880971e15d5420047c390deb28ce    |    13.2.5    |      13.3.0      |
    | https://github.com/yargs/yargs.git | 932cd1177e93f5cc99edfe57a4028e30717bf8fb |   6e4bc2aa0a6c5560928296f18e008a15b2aaf335    |    13.2.5    |      14.0.0      |
    | https://github.com/yargs/yargs.git | 3af7f04cdbfcbd4b3f432aca5144d43f21958c39 |   c36c571e4e15dfe26be1d919e4991fb6ab6ed9fd    |     1.2.0    |       1.3.0      |


Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the GIT_DIR environment variable is set to the cloned repository and the working directory is changed.
    When the --from-commit-hash and --from-version arguments are set as "<from_commit_hash>" and "<from_version>".
    And the --current-version argument is set as "<current_version>".
    And the --batch-commits flag is set.
    And conventional_commits_next_version is executed with the set arguments.
    Then the exit code should be "<expected_exit_code>".


Examples:
    |            repository              |              checkout_commit             |               from_commit_hash               | from_version | current_version | expected_exit_code |
    | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd |   262e796329f4a09083ca07e49f926ae43ac850e9   |    0.2.1     |       0.2.1     |          1         |
    | https://github.com/yargs/yargs.git | 027a6365b737e13116811a8ef43670196e1fa00a |   1f26de809432be9cc6f4f185629f6e5d13236598   |    0.2.3     |       0.3.0     |          0         |
    | https://github.com/yargs/yargs.git | 18b0b752424bf560271e670ff95a0f90c8386787 |   ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6   |    1.0.3     |       1.1.0     |          0         |
    | https://github.com/yargs/yargs.git | 2fed2a7c58de1d7c60858c0e8ff24421609e0dc4 |   6014e39bca3a1e8445aa0fb2a435f6181e344c45   |    1.0.3     |       1.0.3     |          0         |
    | https://github.com/yargs/yargs.git | b9409da199ebca515a848489c206b807fab2e65d |   6e4bc2aa0a6c5560928296f18e008a15b2aaf335   |    1.1.3     |       1.1.4     |          1         |
    | https://github.com/yargs/yargs.git | 0aaa68bf36d35c697426de4dfe2e4e12128c2dc0 |   927810c7615912fb77a160273b2d6a946e9737b8   |    9.4.0     |       9.5.0     |          1         |
    | https://github.com/yargs/yargs.git | 3af7f04cdbfcbd4b3f432aca5144d43f21958c39 |   c36c571e4e15dfe26be1d919e4991fb6ab6ed9fd   |    3.0.0     |       3.1.0     |          0         |
