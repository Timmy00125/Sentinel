use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::de::DeserializeOwned;
use sentinel_config::{PolicyConfig, SentinelConfig};
use sentinel_policy::StaticPolicyEngine;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ModeArg {
    Observe,
    Suggest,
    Apply,
}

#[derive(Debug, Parser)]
#[command(name = "sentinel")]
#[command(about = "Autonomous DevOps agent")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {
        #[arg(long, value_enum)]
        mode: ModeArg,
        #[arg(long)]
        config: String,
    },
    Doctor {
        #[arg(long)]
        config: String,
    },
    Integrations {
        #[command(subcommand)]
        command: IntegrationsCommand,
    },
    Policy {
        #[command(subcommand)]
        command: PolicyCommand,
    },
    Incidents {
        #[command(subcommand)]
        command: IncidentsCommand,
    },
    Remediation {
        #[command(subcommand)]
        command: RemediationCommand,
    },
    Audit {
        #[command(subcommand)]
        command: AuditCommand,
    },
}

#[derive(Debug, Subcommand)]
enum IntegrationsCommand {
    List {
        #[arg(long)]
        config: String,
    },
}

#[derive(Debug, Subcommand)]
enum PolicyCommand {
    Validate {
        #[arg(long)]
        policy: String,
    },
}

#[derive(Debug, Subcommand)]
enum IncidentsCommand {
    List {
        #[arg(long)]
        since: String,
    },
}

#[derive(Debug, Subcommand)]
enum RemediationCommand {
    Explain {
        #[arg(long)]
        incident: String,
    },
    Approve {
        #[arg(long)]
        incident: String,
        #[arg(long)]
        action: String,
    },
}

#[derive(Debug, Subcommand)]
enum AuditCommand {
    Tail {
        #[arg(long)]
        since: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { mode, config } => {
            let loaded: SentinelConfig = read_yaml(&config)?;
            println!("run mode={:?} config_mode={:?}", mode, loaded.mode);
            println!("starter pipeline stub: collector -> detect -> decide -> policy -> execute");
        }
        Commands::Doctor { config } => {
            let loaded: SentinelConfig = read_yaml(&config)?;
            println!("doctor ok: loaded {} integrations", loaded.integrations.len());
        }
        Commands::Integrations { command } => match command {
            IntegrationsCommand::List { config } => {
                let loaded: SentinelConfig = read_yaml(&config)?;
                for integration in loaded.integrations {
                    println!(
                        "id={} kind={} enabled={}",
                        integration.id, integration.kind, integration.enabled
                    );
                }
            }
        },
        Commands::Policy { command } => match command {
            PolicyCommand::Validate { policy } => {
                let loaded: PolicyConfig = read_yaml(&policy)?;
                let _engine = StaticPolicyEngine::new(loaded);
                println!("policy validation passed");
            }
        },
        Commands::Incidents { command } => match command {
            IncidentsCommand::List { since } => {
                println!("incident listing stub since={since}");
            }
        },
        Commands::Remediation { command } => match command {
            RemediationCommand::Explain { incident } => {
                println!("remediation explain stub for incident={incident}");
            }
            RemediationCommand::Approve { incident, action } => {
                println!("remediation approve stub incident={incident} action={action}");
            }
        },
        Commands::Audit { command } => match command {
            AuditCommand::Tail { since } => {
                println!("audit tail stub since={since}");
            }
        },
    }

    Ok(())
}

fn read_yaml<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: DeserializeOwned,
{
    let path_ref = path.as_ref();
    let content = fs::read_to_string(path_ref)
        .with_context(|| format!("failed to read file: {}", path_ref.display()))?;
    serde_yaml::from_str::<T>(&content)
        .with_context(|| format!("failed to parse yaml: {}", path_ref.display()))
}
