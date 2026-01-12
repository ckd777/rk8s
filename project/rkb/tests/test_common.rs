use std::{env, fs, path::Path};
use anyhow::{anyhow, Result};

/// 获取测试用的busybox镜像路径
pub fn bundles_path(image_name: &str) -> String {
    // 在实际测试环境中，需要替换为真实的镜像路径
    format!("test/bundles/{}", image_name)
}

/// 创建临时的compose配置文件
pub fn create_temp_compose_file(content: &str) -> Result<String> {
    let temp_dir = env::temp_dir();
    let compose_path = temp_dir.join("test-compose.yaml");
    fs::write(&compose_path, content)?;
    Ok(compose_path.to_str().unwrap().to_string())
}

/// 清理临时的compose配置文件
pub fn cleanup_temp_compose_file(path: &str) -> Result<()> {
    let path = Path::new(path);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// 清理compose项目
pub fn cleanup_compose_project(project_name: &str) -> Result<()> {
    let compose_dir = Path::new("/run/youki/compose").join(project_name);
    if compose_dir.exists() {
        fs::remove_dir_all(compose_dir)?;
    }
    Ok(())
}

/// 清理容器
pub fn cleanup_container(container_id: &str) -> Result<()> {
    let container_dir = Path::new("/run/youki").join(container_id);
    if container_dir.exists() {
        fs::remove_dir_all(container_dir)?;
    }
    Ok(())
}

/// 清理卷
pub fn cleanup_volume(volume_name: &str) -> Result<()> {
    let volume_dir = Path::new("/var/lib/rkl/volumes").join(volume_name);
    if volume_dir.exists() {
        fs::remove_dir_all(volume_dir)?;
    }
    Ok(())
}

/// 清理pod
pub fn cleanup_pod(pod_name: &str) -> Result<()> {
    let pod_dir = Path::new("/run/youki/pods").join(pod_name);
    if pod_dir.exists() {
        fs::remove_dir_all(pod_dir)?;
    }
    Ok(())
}
