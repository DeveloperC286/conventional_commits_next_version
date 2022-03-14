Feature: A shortened Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD, instead of a full Git commit hash.


  Scenario Outline: A shortened and full Git commit hash can be used interchangeably.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --calculation-mode is provided as "Batch".
    Then the returned version should be "<expected_version>".
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<shortened_from_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --calculation-mode is provided as "Batch".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                                 | checkout_commit                          | from_commit_hash                         | from_version | expected_version | shortened_from_commit_hash |
      | https://github.com/yargs/yargs.git         | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1        | 0.2.2            | 262e796                    |
      | https://github.com/yargs/yargs.git         | 027a6365b737e13116811a8ef43670196e1fa00a | 1f26de809432be9cc6f4f185629f6e5d13236598 | 0.2.3        | 0.3.0            | 1f26de8                    |
      | https://github.com/danielduarte/diffparse  | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c | 1.0.7        | 2.0.0            | 4f6bf53                    |
      | https://gitlab.com/DeveloperC/port-scanner | cb2cd79eac4a8f82d4029a998c59757f93b69a8f | 5b74e60ff6eb2eb575c4fb7cecbf3036b89de8d8 | 0.0.1        | 0.1.0            | 5b74e60                    |


  Scenario Outline: A shortened and full Git commit hash can be used interchangeably.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<shortened_from_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                              | checkout_commit                          | from_commit_hash                         | from_version | expected_version | shortened_from_commit_hash |
      | https://github.com/yargs/yargs.git      | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1        | 0.2.2            | 262e796                    |
      | https://github.com/yargs/yargs.git      | 6014e39bca3a1e8445aa0fb2a435f6181e344c45 | 705384762919641fe9d4c0967452a292f5f52c6d | 1.2.0        | 1.4.0            | 7053847                    |
      | https://gitlab.com/dmfay/massive-js.git | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 7.2.0        | 7.5.3            | 3f20134                    |


  Scenario Outline: The shortened Git commit hash has no matches, so an error is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then their is a could not find shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                              | checkout_commit                          | from_shortened_commit_hash | from_version |
      | https://github.com/yargs/yargs.git      | 089417550ef5a5b8ce3578dd2a989191300b64cd | 272a194                    | 0.2.1        |
      | https://gitlab.com/dmfay/massive-js.git | 482c364acf5505b81c55245fac0472890d351662 | 3f235ee                    | 7.2.0        |


  Scenario Outline: The shortened Git commit hash is ambiguous as multiple commit hashes match it, so an error is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then their is a ambiguous shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                         | checkout_commit                          | from_shortened_commit_hash | from_version |
      | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 3f6                        | 0.2.1        |
