use crate::{copy, image, login, logout, overlayfs, pull, push, repo, run,};
use crate::commands::{ContainerCommand, ComposeCommand, PodCommand, VolumeCommand};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rkb", about = "A container runtime and management tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build a container image from Dockerfile
    Build(image::BuildArgs),
    /// Manage containers
    #[command(subcommand)]
    Container(ContainerCommand),
    /// Manage container compositions
    #[command(subcommand)]
    Compose(ComposeCommand),
    #[command(hide = true)]
    Cleanup(overlayfs::CleanupArgs),
    #[command(hide = true)]
    Copy(copy::CopyArgs),
    /// Login to distribution server
    Login(login::LoginArgs),
    /// Logout from distribution server
    Logout(logout::LogoutArgs),
    #[command(hide = true)]
    Mount(overlayfs::MountArgs),
    /// Pull an image from specific distribution server.
    Pull(pull::PullArgs),
    /// Push an image to specific distribution server.
    Push(push::PushArgs),
    /// Manage pods
    #[command(subcommand)]
    Pod(PodCommand),
    /// List and manage repositories
    Repo(repo::RepoArgs),
    #[command(hide = true)]
    Run(run::RunArgs),
    /// Manage volumes
    #[command(subcommand)]
    Volume(VolumeCommand),
}
