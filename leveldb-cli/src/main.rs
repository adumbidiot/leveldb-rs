use anyhow::Context;
use anyhow::bail;
use anyhow::ensure;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug)]
pub enum Command {
    /// Exit CLI
    Exit,

    /// Open a database
    Open {
        /// The path to the database
        path: String,
    },

    /// List entries
    List,

    /// Set the key format.
    KeyFormat {
        /// The new format
        format: KeyFormat,
    },

    /// Set the value format
    ValueFormat {
        /// The new format
        format: ValueFormat,
    },
}

/// The format to display keys with
#[derive(Debug, Default, Copy, Clone)]
pub enum KeyFormat {
    /// A byte array
    #[default]
    Bytes,

    /// A UTF-8 String,
    Utf8,
}

/// The format to display values with
#[derive(Debug, Default, Copy, Clone)]
pub enum ValueFormat {
    /// A byte array
    #[default]
    Bytes,

    /// Chrome Local Storage
    ChromeLocalStorage,
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        let input = input
            .strip_prefix('.')
            .context("commands must start with a \".\"")?;

        let (command, input) = match input.split_once(' ') {
            Some((command, input)) => (command, Some(input)),
            None => (input, None),
        };

        match command {
            "exit" => {
                ensure!(input.is_none(), "\".exit\" takes no arguments");

                Ok(Self::Exit)
            }
            "open" => {
                let input = input.context("\".open\" needs 1 argument")?;

                Ok(Self::Open { path: input.into() })
            }
            "list" => {
                ensure!(input.is_none(), "\".list\" takes no arguments");

                Ok(Self::List)
            }
            "keyformat" => {
                let input = input.context("\".keyformat\" needs 1 argument")?;

                let format = match input {
                    "bytes" => KeyFormat::Bytes,
                    "utf8" => KeyFormat::Utf8,
                    _ => bail!("unknown key format \"{input}\""),
                };

                Ok(Self::KeyFormat { format })
            }
            "valueformat" => {
                let input = input.context("\".valueformat\" needs 1 argument")?;

                let format = match input {
                    "bytes" => ValueFormat::Bytes,
                    "chromelocalstorage" => ValueFormat::ChromeLocalStorage,
                    _ => bail!("unknown value format \"{input}\""),
                };

                Ok(Self::ValueFormat { format })
            }
            _ => bail!("unknown command \".{command}\""),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    let mut database = None;
    let mut key_format = KeyFormat::default();
    let mut value_format = ValueFormat::default();

    loop {
        {
            let mut stdout = std::io::stdout().lock();
            write!(&mut stdout, "leveldb> ")?;
            stdout.flush()?;
        }

        {
            input.clear();

            let mut stdin = std::io::stdin().lock();
            stdin.read_line(&mut input)?;
        }

        let command: Command = match input.parse() {
            Ok(command) => command,
            Err(error) => {
                println!("Error: {error}");
                continue;
            }
        };

        match command {
            Command::Exit => {
                println!("Goodbye!");
                break;
            }
            Command::Open { path } => {
                if database.is_some() {
                    println!("Error: Another database is already open");
                    continue;
                }

                let new_database = match leveldb::Db::open(path.clone(), Default::default()) {
                    Ok(database) => database,
                    Err(error) => {
                        println!("Error: {error:?}");
                        continue;
                    }
                };

                database = Some(new_database);

                println!("Opened \"{path}\"");
            }
            Command::List => {
                let database = match database.as_mut().context("No database is open") {
                    Ok(database) => database,
                    Err(error) => {
                        println!("Error: {error:?}");
                        continue;
                    }
                };

                for (key, value) in database.iter_owned(&Default::default()) {
                    match key_format {
                        KeyFormat::Bytes => {
                            println!("Key: {key:?}");
                        }
                        KeyFormat::Utf8 => {
                            let key = String::from_utf8(key).context("invalid utf8");

                            match key {
                                Ok(key) => {
                                    println!("Key: {key:?}");
                                }
                                Err(error) => {
                                    println!("Key: {error}");
                                }
                            }
                        }
                    }

                    match value_format {
                        ValueFormat::Bytes => {
                            println!("Value: {value:?}");
                        }
                        ValueFormat::ChromeLocalStorage => {
                            let type_byte = value[0];

                            match type_byte {
                                0 => {
                                    // TODO: Validate buffer len
                                    let value: Vec<u16> = value[1..]
                                        .chunks(2)
                                        .map(|bytes| u16::from_ne_bytes(bytes.try_into().unwrap()))
                                        .collect();

                                    // TODO: Handle invalid UTF16 somehow
                                    let value = String::from_utf16(&value).context("invalid utf16");

                                    match value {
                                        Ok(value) => {
                                            println!("Value: {value:?}");
                                        }
                                        Err(error) => {
                                            println!("Value: {error}");
                                        }
                                    }
                                }
                                1 => {
                                    // TODO: Do I need to handle invalid utf8 here?
                                    let value =
                                        std::str::from_utf8(&value[1..]).context("invalid utf8");
                                    match value {
                                        Ok(value) => {
                                            println!("Value: {value:?}");
                                        }
                                        Err(error) => {
                                            println!("Value: {error}");
                                        }
                                    }
                                }
                                _ => {
                                    println!("Value: Invalid type byte {type_byte}");
                                }
                            }
                        }
                    }

                    println!();
                }
            }
            Command::KeyFormat { format } => {
                key_format = format;

                println!("Key format changed to \"{format:?}\"");
            }
            Command::ValueFormat { format } => {
                value_format = format;

                println!("Value format changed to \"{format:?}\"");
            }
        }
    }

    Ok(())
}
