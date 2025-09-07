use clap::{Parser, Subcommand};
use slmm::{
    core::{manager::ModManager, storage::Storage},
    game_driver::{generic::GenericGameDriver, kind::GameDriverKind},
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "slmm", version, about = "Manage game mods on Linux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    View {
        #[arg(short, long)]
        config_path: PathBuf,
    },

    Deploy {
        #[arg(short, long)]
        config_path: PathBuf,
    },

    AddMod {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        version: String,
        #[arg(short, long)]
        mod_path: PathBuf,
        #[arg(short, long)]
        config_path: PathBuf,
    },

    Create {
        #[arg(short, long)]
        config_path: PathBuf,
        #[arg(short, long)]
        game_path: PathBuf,
        #[arg(short, long)]
        mod_dir_path: PathBuf,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let mut mod_manager: ModManager;

    match cli.command {
        Commands::View { config_path } => {
            mod_manager = ModManager::load_game_state(config_path).unwrap();
            println!("{:#?}", mod_manager);
        }
        Commands::Deploy { config_path } => {
            mod_manager = ModManager::load_game_state(config_path).unwrap();
            println!("{:#?}", mod_manager);
            mod_manager.deploy().unwrap();
        }
        Commands::AddMod {
            name,
            version,
            mod_path,
            config_path,
        } => {
            mod_manager = ModManager::load_game_state(config_path).unwrap();
            println!("{:#?}", mod_manager);
            let installed_mod_uuid = mod_manager.install_mod(name, version, &mod_path).unwrap();
            println!("created mod uuid: {:?}", installed_mod_uuid);
            let installed_mod = mod_manager.get_mod(installed_mod_uuid).unwrap();
            println!("created mod: {:#?}", installed_mod);
            mod_manager.save_game_state().unwrap();
        }
        Commands::Create {
            config_path,
            game_path,
            mod_dir_path,
        } => {
            let storage = Storage::new(config_path);
            let game_driver = GenericGameDriver::new(game_path);
            mod_manager =
                ModManager::new(mod_dir_path, storage, GameDriverKind::Generic(game_driver))
                    .unwrap();
            mod_manager.save_game_state().unwrap();
        }
    }
}
