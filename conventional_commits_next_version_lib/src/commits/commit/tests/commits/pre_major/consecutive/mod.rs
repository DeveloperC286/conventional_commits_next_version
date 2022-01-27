use super::*;

#[test]
fn test_major_footer_increment() {
    //Given
    let batch_commits = false;
    let from_version = Version::parse("0.2.1").unwrap();
    let expected_version = Version::parse("0.3.1").unwrap();

    let commits = Commits {
        commits: VecDeque::from(vec![
            Commit {
                message: "chore(dependencies): update 21/03/2021\n".to_string(),
            },
            Commit {
                message: "feat: tweaks to API surface based on user feedback (#1726)\n\nBREAKING CHANGE: tweaks to ESM/Deno API surface: now exports yargs function by default; getProcessArgvWithoutBin becomes hidBin; types now exported for Deno.".to_string(),
            },
            Commit {
                message: "chore(release): 11.1.0\n".to_string(),
            },
            Commit {
                message: "fix(): fix enumeration for normalized path arguments (#1567)\n\n"
                    .to_string(),
            },
        ]),
    };

    //When
    let returned_version = commits.get_next_version(from_version, batch_commits);

    //Then
    assert_eq!(returned_version, expected_version);
}

#[test]
fn test_major_title_increment() {
    //Given
    let batch_commits = false;
    let from_version = Version::parse("0.7.2").unwrap();
    let expected_version = Version::parse("0.9.0").unwrap();

    let commits = Commits {
        commits: VecDeque::from(vec![
            Commit {
                message: "feat: add usage for single-digit boolean aliases (#1580)\n\n".to_string(),
            },
            Commit {
                message: " refactor!: drop support for Node 6".to_string(),
            },
            Commit {
                message: "docs: state limitations of using command handlers returning promises\n\n"
                    .to_string(),
            },
        ]),
    };

    //When
    let returned_version = commits.get_next_version(from_version, batch_commits);

    //Then
    assert_eq!(returned_version, expected_version);
}
