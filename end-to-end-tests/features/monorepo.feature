Feature: The next semantic version is calculated only from commits altering files which match any of these provided regexes, enabling usage within monorepos.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".
    Given the arguments are reset.
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --monorepo is provided as "<monorepo>".
    Then the returned version should be "<monorepo_expected_version>".

    Examples:
      | repository                                   | checkout_commit                          | commit_hash                              | from_version | expected_version | monorepo                        | monorepo_expected_version |
      | https://github.com/yargs/yargs.git           | acff16db1057ea830a37f2214782e5026be894b6 | cb01c98c44e30f55c2dc9434caef524ae433d9a4 | 1.7.2        | 1.8.0            | helpers/                        | 1.7.3                     |
      | https://github.com/istanbuljs/istanbuljs.git | 1b52fe750d1f800c34dbff168614c0c73bd76026 | df24342395030dc2a40a7ceb0476a9897f3492a3 | 3.0.1        | 3.0.5            | packages/istanbul-reports/test/ | 3.0.2                     |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    Then the returned version should be "<expected_version>".
    Given the arguments are reset.
    When linting from the "<commit_hash>".
    And the argument --from-version is provided as "<from_version>".
    And the argument --monorepo is provided as "<monorepo_1>".
    And the argument --monorepo is provided as "<monorepo_2>".
    Then the returned version should be "<monorepo_expected_version>".

    Examples:
      | repository                         | checkout_commit                          | commit_hash                              | from_version | expected_version | monorepo_1 | monorepo_2   | monorepo_expected_version |
      | https://github.com/yargs/yargs.git | acff16db1057ea830a37f2214782e5026be894b6 | cb01c98c44e30f55c2dc9434caef524ae433d9a4 | 1.7.2        | 1.8.0            | helpers/   | package.json | 1.7.4                     |
