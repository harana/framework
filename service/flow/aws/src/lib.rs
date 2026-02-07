// Harana Actions - AWS Module
//
// This module provides AWS (Amazon Web Services) actions organized by service:
//
// - `acm` - AWS Certificate Manager (ACM) for SSL/TLS certificates
// - `autoscaling` - Auto Scaling group management and policies
// - `cloudfront` - CloudFront CDN distribution management
// - `ec2` - Elastic Compute Cloud (EC2) instance and resource management
// - `ecr` - Elastic Container Registry (ECR) for Docker images
// - `ecs` - Elastic Container Service (ECS) for container orchestration
// - `elb` - Elastic Load Balancing (Application/Network Load Balancers)
// - `eventbridge` - EventBridge event bus and rules management
// - `iam` - Identity and Access Management (IAM) users, roles, and policies
// - `lambda` - Lambda serverless function management and invocation
// - `rds` - Relational Database Service (RDS) database management
// - `route53` - Route 53 DNS management and hosted zones
// - `s3` - Simple Storage Service (S3) object storage
// - `secrets` - Secrets Manager for secure credential storage
// - `ses` - Simple Email Service (SES) for email sending
// - `sqs` - Simple Queue Service (SQS) message queuing
// - `sts` - Security Token Service (STS) for temporary credentials
// - `vpc` - Virtual Private Cloud (VPC) network configuration

pub mod acm;
pub mod autoscaling;
pub mod cloudfront;
pub mod ec2;
pub mod ecr;
pub mod ecs;
pub mod elb;
pub mod eventbridge;
pub mod iam;
pub mod lambda;
pub mod rds;
pub mod route53;
pub mod s3;
pub mod secrets;
pub mod ses;
pub mod sqs;
pub mod sts;
pub mod vpc;
