mod erc {
    use snafu::prelude::*;
    use std::{fs, io, path::PathBuf};

    #[derive(Debug, Snafu)]
    enum Error {
        #[snafu(display("Unable to read configuration from {}: {}", path.display(), source))]
        ReadConfiguration { source: io::Error, path: PathBuf },

        #[snafu(display("Unable to write result to {}: {}", path.display(), source))]
        WriteResult { source: io::Error, path: PathBuf },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    fn process_data() -> Result<()> {
        let path = "config.toml";
        let configuration = fs::read_to_string(path).context(ReadConfigurationSnafu { path })?;
        let path = unpack_config(&configuration);
        fs::write(&path, b"My complex calculation").context(WriteResultSnafu { path })?;
        Ok(())
    }

    fn unpack_config(data: &str) -> &str {
        "/some/path/that/does/not/exist"
    }
}