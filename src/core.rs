use std::process::Command;


/// IsFailure - providing idiomatic ways to access fields without unwrapping
/// 
/// Returns options, partly as it might have been piped, and partly as Success does not have the `stderr` field
pub trait IsFailure {
    /// idiomatic way to check if a command has failed
    fn failed(&self) -> bool;

    /// get code without unwrapping
    fn code(&self) -> i32;

    /// get stdout without unwrapping
    fn stdout(&self) -> Option<String>;

    /// get stderr without unwrapping
    fn stderr(&self) -> Option<String>;
}

impl IsFailure for Result<Success, Failure> {
    fn failed(&self) -> bool {
        self.is_err()
    }
    fn code(&self) -> i32 {
        match self {
            Ok(success) => success.code,
            Err(failure) => failure.code,
        }
    }
    fn stdout(&self) -> Option<String> {
        match self {
            Ok(success) => success.stdout.clone(),
            Err(failure) => failure.stdout.clone(),
        }
    }
    fn stderr(&self) -> Option<String> {
        match self {
            Ok(_) => None,
            Err(failure) => failure.stderr.clone(),
        }
    }
}

/// Successful command execution struct
/// 
/// Therefore, `stderr` is no provided
/// 
/// sometimes piped to parent, so `Option<String>` is used
pub struct Success {
    pub stdout: Option<String>,
    pub code: i32,
}

/// Failed command execution struct
/// 
/// Therefore, `stderr` is provided
/// 
/// sometimes piped to parent, so `Option<String>` is used
pub struct Failure {
    pub stderr: Option<String>,
    pub stdout: Option<String>,
    pub code: i32,
}

/// git struct - the core of `rsgit`
/// 
/// to initialize, see `Git::new`
pub struct Git {
    command: Vec<String>,
}

impl Git {
    /// Creates a new instance of the Git structure
    /// 
    /// The provided items must be able to be coerced into a `Vec<String>`
    /// 
    /// Returned is an instance of the Git struct
    /// # Examples
    /// ```rust
    /// use rsgit::Git;
    /// let cmd = Git::new(vec!["log", "--shortstat"]);
    /// ```
    pub fn new<T>(items: T) -> Git
    where
        T: IntoIterator,
        T::Item: ToString, 
    {
        return Git {
            command: items.into_iter().map(|x| x.to_string()).collect(),
        }
    }


    /// Runs the specified commands, prefixed by `git`
    /// 
    /// Returns either success or failure
    /// 
    /// stdin, stdout and stderr are all inherited from the parent
    /// # Examples
    /// ```rust
    /// use rsgit::{IsFailure, Git};
    /// let cmd = Git::new(vec!["log", "--shortstat"]);
    /// let output = cmd.stream();
    /// println!("git log --shortstat returned code {}", output.code());
    /// ```
    pub fn stream(&self) -> Result<Success, Failure> {
        let mut out = Command::new("git");
        for argument in self.command.clone() {
            out.arg(argument);
        }

        let output = out.status().expect("Failed to execute `git`");
        if output.success() {
            return Ok(Success {
                stdout: None, 
                code: output.code().unwrap_or(0)
            })
        } else {
            return Err(Failure{
                stderr: None,
                stdout: None,
                code: output.code().unwrap_or(1)
            })
        }
    }

    /// Runs the specified commands, prefixed by `git`
    /// 
    /// Returns either success or failure
    /// 
    /// stdin, stdout and stderr are all returned in an object
    /// # Examples
    /// ```rust
    /// use rsgit::{IsFailure, Git};
    /// let cmd = Git::new(vec!["log", "--shortstat"]);
    /// let output = cmd.run();
    /// println!("The output of git log --shortstat was {}", output.stdout().unwrap_or_default());
    /// ```
    pub fn run(&self) -> Result<Success, Failure> {
        let mut out = Command::new("git");
        for argument in self.command.clone() {
            out.arg(argument);
        }

        let output = out.output().expect("Failed to execute `git`");

        if output.status.success() {
            return Ok(Success {
                stdout: Some(String::from_utf8(output.stdout).unwrap_or("".to_string())),
                code: output.status.code().unwrap_or(0)
            })
        } else {
            return Err(Failure {
                stderr: Some(String::from_utf8(output.stderr).unwrap_or("".to_string())),
                stdout: Some(String::from_utf8(output.stdout).unwrap_or("".to_string())),
                code: output.status.code().unwrap_or(1)
            })
        }
    }
}

pub trait Run {
    /// run a command straight from a object
    fn run(self) -> Result<Success, Failure>;

    /// stream a command straight from a object
    fn stream(self) -> Result<Success, Failure>; 
}

/// `run` - allows you to run a command directly from a type that support conversion to `Vec<String>`
/// 
/// Works in the same way as the main run function, returning an object
/// # Examples
/// ```rust
/// use rsgit::Run;
/// let output = vec!["log", "--shortstat"].run();
/// ```
/// `stream` - allows you to run a command directly from a type that support conversion to `Vec<String>`
/// 
/// Works in the same way as the main run function, returning an object
/// # Examples
/// ```rust
/// use rsgit::Run;
/// let _ = vec!["log", "--shortstat"].stream();
/// ```
impl<T> Run for T
where
    T: IntoIterator,
    T::Item: ToString,
{

    fn run(self) -> Result<Success, Failure> {
        Git::new(self.into_iter().map(|x| x.to_string())).run()
    }

    fn stream(self) -> Result<Success, Failure> {
        Git::new(self.into_iter().map(|x| x.to_string())).stream()
    }
}