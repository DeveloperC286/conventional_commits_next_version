Feature: A repository using Conventional Commits can batch together the commits taking the biggest increment as the next semantic version by using the batch-commits flag.


Scenario Outline:
    Given I clone the repository "<repository>" and checkout the commit "<checkout_commit>".
    When I call conventional-commits-next-version with the from commit "<from_commit>" and version "<current_version>" and batch-commits flag.
    Then the returned version should be "<expected_version>".


Examples:
  |            repository              |              checkout_commit             |               from_commit                | current_version | expected_version |
  | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 |      0.2.1      |       0.2.2      |
  | https://github.com/yargs/yargs.git | 027a6365b737e13116811a8ef43670196e1fa00a | 1f26de809432be9cc6f4f185629f6e5d13236598 |      0.2.3      |       0.3.0      |
  | https://github.com/yargs/yargs.git | 18b0b752424bf560271e670ff95a0f90c8386787 | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 |      1.0.3      |       1.1.0      |
  | https://github.com/yargs/yargs.git | 2fed2a7c58de1d7c60858c0e8ff24421609e0dc4 | 6014e39bca3a1e8445aa0fb2a435f6181e344c45 |      1.0.3      |       1.0.3      |
  | https://github.com/yargs/yargs.git | b80bcf73533cedeb0118fc9b54a9899c89b98344 | 8aae3332251d09fa136db17ef4a40d83fa052bc4 |      1.1.3      |       1.1.4      |
  | https://github.com/yargs/yargs.git | 932cd1177e93f5cc99edfe57a4028e30717bf8fb | b7722a624424880971e15d5420047c390deb28ce |     13.2.5      |      13.3.0      |
  | https://github.com/yargs/yargs.git | 932cd1177e93f5cc99edfe57a4028e30717bf8fb | 6e4bc2aa0a6c5560928296f18e008a15b2aaf335 |     13.2.5      |      14.0.0      |
