Feature: Ensuring conventional-commits-next-version correctly calculates next version based on conventional commits in a consecutive manner.

Scenario: One fix conventional commit increments patch semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "089417550ef5a5b8ce3578dd2a989191300b64cd".
  When I call conventional-commits-next-version with the from commit "262e796329f4a09083ca07e49f926ae43ac850e9" and version "0.2.1".
  Then the returned version should be "0.2.2".

Scenario: One feat conventional commit increments minor semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "027a6365b737e13116811a8ef43670196e1fa00a".
  When I call conventional-commits-next-version with the from commit "1f26de809432be9cc6f4f185629f6e5d13236598" and version "0.2.3".
  Then the returned version should be "0.3.0".


Scenario: One feat conventional commit increments minor semantic version and then one fix conventional commit increments patch semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "18b0b752424bf560271e670ff95a0f90c8386787".
  When I call conventional-commits-next-version with the from commit "ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6" and version "1.0.3".
  Then the returned version should be "1.1.1".


Scenario: Non fix/feat conventional commit do not change the semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "2fed2a7c58de1d7c60858c0e8ff24421609e0dc4".
  When I call conventional-commits-next-version with the from commit "6014e39bca3a1e8445aa0fb2a435f6181e344c45" and version "1.0.3".
  Then the returned version should be "1.0.3".


Scenario: One breaking change in the conventional commit title increments major semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "b9409da199ebca515a848489c206b807fab2e65d".
  When I call conventional-commits-next-version with the from commit "6e4bc2aa0a6c5560928296f18e008a15b2aaf335" and version "1.1.3".
  Then the returned version should be "2.0.0".


Scenario: One breaking change in only the conventional commit footer increments major semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "0aaa68bf36d35c697426de4dfe2e4e12128c2dc0".
  When I call conventional-commits-next-version with the from commit "927810c7615912fb77a160273b2d6a946e9737b8" and version "9.4.0".
  Then the returned version should be "10.0.0".
