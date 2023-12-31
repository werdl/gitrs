use std::process::Command;

pub trait IsFailure {
    fn failed(&self) -> bool;
    fn code(&self) -> i32;
    fn stdout(&self) -> Option<String>;
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

pub struct Success {
    pub stdout: Option<String>, // sometimes piped to parent stdout
    pub code: i32,
}

pub struct Failure {
    pub stderr: Option<String>,
    pub stdout: Option<String>,
    pub code: i32,
}

pub struct Git {
    command: Vec<String>,
}

impl Git {

    /// Creates a new instance of the Git structure
    /// 
    /// The provided items must be able to be coerced into a Vec<String>
    /// 
    /// Returned is an instance of the Git struct
    /// # Examples
    /// ```rust
    /// use gitrs::core::*;
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
    /// use gitrs::core::*;
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
    /// use gitrs::core::*;
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
    fn run(&self) -> Result<Success, Failure>;
    fn stream(&self) -> Result<Success, Failure>;
}

impl <T: ToString> Run for T{
    fn run(&self) -> Result<Success, Failure>{
        return Git::new(self.to_string().split(" ").collect::<Vec<&str>>()).run();
    }

    fn stream(&self) -> Result<Success, Failure>{
        return Git::new(self.to_string().split(" ").collect::<Vec<&str>>()).stream();
    }
}