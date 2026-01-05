// Harana Actions - AWS SES Module Output Types
// Auto-generated output structs for AWS SES action methods.

use serde::{Deserialize, Serialize};

// send_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailOutput {
    pub message_id: String,
    pub success: bool,
}

// TODO: Add remaining output types - see core/schema/actions/aws_ses.yml
