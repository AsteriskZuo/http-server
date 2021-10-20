use http_server_lib::make_server;
use std::process::exit;

#[cfg(feature = "dhat-profiling")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat-profiling")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[tokio::main]
async fn main() {
    #[cfg(feature = "dhat-profiling")]
    let _dhat = Dhat::start_heap_profiling();

    use std::env;

    let old_work_dir = env::current_dir().expect("old_work_dir");
    let exe = env::current_exe().expect("exe");
    let exe_dir = exe.parent().expect("exe_dir");
    assert!(env::set_current_dir(&exe_dir).is_ok());
    println!(
        "Successfully changed working directory to new:{}, old:{}, exe:{}",
        exe_dir.display(),
        old_work_dir.display(),
        exe.display()
    );

    match make_server() {
        Ok(server) => {
            server.run().await;
        }
        Err(error) => {
            eprint!("{:?}", error);
            exit(1);
        }
    }
}
