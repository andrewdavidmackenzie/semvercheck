use semver::{BuildMetadata, Prerelease, Version, VersionReq};

fn main() {
    use git2::Repository;

    match Repository::open(std::env::current_dir().unwrap()) {
        Ok(repo) => {
            println!("Opened {:?}", repo.path());
            let head = repo.head().unwrap();
            println!("HEAD is local branch {}", head.is_branch());
        },
        Err(e) => panic!("failed to open: {}", e),
    };

    let req = VersionReq::parse(">=1.2.3, <1.8.0").unwrap();

    // Check whether this requirement matches version 1.2.3-alpha.1 (no)
    let version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: Prerelease::new("alpha.1").unwrap(),
        build: BuildMetadata::EMPTY,
    };
    assert!(!req.matches(&version));

    // Check whether it matches 1.3.0 (yes it does)
    let version = Version::parse("1.3.0").unwrap();
    assert!(req.matches(&version));
}
