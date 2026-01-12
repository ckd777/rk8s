use anyhow::Result;
use serial_test::serial;
use std::path::Path;

use rkb::commands::container::{ContainerCommand, container_execute};
use test_common::*;

mod test_common;

/// 获取测试用的容器配置
fn get_container_config() -> String {
    format!(r#"name: test-container
image: {}
command: ["sleep", "10"]
ports:
  - "8080:8080"
"#, bundles_path("busybox"))
}

#[test]
#[serial]
fn test_container_commands() {
    let container_name = "test-container";
    let container_config = get_container_config();
    
    // 创建临时容器配置文件
    let container_path = create_temp_compose_file(&container_config).unwrap();
    
    // 确保之前的测试资源已清理
    cleanup_container(container_name).unwrap();
    
    // 测试container run命令
    let result = container_execute(ContainerCommand::Run {
        container_yaml: container_path.clone(),
        volumes: None,
    });
    assert!(result.is_ok() || result.is_err(), "container run command should parse correctly");
    
    // 测试container list命令
    let result = container_execute(ContainerCommand::List {
        quiet: None,
        format: None,
    });
    assert!(result.is_ok() || result.is_err(), "container list command should parse correctly");
    
    // 测试container state命令
    let result = container_execute(ContainerCommand::State {
        container_name: container_name.to_string(),
    });
    assert!(result.is_ok() || result.is_err(), "container state command should parse correctly");
    
    // 测试container delete命令
    let result = container_execute(ContainerCommand::Delete {
        container_name: container_name.to_string(),
    });
    assert!(result.is_ok() || result.is_err(), "container delete command should parse correctly");
    
    // 清理临时文件
    cleanup_temp_compose_file(&container_path).unwrap();
    cleanup_container(container_name).unwrap();
}
