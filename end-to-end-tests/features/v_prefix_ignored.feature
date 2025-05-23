Feature: The semantic version if prefixed with a v is supported and can still be parsed.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".


    Examples:
      | repository                                   | checkout_commit                          | commit_hash                              | from_version | expected_version |
      | https://github.com/yargs/yargs.git           | 089417550ef5a5b8ce3578dd2a989191300b64cd | 262e796329f4a09083ca07e49f926ae43ac850e9 | v0.2.1       | 0.2.2            |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | c4f276e26455dc96705a49e7651e86a2345724ec | V1.0.4       | 1.0.5            |
      | https://github.com/BlazeSoftware/atoms.git   | 987956d91c24ae56b7198bbb527f45a88f69fcda | 2a174d6121a4f3f27d3782c3d35218d990b008df | V9.2.3       | 9.3.2            |