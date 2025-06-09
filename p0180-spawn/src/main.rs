/// サブプロセスの起動のデモ。
/// このプログラムは、サブプロセスとして/usr/bin/lsblkを起動する。
use std::process::Command;

fn main() {
    // サブプロセスを起動する
    let mut child = Command::new("/usr/bin/lsblk")
        .spawn()
        .expect("lsblk should run successfully");

    // サブプロセスの終了を待つ
    let _ = child
        .wait()
        .expect("Child process should exit successfully");
}
