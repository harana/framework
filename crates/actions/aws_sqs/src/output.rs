// Harana Actions - AWS SQS Module Output Types
// Auto-generated output structs for AWS SQS action methods.

use serde::{Deserialize, Serialize};

// create_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueueOutput {
    pub queue_url: String,
    pub success: bool,
}

// delete_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQueueOutput {
    pub success: bool,
}

// send_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    pub md5_of_message_attributes: String,
    pub md5_of_message_body: String,
    pub message_id: String,
    pub sequence_number: String,
    pub success: bool,
}

// TODO: Add remaining output types - see core/schema/actions/aws_sqs.yml
