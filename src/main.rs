mod handlers;

use clap::Parser;
use handlers::args_handler::*;
use handlers::cmd_handler::run_cmd;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Force overwrite
    #[arg(short, long)]
    force: bool,
    /// Backup dotfiles
    #[arg(short, long)]
    backup: bool,
    /// Link to homedir
    #[arg(short, long)]
    link: bool,
    /// git config (.gitconfig_shared)
    #[arg(short, long)]
    gitconfig_shared: bool,
}

fn main() {
    let args = Args::parse();
    install_handler(args);
}

fn install_handler(args: Args) {
    link_to_homedir(args.force, args.backup, args.link);
    println!("");

    if args.gitconfig_shared {
        run_cmd(
            "git config --global include.path ~/.gitconfig_shared",
            true,
            Some(&true),
        );
        println!("");
    }

    run_cmd(r#"echo -e \e[1;36m Install completed!!!! \e[m"#, true, None);
}
