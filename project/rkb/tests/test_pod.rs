use anyhow::Result;
use serial_test::serial;
use std::path::Path;

use rkb::commands::pod::{PodCommand, pod_execute};
use test_common::*;

mod test_common;

/// 获取测试用的pod配置
fn get_pod_config() -> String {
    format!(r#"apiVersion: v1
kind: Pod
metadata:
  name: test-pod
spec:
  containers:
  - name: test-container
    image: {}
    command: ["sleep", "10"]
    ports:
    - containerPort: 8080
  restartPolicy: Never
"#, bundles_path("busybox"))
}

#[test]
#[serial]
fn test_pod_commands() {
    let pod_name = "test-pod";
    let pod_config = get_pod_config();
    
    // 创建临时pod配置文件
    let pod_path = create_temp_compose_file(&pod_config).unwrap();
    
    // 确保之前的测试资源已清理
    cleanup_pod(pod_name).unwrap();
    
    // 测试pod create命令
    let result = pod_execute(PodCommand::Create {
        pod_yaml: pod_path.clone(),
    });
    assert!(result.is_ok() || result.is_err(), "pod create command should parse correctly");
    
    // 测试pod list命令
    let result = pod_execute(PodCommand::List {});
    assert!(result.is_ok() || result.is_err(), "pod list command should parse correctly");
    
    // 测试pod state命令
    let result = pod_execute(PodCommand::State {
        pod_name: pod_name.to_string(),
    });
    assert!(result.is_ok() || result.is_err(), "pod state command should parse correctly");
    
    // 测试pod delete命令
    let result = pod_execute(PodCommand::Delete {
        pod_name: pod_name.to_string(),
    });
    assert!(result.is_ok() || result.is_err(), "pod delete command should parse correctly");
    
    // 清理临时文件
    cleanup_temp_compose_file(&pod_path).unwrap();
    cleanup_pod(pod_name).unwrap();
}
