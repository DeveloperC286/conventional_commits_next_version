Feature: The usage of filtering on multiple monorepos within the repository is supported, the version is calculated only from commits altering the specified directory.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the returned version should be "<expected_version>".
    Given the arguments are reset.
    When the argument --from-version is provided as "<from_version>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --monorepo is provided as "<monorepo_1>".
    And the argument --monorepo is provided as "<monorepo_2>".
    Then the returned version should be "<monorepo_expected_version>".

    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | from_version | expected_version | monorepo_1 | monorepo_2   | monorepo_expected_version |
      | https://github.com/yargs/yargs.git | acff16db1057ea830a37f2214782e5026be894b6 | cb01c98c44e30f55c2dc9434caef524ae433d9a4 | 1.7.2        | 1.8.0            | helpers/   | package.json | 1.7.4                     |
