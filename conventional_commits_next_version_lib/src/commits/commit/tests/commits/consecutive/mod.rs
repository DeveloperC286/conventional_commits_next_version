use super::*;

#[test]
fn test_mhy() {
    //https://github.com/wintercounter/mhy
    //Given
    let calculation_mode = CalculationMode::Consecutive;
    let from_version = Version::parse("1.2.1").unwrap();
    let expected_version = Version::parse("1.3.2").unwrap();

    let commits = Commits {
        commits: VecDeque::from(vec![
            Commit {
                message: "chore(dependencies): update 21/03/2021\n".to_string(),
            },
            Commit {
                message: "fix(babel): remove `browsers: false` from preset config as it\'s no longer valid\n".to_string(),
            },
            Commit {
                message: "feat: move to NPM v7 support\nBREAKING CHANGES: npm < 7 are no no longer supported due to the peer dependency changes it introduced\n".to_string(),
            },
            Commit {
                message: "release\n".to_string(),
            },
            Commit {
                message: "fix: remove obsolete dep `@apollo/react-ssr`\n".to_string(),
            },
            Commit {
                message: "fix(eslint): update to new `eslint-config-prettier` usage\n".to_string(),
            },
            Commit {
                message: "chore(release): 11.1.0\n".to_string(),
            },
            Commit {
                message: "release\n".to_string(),
            },
        ])
    };

    //When
    let returned_version = commits.get_next_version(from_version, calculation_mode);

    //Then
    assert_eq!(returned_version, expected_version);
}
