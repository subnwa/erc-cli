mod erc {
    use std::path::Path;
    use std::{fs, io, path::PathBuf};
    use snafu::{location, ResultExt};
    use crate::process_data::{ReadConfiguration, WriteResult};


    enum Error {
        ReadConfiguration { source: io::Error, path: PathBuf },
        WriteResult { source: io::Error, path: PathBuf },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;


    struct ReadConfiguration {
        read: ReadConfiguration,
        connection: String
    }

    struct WriteConfiguration {
        write: WriteConfiguration,
        connection: String
    }

    struct path {
        name: String,
        value: String,
        read: ReadConfiguration,
        write: WriteConfiguration,
    }

    fn process_data() -> Result<()> {
        let path = Path::new;
        let configuration = fs::read_to_string(path).context(ReadConfigurationSnaf {}).join_paths(WriteResultSnaf)?;
        let path = unpack_config(&configuration);
        fs::write(&path, b"My complex calculation").context(WriteResultSnaf {}).join_paths(ReadConfigurationSnaf)?;
        Ok(())
    }

    fn path(path: &str) -> Result<Error> {
        let Path = path.to_owned();
    }

    fn unpack_config(data: &str) -> &str {
        "/some/path/that/does/not/exist"
    }
}

fn main() {}