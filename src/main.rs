use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tokio::process::Command;
use std::env;

// シェルスクリプトを非同期で実行する関数
async fn run_shell_script(script_name: String, arg: String) {
    let output = Command::new("sh")
        .arg(script_name)
        .arg(&arg)
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            println!("Output for '{}': {}", &arg, String::from_utf8_lossy(&output.stdout));
        }
        Ok(output) => {
            eprintln!("Error for '{}': {}", &arg, String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Failed to execute '{}': {:?}", &arg, e);
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // コマンドライン引数の取得
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <shell_script>", args[0]);
        return Ok(());
    }

    // シェルスクリプト名を取得（String型でコピー）
    let script_name = args[1].clone();

    // 読み込むファイルのパス
    let path = Path::new("args.txt");

    // ファイルを開く
    let file = File::open(&path)?;

    // BufReaderを使用してファイルを行ごとに読み込む
    let reader = io::BufReader::new(file);

    let mut handles = vec![];

    // 各行を非同期で処理
    for line in reader.lines() {
        let line_content = line?;

        // 非同期でシェルスクリプトを実行するタスクを生成
        let handle = tokio::spawn(run_shell_script(script_name.clone(), line_content));

        handles.push(handle);
    }

    // 全てのタスクが終了するのを待つ
    for handle in handles {
        handle.await?;
    }

    Ok(())
}