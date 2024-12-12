Feature: The increments are batched together and the largest increment determines the next calculated semantic version.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --calculation-mode is provided as "batch".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                                | checkout_commit                          | from_commit_hash                         | from_version | expected_version |
      | https://github.com/yargs/yargs.git        | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1        | 0.2.2            |
      | https://github.com/yargs/yargs.git        | 027a6365b737e13116811a8ef43670196e1fa00a | 1f26de809432be9cc6f4f185629f6e5d13236598 | 0.2.3        | 0.3.0            |
      | https://github.com/yargs/yargs.git        | 18b0b752424bf560271e670ff95a0f90c8386787 | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 | 1.0.3        | 1.1.0            |
      | https://github.com/yargs/yargs.git        | 2fed2a7c58de1d7c60858c0e8ff24421609e0dc4 | 6014e39bca3a1e8445aa0fb2a435f6181e344c45 | 1.0.3        | 1.0.3            |
      | https://github.com/yargs/yargs.git        | b80bcf73533cedeb0118fc9b54a9899c89b98344 | 8aae3332251d09fa136db17ef4a40d83fa052bc4 | 1.1.3        | 1.1.4            |
      | https://github.com/yargs/yargs.git        | 932cd1177e93f5cc99edfe57a4028e30717bf8fb | b7722a624424880971e15d5420047c390deb28ce | 13.2.5       | 13.3.0           |
      | https://github.com/yargs/yargs.git        | 932cd1177e93f5cc99edfe57a4028e30717bf8fb | 6e4bc2aa0a6c5560928296f18e008a15b2aaf335 | 13.2.5       | 14.0.0           |
      | https://github.com/yargs/yargs.git        | 3af7f04cdbfcbd4b3f432aca5144d43f21958c39 | c36c571e4e15dfe26be1d919e4991fb6ab6ed9fd | 1.2.0        | 1.3.0            |
      | https://github.com/danielduarte/diffparse | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c | 1.0.7        | 2.0.0            |
