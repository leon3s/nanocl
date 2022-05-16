use docker_api::Docker;

#[cfg(unix)]
pub fn new_docker() -> Result<Docker, ()> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(not(unix))]
pub fn new_docker() -> Result<Docker, ()> {
    Docker::new("tcp://127.0.0.1:8080")
}
