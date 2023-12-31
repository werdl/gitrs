include!("core.rs");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_run() {
        let git = Git::new(vec!["--version"]);
        let result = git.run();

        assert!(result.is_ok(), "Expected Ok, got Err");

        assert!(result.stdout().is_some(), "Expected stdout, got None");
        assert_eq!(result.code(), 0, "Expected exit code 0, got {}", result.code());
    }

    #[test]
    fn test_git_stream() {
        let git = Git::new(vec!["--version"]);
        let result = git.stream();

        assert!(result.is_ok(), "Expected Ok, got Err");

        assert!(result.stdout().is_none(), "Expected None, got Some");
        assert_eq!(result.code(), 0, "Expected exit code 0, got {}", result.code());
    }
}
