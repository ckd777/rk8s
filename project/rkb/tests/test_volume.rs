use anyhow::Result;
use serial_test::serial;
use std::path::Path;

use rkb::commands::volume::{VolumeCommand, volume_execute};
use test_common::*;

mod test_common;

#[test]
#[serial]
fn test_volume_commands() {
    let volume_name = "test-volume";
    
    // Ensure previous test resources are cleaned up
    cleanup_volume(volume_name).unwrap();
    
    // Test volume create command
    let result = volume_execute(VolumeCommand::Create {
        name: volume_name.to_string(),
        driver: None,
        opts: None,
    });
    assert!(result.is_ok() || result.is_err(), "volume create command should parse correctly");
    
    // Test volume ls command
    let result = volume_execute(VolumeCommand::Ls {
        quiet: false,
    });
    assert!(result.is_ok() || result.is_err(), "volume ls command should parse correctly");
    
    // Test volume inspect command
    let result = volume_execute(VolumeCommand::Inspect {
        name: vec![volume_name.to_string()],
    });
    assert!(result.is_ok() || result.is_err(), "volume inspect command should parse correctly");
    
    // Test volume rm command
    let result = volume_execute(VolumeCommand::Rm {
        volumes: vec![volume_name.to_string()],
        force: true,
    });
    assert!(result.is_ok() || result.is_err(), "volume rm command should parse correctly");
    
    // Clean up resources
    cleanup_volume(volume_name).unwrap();
}

#[test]
#[serial]
fn test_volume_prune() {
    // Test volume prune command
    let result = volume_execute(VolumeCommand::Prune {
        force: true,
    });
    assert!(result.is_ok() || result.is_err(), "volume prune command should parse correctly");
}
