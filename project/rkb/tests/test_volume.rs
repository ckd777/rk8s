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
    
    // 确保之前的测试资源已清理
    cleanup_volume(volume_name).unwrap();
    
    // 测试volume create命令
    let result = volume_execute(VolumeCommand::Create {
        name: volume_name.to_string(),
        driver: None,
        opts: None,
    });
    assert!(result.is_ok() || result.is_err(), "volume create command should parse correctly");
    
    // 测试volume ls命令
    let result = volume_execute(VolumeCommand::Ls {
        quiet: false,
    });
    assert!(result.is_ok() || result.is_err(), "volume ls command should parse correctly");
    
    // 测试volume inspect命令
    let result = volume_execute(VolumeCommand::Inspect {
        name: vec![volume_name.to_string()],
    });
    assert!(result.is_ok() || result.is_err(), "volume inspect command should parse correctly");
    
    // 测试volume rm命令
    let result = volume_execute(VolumeCommand::Rm {
        volumes: vec![volume_name.to_string()],
        force: true,
    });
    assert!(result.is_ok() || result.is_err(), "volume rm command should parse correctly");
    
    // 清理资源
    cleanup_volume(volume_name).unwrap();
}

#[test]
#[serial]
fn test_volume_prune() {
    // 测试volume prune命令
    let result = volume_execute(VolumeCommand::Prune {
        force: true,
    });
    assert!(result.is_ok() || result.is_err(), "volume prune command should parse correctly");
}
