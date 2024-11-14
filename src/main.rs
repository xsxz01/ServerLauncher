use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use clap::{Parser, Subcommand};
use rhai::{Engine, Func};
#[derive(Subcommand)]
pub enum Action {
    Start,
    Stop,
    Restart,
}
/// 服务端管理脚本
#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    /// 执行服务端动作，可选值：start, stop, restart
    #[command(subcommand)]
    pub action: Option<Action>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(act) = cli.action {
        match act {
            Action::Start => {
                start();
            },
            Action::Stop => stop(),
            Action::Restart => restart(),
        }
    }
}

fn get_engine() -> Engine {
    let mut engine = Engine::new();
    // 注册运行脚本的函数
    engine.register_fn("run_script", run_script_rhai);
    engine
}

fn start() {
    // 读取文件lua/main.rhai
    let mut main_script_file = fs::File::open("./lua/main.rhai").expect("无法打开lua/main.rhai");
    let mut main_script_content = String::new();
    main_script_file.read_to_string(&mut main_script_content).expect("无法读取lua/main.rhai");
    // 获取引擎
    let engine = get_engine();
    // 获取start函数
    let start_fun_result = Func::<(),()>::create_from_script(
        engine,
        main_script_content.as_str(),
        "start"
    );
    // 执行start函数
    match start_fun_result {
        Ok(start_fun) => {
            start_fun().expect("启动命令执行失败");
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }
    // run_script(PathBuf::from("/data/BDZC/LogServer/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/CenterServer/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/YiWanLogServer01/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/YiWanLogServer02/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/YiWanLogServer03/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/YiWanLogServer04/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/YiWanLogServer05/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/RechargeServer/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/RouterServer10/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/SeasonServer10/"), Some("start.sh"), Some(""));
    // run_script(PathBuf::from("/data/BDZC/gmsv1001/"), Some("game.sh"), Some("start"));
}

fn stop() {
    run_script(PathBuf::from("/data/BDZC/LogServer/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/CenterServer/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/YiWanLogServer01/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/YiWanLogServer02/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/YiWanLogServer03/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/YiWanLogServer04/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/YiWanLogServer05/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/RechargeServer/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/RouterServer10/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/SeasonServer10/"), Some("stop.sh"), Some(""));
    run_script(PathBuf::from("/data/BDZC/gmsv1001/"), Some("game.sh"), Some("stop"));
}

fn restart() {
    stop();
    // 延时5s
    std::thread::sleep(std::time::Duration::from_secs(5));
    start();
}

fn run_script_rhai(path_buf: &str, script: &str, param: &str) {
    run_script(PathBuf::from(path_buf), Some(script), Some(param))
}

fn run_script(path_buf: PathBuf, script: Option<&str>, param: Option<&str>) {
    // 检查是否为root用户（在类Unix系统下）
    let whoami_result = Command::new("whoami")
        .output()
        .expect("无法执行whoami命令");

    let whoami_output = String::from_utf8_lossy(&whoami_result.stdout);
    let is_root = whoami_output.trim() == "root";

    if is_root {
        // 已经是root用户，直接执行命令
        let script_command = Command::new("bash")
            .current_dir(path_buf.clone())
            .arg(script.unwrap_or("./start.sh"))
            .arg(param.unwrap_or(""))
            .status()
            .expect(format!("无法执行sudo命令: {}, 在目录: {}", script.unwrap_or("./start.sh"), path_buf.display()).as_str());
        if script_command.success() {
            println!("执行目录{}的脚本{}成功", path_buf.display(), script.unwrap_or("start.sh"));
        }else {
            println!("执行目录{}的脚本{}失败", path_buf.display(), script.unwrap_or("start.sh"));
        }
    } else {
        // 不是root用户，尝试使用sudo获取权限执行命令
        let target_script = format!("./{}", script.unwrap_or("start.sh"));
        let target_param = param.unwrap_or("").to_string();

        let script_command = Command::new("sudo")
            .current_dir(path_buf.clone())
            .arg("bash")
            .arg(target_script.clone())
            .arg(target_param.clone())
            .status()
            .expect(format!("无法执行sudo命令: {}, 在目录: {}", target_script.clone(), path_buf.display()).as_str());
        if script_command.success() {
            println!("{}", format!("管理员执行目录{}的脚本{}成功", path_buf.display(), target_script.clone()));
        }else {
            println!("{}", format!("管理员执行目录{}的脚本{}失败", path_buf.display(), target_script.clone()));
        }
    }
}