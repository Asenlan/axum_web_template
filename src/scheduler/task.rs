use crate::error::AppError;
use std::time::Duration;
use tokio::time;
use tokio_util::sync::CancellationToken;

// 定时任务实现
pub async fn start_timer(cancel_token: CancellationToken) {
    let task = tokio::spawn({
        let token = cancel_token.clone();
        async move {
            let mut interval = time::interval(Duration::from_secs(3600));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = execute_scheduled_task().await {
                            eprintln!("定时任务执行失败: {:?}", e);
                        }
                    }
                    _ = token.cancelled() => {
                        println!("收到取消信号，开始优雅关闭");
                        break;
                    }
                }
            }

            // 清理资源
            cleanup_resources().await;
            println!("定时任务已完全关闭");
        }
    });

    // 可选的：自动关闭任务（30秒后）
    // tokio::spawn({
    //     let token = cancel_token.clone();
    //     async move {
    //         time::sleep(Duration::from_secs(30)).await;
    //         println!("发送自动关闭信号");
    //         token.cancel();
    //     }
    // });

    // 等待任务结束（放在AppState中保持生命周期）
    let _ = task.await;
}
async fn execute_scheduled_task() -> Result<(), AppError> {
    println!(
        "定时任务执行: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    Ok(())
}
// 清理函数
async fn cleanup_resources() {
    println!("清理数据库连接和其他资源...");
    tokio::time::sleep(Duration::from_secs(1)).await;
}
