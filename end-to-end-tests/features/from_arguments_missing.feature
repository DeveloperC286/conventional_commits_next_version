Feature: A from argument is required and one must be provided.


  Scenario Outline: You must provide one of the from arguments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-version is provided as "<from_version>".
    Then their is a missing from argument error.


    Examples:
      | repository                         | checkout_commit                          | from_version |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 1.0.0        |
