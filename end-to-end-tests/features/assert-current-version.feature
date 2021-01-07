Feature: The current version argument supplied is asserted to be equal or greater than the calculated next Semantic Versioning.


  Scenario Outline: The current version assertion passes with batched together increments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --current-version is provided as "<current_version>".
    And the --batch-commits flag is set.
    Then the current version assertion passes.


    Examples:
      | repository                                 | checkout_commit                          | from_commit_hash                         | from_version | current_version |
      | https://github.com/yargs/yargs.git         | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1        | 0.2.2           |
      | https://github.com/monicahq/monica.git     | c1e91b32b4874253a2d74ed786c3b98780a60320 | 51901e7622a9227a396704532c50051c21075d35 | 1.5.2        | 1.6.0           |
      | https://github.com/BlazeSoftware/atoms.git | c2dcb3e97a1bd3516bed85ff1769c30211f2337a | 46dd08718905cbdd2d056a2eac720052b9691985 | 11.0.0       | 12.0.1          |


  Scenario Outline: The current version assertion fails with batched together increments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --current-version is provided as "<current_version>".
    And the --batch-commits flag is set.
    Then the current version assertion fails.


    Examples:
      | repository                                 | checkout_commit                          | from_commit_hash                         | from_version | current_version |
      | https://github.com/yargs/yargs.git         | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | 0.2.1        | 0.2.1           |
      | https://github.com/monicahq/monica.git     | c1e91b32b4874253a2d74ed786c3b98780a60320 | 51901e7622a9227a396704532c50051c21075d35 | 1.5.2        | 1.5.2           |
      | https://github.com/BlazeSoftware/atoms.git | c2dcb3e97a1bd3516bed85ff1769c30211f2337a | 46dd08718905cbdd2d056a2eac720052b9691985 | 11.0.0       | 11.0.3          |


  Scenario Outline: The current version assertion passes with consecutive increments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --current-version is provided as "<current_version>".
    Then the current version assertion passes.


    Examples:
      | repository                                   | checkout_commit                          | from_commit_hash                         | from_version | current_version |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | 5afe20347dd3ae954b31707a67f381f87920797f | 4.1.2        | 4.2.7           |
      | https://gitlab.com/dmfay/massive-js.git      | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 2.0.1        | 3.1.0           |


  Scenario Outline: The current version assertion fails with consecutive increments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --current-version is provided as "<current_version>".
    Then the current version assertion fails.


    Examples:
      | repository                                   | checkout_commit                          | from_commit_hash                         | from_version | current_version |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | 5afe20347dd3ae954b31707a67f381f87920797f | 4.1.2        | 4.1.3           |
      | https://gitlab.com/dmfay/massive-js.git      | 482c364acf5505b81c55245fac0472890d351662 | 3f20134864b19b11541287af440540c7ad0ed986 | 2.0.1        | 2.1.0           |
