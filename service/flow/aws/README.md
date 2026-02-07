# Harana Actions - AWS Module

Consolidated AWS (Amazon Web Services) actions for the Harana framework.

## Services

This module provides actions for the following AWS services:

- **`acm`** - AWS Certificate Manager for SSL/TLS certificates
- **`autoscaling`** - Auto Scaling group management and policies
- **`cloudfront`** - CloudFront CDN distribution management
- **`ec2`** - Elastic Compute Cloud instance and resource management
- **`ecr`** - Elastic Container Registry for Docker images
- **`ecs`** - Elastic Container Service for container orchestration
- **`elb`** - Elastic Load Balancing (Application/Network Load Balancers)
- **`eventbridge`** - EventBridge event bus and rules management
- **`iam`** - Identity and Access Management users, roles, and policies
- **`lambda`** - Lambda serverless function management and invocation
- **`rds`** - Relational Database Service database management
- **`route53`** - Route 53 DNS management and hosted zones
- **`s3`** - Simple Storage Service object storage
- **`secrets`** - Secrets Manager for secure credential storage
- **`ses`** - Simple Email Service for email sending
- **`sqs`** - Simple Queue Service message queuing
- **`sts`** - Security Token Service for temporary credentials
- **`vpc`** - Virtual Private Cloud network configuration

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
harana-actions-aws = "0.1.0"
```

Example usage:

```rust
use harana_actions_aws::s3;
use harana_actions_aws::lambda;

// Use S3 actions
let result = s3::list_buckets(None, None).await?;

// Use Lambda actions
let result = lambda::invoke_function(
    None,
    "my-function".to_string(),
    Some(payload),
    None,
    None
).await?;
```

## Authentication

AWS actions use the standard AWS credential chain:
1. Environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
2. AWS credentials file (`~/.aws/credentials`)
3. IAM instance profile (when running on EC2)
4. ECS task role (when running in ECS)

## Configuration

Most actions accept an optional `region` parameter. If not provided, the region is determined from:
1. The `AWS_REGION` or `AWS_DEFAULT_REGION` environment variable
2. The AWS config file (`~/.aws/config`)
3. Defaults to `us-east-1`

## Development

Build the module:
```bash
cargo build -p harana-actions-aws
```

Run tests:
```bash
cargo test -p harana-actions-aws
```

Check for errors:
```bash
cargo check -p harana-actions-aws
```

## Migration

This module consolidates the previous individual AWS service packages. See [MIGRATION.md](MIGRATION.md) for details.
