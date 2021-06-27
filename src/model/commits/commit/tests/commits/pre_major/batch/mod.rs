use super::*;

#[test]
fn test_major_footer_increment() {
    //Given
    let batch_commits = true;
    let from_version = Version::parse("0.7.1").unwrap();
    let expected_version = Version::parse("0.8.0").unwrap();

    let commits = Commits {
        commits: vec![
            Commit {
                message: "feat: add usage for single-digit boolean aliases (#1580)\n\n".to_string(),
            },
            Commit {
                message: "chore: upgrade yargs-parser (#633)\n\n* chore: upgrade yargs-parser\r\nBREAKING-CHANGE: coerce is now applied as a final step after other parsing is complete\r\n\r\n* add documentation for breaking changes in yargs-parser@4\r\n\r\n* fix: a few small editing nits\r\n\r\n* fix: bump yargs-parser again\r\n".to_string(),
            },
            Commit {
                message: "fix(deps): fix enumeration for normalized path arguments (#1567)\n\n".to_string(),
            },
        ]
    };

    //When
    let returned_version = commits.get_next_version(from_version, batch_commits);

    //Then
    assert_eq!(returned_version, expected_version);
}

#[test]
fn test_major_title_increment() {
    //Given
    let batch_commits = true;
    let from_version = Version::parse("0.5.9").unwrap();
    let expected_version = Version::parse("0.6.0").unwrap();

    let commits = Commits {
        commits: vec![
            Commit {
                message: "fix(): fix enumeration for normalized path arguments (#1567)\n\n"
                    .to_string(),
            },
            Commit {
                message: " refactor!: drop support for Node 6".to_string(),
            },
            Commit {
                message: "docs: state limitations of using command handlers returning promises\n\n"
                    .to_string(),
            },
        ],
    };

    //When
    let returned_version = commits.get_next_version(from_version, batch_commits);

    //Then
    assert_eq!(returned_version, expected_version);
}
