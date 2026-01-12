use crate::commands::pod::PodInfo;
use crate::commands::{Exec, ExecPod};
use crate::commands::{delete, exec, load_container, start, state};
use anyhow::{Result, anyhow};
use liboci_cli::{Delete, Start, State};
use libruntime::rootpath;
use tracing::{error, info};

use libcontainer::syscall::syscall::create_syscall;

pub fn delete_pod(pod_name: &str) -> Result<(), anyhow::Error> {
    let root_path = rootpath::determine(None, &*create_syscall())?;
    let pod_info = PodInfo::load(&root_path, pod_name)?;
    let container = load_container(root_path.clone(), pod_name)
        .map_err(|e| anyhow!("Failed to load container {}: {}", pod_name, e))?;
    let pid_i32 = container
        .state
        .pid
        .ok_or_else(|| anyhow!("PID not found for container {}", pod_name))?;
    remove_pod_network(pid_i32)?;
    // delete all container
    for container_name in &pod_info.container_names {
        let delete_args = Delete {
            container_id: container_name.clone(),
            force: true,
        };
        let root_path = rootpath::determine(None, &*create_syscall())?;
        if let Err(delete_err) = delete(delete_args, root_path.clone()) {
            error!(
                "Failed to delete container {}: {}",
                container_name, delete_err
            );
        } else {
            info!("Container deleted: {}", container_name);
        }
    }

    // delete pause container
    let delete_args = Delete {
        container_id: pod_info.pod_sandbox_id.clone(),
        force: true,
    };
    let root_path = rootpath::determine(None, &*create_syscall())?;
    if let Err(delete_err) = delete(delete_args, root_path.clone()) {
        error!(
            "Failed to delete PodSandbox {}: {}",
            pod_info.pod_sandbox_id, delete_err
        );
    } else {
        info!("PodSandbox deleted: {}", pod_info.pod_sandbox_id);
    }

    // delete pod file
    PodInfo::delete(&root_path, pod_name)?;
    info!("Pod {} deleted successfully", pod_name);
    Ok(())
}

pub fn remove_pod_network(pid: i32) -> Result<(), anyhow::Error> {
    // TODO: Implement CNI network removal without task::get_cni
    Ok(())
}

pub fn create_pod(pod_yaml: &str) -> Result<(), anyhow::Error> {
    // TODO: Implement create_pod without common::TaskRunner
    Err(anyhow!("Create pod not implemented yet"))
}

pub fn start_pod(pod_name: &str) -> Result<(), anyhow::Error> {
    let root_path = rootpath::determine(None, &*create_syscall())?;
    let pod_info = PodInfo::load(&root_path, pod_name)?;

    if pod_info.container_names.is_empty() {
        return Err(anyhow!("No containers found for Pod {}", pod_name));
    }

    for container_name in &pod_info.container_names {
        let start_args = Start {
            container_id: container_name.clone(),
        };
        start(start_args, root_path.clone())
            .map_err(|e| anyhow!("Failed to start container {}: {}", container_name, e))?;
        info!("Container started: {}", container_name);
    }

    info!("Pod {} started successfully", pod_name);
    Ok(())
}

pub fn state_pod(pod_name: &str) -> Result<(), anyhow::Error> {
    let root_path = rootpath::determine(None, &*create_syscall())?;
    let pod_info = PodInfo::load(&root_path, pod_name)?;

    info!("Pod: {pod_name}");

    info!("PodSandbox ID: {}", pod_info.pod_sandbox_id);
    let _ = state(
        State {
            container_id: pod_info.pod_sandbox_id.clone(),
        },
        root_path.clone(),
    );

    info!("Containers:");
    for container_name in &pod_info.container_names {
        let _container_state = state(
            State {
                container_id: container_name.clone(),
            },
            root_path.clone(),
        );
    }

    Ok(())
}

pub fn exec_pod(args: ExecPod) -> Result<i32> {
    let root_path = rootpath::determine(None, &*create_syscall())?;
    let pod_info_path = root_path.join("pods").join(&args.pod_name);
    if !pod_info_path.exists() {
        return Err(anyhow::anyhow!("Pod {} not found", args.pod_name));
    }
    let args = Exec::from(args);
    let exit_code = exec(args, root_path)?;
    Ok(exit_code)
}
