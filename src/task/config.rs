/*
Basic configuration for the tasks

Author Andrew Evans
*/

/// Task Configuration
pub struct TaskConfig{
    task_name: String,
    reply_to: String,
    correlation_id: String,
    expires: String,
    priority: i8,
    time_limit: i64,
    soft_time_limit: i64,
    eta: String,
    retries: i8,
}
