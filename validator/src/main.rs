#![allow(clippy::arithmetic_side_effects)]

#[cfg(not(any(target_env = "msvc", target_os = "freebsd")))]
use jemallocator::Jemalloc;
use {
    builder_validator::{
        cli::{initialize_app, warn_deprecated_args, DefaultArgs},
        commands,
    },
    solana_streamer::socket::SocketAddrSpace,
    std::path::PathBuf,
};

#[cfg(not(any(target_env = "msvc", target_os = "freebsd")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub fn main() {
    let default_args = DefaultArgs::new();
    let app_version = "1.0.0";
    let cli_app = initialize_app(app_version, &default_args);
    let matches = cli_app.get_matches();
    warn_deprecated_args(&matches);

    let socket_addr_space = SocketAddrSpace::new(matches.is_present("allow_private_addr"));
    let ledger_path = PathBuf::from(matches.value_of("ledger_path").unwrap());

    match matches.subcommand() {
        ("init", _) => commands::run::execute(
            &matches,
            app_version,
            socket_addr_space,
            &ledger_path,
            "Initialize",
        ),
        ("", _) | ("run", _) => commands::run::execute(
            &matches,
            app_version,
            socket_addr_space,
            &ledger_path,
            "Run",
        ),
        ("authorized-voter", Some(sub_matches)) => {
            commands::authorized_voter::execute(sub_matches, &ledger_path)
        }
        ("plugin", Some(sub_matches)) => commands::plugin::execute(sub_matches, &ledger_path),
        ("contact-info", Some(sub_matches)) => {
            commands::contact_info::execute(sub_matches, &ledger_path)
        }
        ("exit", Some(sub_matches)) => commands::exit::execute(sub_matches, &ledger_path),
        ("monitor", _) => commands::monitor::execute(&matches, &ledger_path),
        ("staked-nodes-overrides", Some(sub_matches)) => {
            commands::staked_nodes_overrides::execute(sub_matches, &ledger_path)
        }
        ("set-identity", Some(sub_matches)) => {
            commands::set_identity::execute(sub_matches, &ledger_path)
        }
        ("set-log-filter", Some(sub_matches)) => {
            commands::set_log_filter::execute(sub_matches, &ledger_path)
        }
        ("wait-for-restart-window", Some(sub_matches)) => {
            commands::wait_for_restart_window::execute(sub_matches, &ledger_path)
        }
        ("repair-shred-from-peer", Some(sub_matches)) => {
            commands::repair_shred_from_peer::execute(sub_matches, &ledger_path)
        }
        ("repair-whitelist", Some(sub_matches)) => {
            commands::repair_whitelist::execute(sub_matches, &ledger_path)
        }
        ("set-public-address", Some(sub_matches)) => {
            commands::set_public_address::execute(sub_matches, &ledger_path)
        }
        ("set-engine-config", Some(sub_matches)) => {
            commands::engine::execute(sub_matches, &ledger_path)
        }
        ("set-relayer-config", Some(sub_matches)) => {
            commands::relayer::execute(sub_matches, &ledger_path)
        }
        ("set-shred-receiver-address", Some(sub_matches)) => {
            commands::shred::set_receiver_execute(sub_matches, &ledger_path)
        }
        ("set-shred-retransmit-address", Some(sub_matches)) => {
            commands::shred::set_retransmit_execute(sub_matches, &ledger_path)
        }
        ("runtime-plugin", Some(sub_matches)) => {
            commands::runtime_plugin::execute(sub_matches, &ledger_path)
        }
        _ => unreachable!(),
    };
}
