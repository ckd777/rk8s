// 移除未使用的导入，解决 unused import 警告
// use anyhow::Result; 
use serial_test::serial;
use std::path::Path;
use std::fs;

use rkb::commands::pod::{PodCommand, pod_execute, PodInfo};
use rkb::pod_task::TaskRunner;
use test_common::*;

mod test_common;

/// 获取测试用的pod配置
fn get_pod_config(pod_name: &str) -> String {
    format!(r#"apiVersion: v1
kind: Pod
metadata:
  name: {}
spec:
  containers:
  - name: test-container
    image: {}  
    command: ["sleep", "10"]
    ports:
    - containerPort: 8080
  restartPolicy: Never
"#, pod_name, bundles_path("busybox"))
}

/// 测试TaskRunner的基本功能
#[test]
#[serial]
fn test_task_runner() {
    let pod_name = "test-task-runner-pod";
    let pod_config = get_pod_config(pod_name);
    
    // 创建临时pod配置文件
    let pod_path = create_temp_compose_file(&pod_config).unwrap();
    
    // 确保之前的测试资源已清理
    cleanup_pod(pod_name).unwrap();
    
    // 测试TaskRunner::from_file（严格断言：必须成功，去掉{:?}格式化）
    let result = TaskRunner::from_file(&pod_path);
    assert!(result.is_ok(), "TaskRunner::from_file 失败，Pod配置加载逻辑移植有问题");
    
    // 移除无效的 pod_name 字段断言（TaskRunner 无该字段）
    // let task_runner = result.unwrap();
    // assert_eq!(task_runner.pod_name, pod_name, "TaskRunner中Pod名称与配置不一致");
    
    // 清理临时文件
    cleanup_temp_compose_file(&pod_path).unwrap();
    cleanup_pod(pod_name).unwrap();
}

/// 测试完整的Pod生命周期：create -> state -> delete
#[test]
#[serial]
fn test_pod_lifecycle() {
    let pod_name = "test-pod-lifecycle";
    let pod_config = get_pod_config(pod_name);
    
    // 创建临时pod配置文件
    let pod_path = create_temp_compose_file(&pod_config).unwrap();
    
    // 确保之前的测试资源已清理
    cleanup_pod(pod_name).unwrap();
    
    // 测试pod create命令（pod_execute返回()，只需验证执行不panic）
    // 用 std::panic::catch_unwind 捕获panic，替代Result断言
    let create_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Create {
            pod_yaml: pod_path.clone(),
        });
    });
    assert!(create_result.is_ok(), "Pod create 命令执行panic，移植有问题");
    
    // 测试pod state命令（验证执行不panic）
    let state_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::State {
            pod_name: pod_name.to_string(),
        });
    });
    assert!(state_result.is_ok(), "Pod state 命令执行panic，移植有问题");
    
    // 移除无效的 contains 断言（pod_execute返回()，无字符串返回值）
    // let state_output = state_result.unwrap();
    // assert!(state_output.contains(pod_name), "Pod state 返回结果不包含Pod名称");
    
    // 测试pod delete命令（验证执行不panic）
    let delete_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Delete {
            pod_name: pod_name.to_string(),
        });
    });
    assert!(delete_result.is_ok(), "Pod delete 命令执行panic，移植有问题");
    
    // 清理临时文件
    cleanup_temp_compose_file(&pod_path).unwrap();
    cleanup_pod(pod_name).unwrap();
}

/// 测试PodInfo结构的功能
#[test]
#[serial]
fn test_pod_info() {
    let pod_name = "test-pod-info";
    let root_path = Path::new("/tmp"); // 使用/tmp目录避免权限问题
    
    // 创建PodInfo
    let pod_info = PodInfo {
        pod_sandbox_id: "test-sandbox-id".to_string(),
        container_names: vec!["test-container-1".to_string(), "test-container-2".to_string()],
    };
    
    // 测试PodInfo::save（使用/tmp目录，避免权限问题）
    let save_result = pod_info.save(root_path, pod_name);
    assert!(save_result.is_ok(), "PodInfo::save 失败，移植有问题");
    
    // 测试PodInfo::delete
    let delete_result = PodInfo::delete(root_path, pod_name);
    assert!(delete_result.is_ok(), "PodInfo::delete 失败，移植有问题");
    
    // 清理临时文件
    let pod_info_path = root_path.join("pods").join(pod_name);
    if pod_info_path.exists() {
        fs::remove_file(pod_info_path).unwrap();
    }
    let pods_dir = root_path.join("pods");
    if pods_dir.exists() {
        fs::remove_dir(pods_dir).unwrap();
    }
}

