Feature: Ensuring conventional-commits-next-version correctly calculates next version based on batching together the conventional commits.

Scenario: One fix conventional commit increments patch semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "089417550ef5a5b8ce3578dd2a989191300b64cd".
  When I call conventional-commits-next-version with the from commit "262e796329f4a09083ca07e49f926ae43ac850e9" and version "0.2.1" and batch-commits flag.
  Then the returned version should be "0.2.2".

Scenario: One feat conventional commit increments minor semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "027a6365b737e13116811a8ef43670196e1fa00a".
  When I call conventional-commits-next-version with the from commit "1f26de809432be9cc6f4f185629f6e5d13236598" and version "0.2.3" and batch-commits flag.
  Then the returned version should be "0.3.0".


Scenario: One feat conventional commit and one fix conventional commit increments minor semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "18b0b752424bf560271e670ff95a0f90c8386787".
  When I call conventional-commits-next-version with the from commit "ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6" and version "1.0.3" and batch-commits flag.
  Then the returned version should be "1.1.0".


Scenario: Non fix/feat/major conventional commits do not change the semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "2fed2a7c58de1d7c60858c0e8ff24421609e0dc4".
  When I call conventional-commits-next-version with the from commit "6014e39bca3a1e8445aa0fb2a435f6181e344c45" and version "1.0.3" and batch-commits flag.
  Then the returned version should be "1.0.3".


Scenario: Two fix conventional commits increments patch semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "b80bcf73533cedeb0118fc9b54a9899c89b98344".
  When I call conventional-commits-next-version with the from commit "8aae3332251d09fa136db17ef4a40d83fa052bc4" and version "1.1.3" and batch-commits flag.
  Then the returned version should be "1.1.4".


Scenario: Six fix and two feat conventional commits increments minor semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "932cd1177e93f5cc99edfe57a4028e30717bf8fb".
  When I call conventional-commits-next-version with the from commit "b7722a624424880971e15d5420047c390deb28ce" and version "13.2.5" and batch-commits flag.
  Then the returned version should be "13.3.0".


Scenario: One breaking change, thirteen fix and six feat conventional commits increments major semantic version.
  Given I clone the repository "https://github.com/yargs/yargs.git".
  Given I checkout the commit "932cd1177e93f5cc99edfe57a4028e30717bf8fb".
  When I call conventional-commits-next-version with the from commit "6e4bc2aa0a6c5560928296f18e008a15b2aaf335" and version "13.2.5" and batch-commits flag.
  Then the returned version should be "14.0.0".
