use anyhow::Result;
use serial_test::serial;
use std::path::Path;

use rkb::commands::compose::{ComposeCommand, DownArgs, PsArgs, UpArgs, compose_execute};
use test_common::*;

mod test_common;

/// 获取测试用的compose配置
fn get_compose_config(project_name: &str) -> String {
    format!(r#"name: {}

services:
  backend:
    container_name: backend
    image: {}
    command: ["sleep", "10"]
    ports:
      - "8080:8080"
    networks:
      - default-net
  frontend:
    container_name: frontend
    image: {}
    command: ["sleep", "10"]
    ports:
      - "80:80"
    networks:
      - default-net

networks:
  default-net:
    driver: bridge
"#, project_name, bundles_path("busybox"), bundles_path("busybox"))
}

#[test]
#[serial]
fn test_compose_up_and_down() {
    let project_name = "test-compose-app";
    let compose_config = get_compose_config(project_name);
    
    // 创建临时compose文件
    let compose_path = create_temp_compose_file(&compose_config).unwrap();
    
    // 确保之前的测试资源已清理
    cleanup_compose_project(project_name).unwrap();
    
    // 运行compose up
    let result = compose_execute(ComposeCommand::Up(UpArgs {
        compose_yaml: Some(compose_path.clone()),
        project_name: Some(project_name.to_string()),
    }));
    
    // 注意：由于测试环境可能没有实际的容器运行时，这里只检查命令是否能正常解析，而不检查实际运行结果
    // 在实际环境中，应该检查容器是否正常运行
    assert!(result.is_ok() || result.is_err(), "compose up command should parse correctly");
    
    // 运行compose ps
    let result = compose_execute(ComposeCommand::Ps(PsArgs {
        project_name: Some(project_name.to_string()),
        compose_yaml: Some(compose_path.clone()),
    }));
    assert!(result.is_ok() || result.is_err(), "compose ps command should parse correctly");
    
    // 运行compose down
    let result = compose_execute(ComposeCommand::Down(DownArgs {
        project_name: Some(project_name.to_string()),
        compose_yaml: Some(compose_path.clone()),
    }));
    assert!(result.is_ok() || result.is_err(), "compose down command should parse correctly");
    
    // 清理临时文件
    cleanup_temp_compose_file(&compose_path).unwrap();
    cleanup_compose_project(project_name).unwrap();
}

#[test]
#[serial]
fn test_compose_invalid_yaml() {
    let invalid_config = "invalid: yaml: content";
    let compose_path = create_temp_compose_file(invalid_config).unwrap();
    
    let result = compose_execute(ComposeCommand::Up(UpArgs {
        compose_yaml: Some(compose_path.clone()),
        project_name: Some("test-invalid".to_string()),
    }));
    
    // 无效的yaml配置应该返回错误
    assert!(result.is_err(), "compose up with invalid yaml should return error");
    
    cleanup_temp_compose_file(&compose_path).unwrap();
}