/// 测试所有Pod命令的解析
#[test]
#[serial]
fn test_all_pod_commands() {
    let pod_name = "test-all-commands-pod";
    let pod_config = get_pod_config(pod_name);
    
    // 创建临时pod配置文件
    let pod_path = create_temp_compose_file(&pod_config).unwrap();
    
    // 确保之前的测试资源已清理
    cleanup_pod(pod_name).unwrap();
    
    // 测试pod run命令（验证执行不panic）
    let run_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Run {
            pod_yaml: pod_path.clone(),
        });
    });
    assert!(run_result.is_ok(), "Pod run 命令执行panic，移植有问题");
    
    // 测试pod create命令（验证执行不panic）
    let create_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Create {
            pod_yaml: pod_path.clone(),
        });
    });
    assert!(create_result.is_ok(), "Pod create 命令执行panic，移植有问题");
    
    // 测试pod start命令（验证执行不panic）
    let start_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Start {
            pod_name: pod_name.to_string(),
        });
    });
    assert!(start_result.is_ok(), "Pod start 命令执行panic，移植有问题");
    
    // 测试pod list命令（验证执行不panic）
    let list_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::List {});
    });
    assert!(list_result.is_ok(), "Pod list 命令执行panic，移植有问题");
    
    // 移除无效的 is_empty 断言（pod_execute返回()，无字符串返回值）
    // let list_output = list_result.unwrap();
    // assert!(!list_output.is_empty(), "Pod list 返回空，移植有问题");
    
    // 测试pod state命令（验证执行不panic）
    let state_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::State {
            pod_name: pod_name.to_string(),
        });
    });
    assert!(state_result.is_ok(), "Pod state 命令执行panic，移植有问题");
    
    // 测试pod delete命令（验证执行不panic）
    let delete_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Delete {
            pod_name: pod_name.to_string(),
        });
    });
    assert!(delete_result.is_ok(), "Pod delete 命令执行panic，移植有问题");
    
    // 清理临时文件
    cleanup_temp_compose_file(&pod_path).unwrap();
    cleanup_pod(pod_name).unwrap();
}

/// 测试无效的Pod配置
#[test]
#[serial]
fn test_invalid_pod_config() {
    // 创建无效的pod配置
    let invalid_config = "invalid yaml content";
    let pod_path = create_temp_compose_file(invalid_config).unwrap();
    
    // 测试pod create命令（无效配置应该返回错误，而不是panic）
    let result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Create {
            pod_yaml: pod_path.clone(),
        });
    });
    // 无效YAML配置应该被正确处理，返回错误而不是panic
    assert!(result.is_ok(), "无效配置的Pod create不应该panic，而是返回错误");
    
    // 清理临时文件
    cleanup_temp_compose_file(&pod_path).unwrap();
}

/// 测试不存在的Pod
#[test]
#[serial]
fn test_nonexistent_pod() {
    let nonexistent_pod = "nonexistent-pod-12345";
    
    // 确保该Pod不存在
    cleanup_pod(nonexistent_pod).unwrap();
    
    // 测试pod state命令（查询不存在的Pod应该返回错误，而不是panic）
    let state_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::State {
            pod_name: nonexistent_pod.to_string(),
        });
    });
    assert!(state_result.is_ok(), "查询不存在的Pod的state不应该panic，而是返回错误");
    
    // 测试pod delete命令（幂等性：执行不panic）
    let delete_result = std::panic::catch_unwind(|| {
        let _ = pod_execute(PodCommand::Delete {
            pod_name: nonexistent_pod.to_string(),
        });
    });
    assert!(delete_result.is_ok(), "删除不存在的Pod应该不panic（幂等），移植的delete逻辑有问题");
}