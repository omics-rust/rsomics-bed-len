use clap::Parser;
use rsomics_common::{CommonFlags, Result, RsomicsError, Tool, ToolMeta};
use rsomics_help::{Example, FlagSpec, HelpSpec, Section};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

use rsomics_bed_len::lengths;

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

pub const HELP: HelpSpec = HelpSpec {
    name: META.name,
    version: META.version,
    tagline: "Append interval length (end − start) as a new column to BED records.",
    origin: None,
    usage_lines: &["[OPTIONS] [INPUT]"],
    sections: &[Section {
        title: "OPTIONS",
        flags: &[
            FlagSpec {
                short: Some('o'),
                long: "out",
                aliases: &[],
                value: Some("<FILE>"),
                type_hint: Some("Path"),
                required: false,
                default: Some("stdout"),
                description: "Write output to FILE",
                why_default: None,
            },
            FlagSpec {
                short: Some('h'),
                long: "help",
                aliases: &[],
                value: None,
                type_hint: Some("bool"),
                required: false,
                default: None,
                description: "Show this help",
                why_default: None,
            },
        ],
    }],
    examples: &[
        Example {
            description: "Print lengths from file",
            command: "rsomics-bed-len intervals.bed",
        },
        Example {
            description: "Pipe from stdin",
            command: "cat intervals.bed | rsomics-bed-len",
        },
    ],
    json_result_schema_doc: None,
};

#[derive(Parser, Debug)]
#[command(name = "rsomics-bed-len", disable_help_flag = true)]
pub struct Cli {
    /// Input BED file (default: stdin)
    pub input: Option<PathBuf>,

    /// Output file (default: stdout)
    #[arg(short = 'o', long = "out")]
    pub out: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonFlags,
}

impl Tool for Cli {
    fn meta() -> ToolMeta {
        META
    }
    fn common(&self) -> &CommonFlags {
        &self.common
    }

    fn execute(self) -> Result<()> {
        let stdout = io::stdout();
        let mut output: Box<dyn io::Write> = match &self.out {
            Some(p) => Box::new(File::create(p).map_err(RsomicsError::Io)?),
            None => Box::new(stdout.lock()),
        };

        match &self.input {
            Some(p) => {
                let reader = BufReader::new(File::open(p).map_err(RsomicsError::Io)?);
                lengths(reader, &mut output)?;
            }
            None => {
                let stdin = io::stdin();
                lengths(stdin.lock(), &mut output)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        super::Cli::command().debug_assert();
    }
}
