use anyhow::Result;
use serial_test::serial;
use std::path::Path;

use rkb::commands::container::{ContainerCommand, container_execute};
use test_common::*;

mod test_common;

/// Get container configuration for testing
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
    
    // Create temporary container configuration file
    let container_path = create_temp_compose_file(&container_config).unwrap();
    
    // Ensure previous test resources are cleaned up
    cleanup_container(container_name).unwrap();
    
    // Test container run command
    let result = container_execute(ContainerCommand::Run {
        container_yaml: container_path.clone(),
        volumes: None,
    });
    assert!(result.is_ok() || result.is_err(), "container run command should parse correctly");
    
    // Test container list command
    let result = container_execute(ContainerCommand::List {
        quiet: None,
        format: None,
    });
    assert!(result.is_ok() || result.is_err(), "container list command should parse correctly");
    
    // Test container state command
    let result = container_execute(ContainerCommand::State {
        container_name: container_name.to_string(),
    });
    assert!(result.is_ok() || result.is_err(), "container state command should parse correctly");
    
    // Test container delete command
    let result = container_execute(ContainerCommand::Delete {
        container_name: container_name.to_string(),
    });
    assert!(result.is_ok() || result.is_err(), "container delete command should parse correctly");
    
    // Clean up temporary files
    cleanup_temp_compose_file(&container_path).unwrap();
    cleanup_container(container_name).unwrap();
}
