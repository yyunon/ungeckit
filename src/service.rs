use log::*;
use tokio::net::unix::pipe::Receiver;
use std::io::{self};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::process::Command;
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

use crate::utils::error::GeckError;

pub struct Context {
    pub handle: Handle,
    pub runtime: Option<Runtime>,
}
impl Context {
    pub fn new() -> Arc<Mutex<Self>> {
        let (handle, runtime) = Self::get_current_runtime();
        Arc::new(Mutex::new(Self {
            handle: handle,
            runtime: runtime,
        }))
    }

    pub fn get_current_runtime() -> (Handle, Option<Runtime>) {
        match Handle::try_current() {
            Ok(handle) => {
                debug!("Getting current runtime");
                (handle, None)
            }
            Err(_) => {
                debug!("Creating a runtime as there is no runtime...");
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                (rt.handle().clone(), Some(rt))
            }
        }
    }

    pub fn abort_if_exists(task: &Option<JoinHandle<Result<(), io::Error>>>) {
        // TODO assert!(self.geckodriver_service.as_mut().unwrap().await.unwrap_err().is_cancelled()); Can run on async
        if let Some(t) = task {
            info!("Found a task handle, so closing it...");
            t.abort();
        }
    }
}
pub struct Service {
    driver_path: String,
    context: Arc<Mutex<Context>>,
		channel: (mpsc::Sender<String>, mpsc::Receiver<String>), // TODO: See if logs can be handled via this silently
    geckodriver_service: Option<JoinHandle<Result<(), io::Error>>>,
    stdout_service: Option<JoinHandle<Result<(), io::Error>>>,
    stderr_service: Option<JoinHandle<Result<(), io::Error>>>,
}
impl Service {
    pub fn new(context: &Arc<Mutex<Context>>, driver_path: &String) -> Self {
        Self {
            driver_path: driver_path.clone(),
            context: context.clone(),
						channel: mpsc::channel(32),
            geckodriver_service: None,
            stdout_service: None,
            stderr_service: None,
        }
    }

    pub async fn start_async(
        &mut self,
        args: Vec<&'static str>,
    ) -> std::result::Result<(), GeckError> {
        let d_path = self.driver_path.clone();
        let c_args = args.clone();
        let mut command = Command::new(d_path);
        let mut output = command
            .args(c_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to spawn process geckodriver!");

        let stdout = output.stdout.take().expect("Failed to take stdout!");
        let stderr = output.stderr.take().expect("Failed to take stderr!");

        // A task to to spawn to retrieve command output
				let send_channel = self.channel.0.clone();
        self.stdout_service = Some(tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Some(line) = reader.next_line().await? {
								if line.contains("Listening on") {
									send_channel.send(line.clone()).await.unwrap();
								}
                debug!("{}", line)
            }
            Ok(())
        }));

        // A task to to spawn to retrieve command output
        self.stderr_service = Some(tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Some(line) = reader.next_line().await? {
                error!("{}", line)
            }
            Ok(())
        }));

        // A task to to spawn to run the command
        self.geckodriver_service = Some(tokio::spawn(async move {
            let status = output.wait().await.expect("Failed to run geckodriver!");
            if status.success() {
                Ok(())
            } else {
                // Since this is the main task we panic in case task failures!
                panic!("Driver service failed with status {}", status)
            }
        }));
        debug!("Spawned driver service...");
        Ok(())
    }

    pub fn start(&mut self, args: Vec<&'static str>) -> std::result::Result<(), GeckError> {
        let d_path = self.driver_path.clone();
        let c_args = args.clone();
        self.context.lock().unwrap().handle.block_on(async {
            let mut command = Command::new(d_path);
            let mut output = command
                .args(c_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .kill_on_drop(true)
                .spawn()
                .expect("Failed to spawn process geckodriver!");

            let stdout = output.stdout.take().expect("Failed to take stdout!");
            let stderr = output.stderr.take().expect("Failed to take stderr!");

            // A task to to spawn to retrieve command output
						let sender = self.channel.0.clone();
            self.stdout_service = Some(tokio::spawn(async move {
                let mut reader = BufReader::new(stdout).lines();
                while let Some(line) = reader.next_line().await? {
                    debug!("{}", line);
										if line.contains("INFO") || line.contains("TRACE") {
											sender.send(line.clone()).await.unwrap();
										}
                }
                Ok(())
            }));

            // A task to to spawn to retrieve command output
            self.stderr_service = Some(tokio::spawn(async move {
                let mut reader = BufReader::new(stderr).lines();
                while let Some(line) = reader.next_line().await? {
                    error!("{}", line)
                }
                Ok(())
            }));

            // A task to to spawn to run the command
            self.geckodriver_service = Some(tokio::spawn(async move {
                let status = output.wait().await.expect("Failed to run geckodriver!");
                if status.success() {
                    Ok(())
                } else {
                    // Since this is the main task we panic in case task failures!
                    panic!("Driver service failed with status {}", status)
                }
            }));
        });
        debug!("Spawned driver service...");
        Ok(())
    }

		pub fn session_is_up(&mut self) -> Result<bool, GeckError>{
			Ok(self.channel.1.blocking_recv().unwrap().contains("Listening on"))
		}
}

impl Drop for Service {
    fn drop(&mut self) {
        Context::abort_if_exists(&self.geckodriver_service);
        Context::abort_if_exists(&self.stdout_service);
        Context::abort_if_exists(&self.stderr_service);
    }
}
