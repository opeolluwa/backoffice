use backoffice_config::env::AppConfig;
use bullmq::redis_connection::RedisConnection;
use bullmq::worker::{CancellationToken, ProcessorFn};
use bullmq::{Job, Queue, QueueOptions, Worker, WorkerOptions};
use std::sync::Arc;
use tracing::instrument::WithSubscriber;

const DEFAULT_QUEUE_NAME: &str = "my-queue";

async fn run(config: &AppConfig, redis_connection: &RedisConnection) -> bullmq::Result<()> {
    let mut queue_options = QueueOptions::default();
    queue_options.redis_connection = Some(redis_connection);
    let queue = Queue::with_options(DEFAULT_QUEUE_NAME, queue_options)
        .await?
        .with_current_subscriber();

    // Create a worker
    let processor: ProcessorFn = Arc::new(|job: Job, _token: CancellationToken| {
        Box::pin(async move {
            println!("Processing job: {} - {}", job.id(), job.name());
            Ok(serde_json::json!({"sent": true}))
        })
    });

    let worker = Worker::with_options(DEFAULT_QUEUE_NAME, processor, WorkerOptions::default())
        .await?
        .with_current_subscriber();

    // Worker processes jobs automatically...
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // worker.close(5000).await?;
    Ok(())
}

// pub async fn enqueue<T: serde::Serialize>(
//     queue_name: &str,
//     job_name: &str,
//     data: T,
// ) -> bullmq::Result<()> {
//     let queue = Queue::new(queue_name, QueueOptions::default()).await?;
//     queue.add(job_name, serde_json::json!(data), None).await?;
//     Ok(())
// }
