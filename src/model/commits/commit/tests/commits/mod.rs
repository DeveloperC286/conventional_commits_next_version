use super::*;

#[test]
fn test_nfpm_version_2_5_1_in_batch_mode() {
    //https://github.com/goreleaser/nfpm/releases/tag/v2.5.1
    //Given
    let batch_commits = true;
    let from_version = Version::parse("2.5.0").unwrap();
    let expected_version = Version::parse("2.5.1").unwrap();

    let commits = Commits {
        commits: vec![
            Commit {
                message: "fix(ci): run build workflow on tags\n\nSigned-off-by: Carlos Alexandro Becker <caarlos0@gmail.com>\n".to_string(),
            },
            Commit {
                message: "chore(deps): bump github.com/spf13/cobra from 1.1.1 to 1.1.3 (#328)\n\nBumps [github.com/spf13/cobra](https://github.com/spf13/cobra) from 1.1.1 to 1.1.3.\r\n- [Release notes](https://github.com/spf13/cobra/releases)\r\n- [Changelog](https://github.com/spf13/cobra/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/spf13/cobra/compare/v1.1.1...v1.1.3)\r\n\r\nSigned-off-by: dependabot-preview[bot] <support@dependabot.com>\r\n\r\nCo-authored-by: dependabot-preview[bot] <27856297+dependabot-preview[bot]@users.noreply.github.com>".to_string(),
            },
            Commit {
                message: "chore: go mod tidy\n\nSigned-off-by: Carlos Becker <caarlos0@gmail.com>\n".to_string(),
            },
            Commit {
                message: "Upgrade to GitHub-native Dependabot (#330)\n\nCo-authored-by: dependabot-preview[bot] <27856297+dependabot-preview[bot]@users.noreply.github.com>".to_string(),
            },
            Commit {
                message: "chore(deps): bump github.com/ProtonMail/gopenpgp/v2 from 2.1.7 to 2.1.8 (#329)\n\nBumps [github.com/ProtonMail/gopenpgp/v2](https://github.com/ProtonMail/gopenpgp) from 2.1.7 to 2.1.8.\r\n- [Release notes](https://github.com/ProtonMail/gopenpgp/releases)\r\n- [Changelog](https://github.com/ProtonMail/gopenpgp/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/ProtonMail/gopenpgp/compare/v2.1.7...v2.1.8)\r\n\r\nSigned-off-by: dependabot-preview[bot] <support@dependabot.com>\r\n\r\nCo-authored-by: dependabot-preview[bot] <27856297+dependabot-preview[bot]@users.noreply.github.com>".to_string(),
            },
            Commit {
                message: "fix: go mod tidy\n\nSigned-off-by: Carlos A Becker <caarlos0@gmail.com>\n".to_string(),
            },
            Commit {
                message: "fix: remove semver parsing code before schema version detection (#333)\n\nCo-authored-by: Carlos Alexandro Becker <caarlos0@users.noreply.github.com>".to_string(),
            },
        ]
    };

    //When
    let returned_version = commits.get_next_version(from_version, batch_commits);

    //Then
    assert_eq!(returned_version, expected_version);
}

#[test]
fn test_mhy_in_consecutive_mode() {
    //https://github.com/wintercounter/mhy
    //Given
    let batch_commits = false;
    let from_version = Version::parse("1.2.1").unwrap();
    let expected_version = Version::parse("1.3.2").unwrap();

    let commits = Commits {
        commits: vec![
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
        ]
    };

    //When
    let returned_version = commits.get_next_version(from_version, batch_commits);

    //Then
    assert_eq!(returned_version, expected_version);
}
