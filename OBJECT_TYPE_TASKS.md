# Object Type Conversion Tasks

This document tracks all fields with type `object` that need to be converted to specific class types.

## Completed Files

- [x] **github.yml** - `author`, `committer`, `commit`, `content`, `config` → `Author`, `Committer`, `Commit`, `FileContent`, `WebhookConfig`
- [x] **address.yml** - `address` → `Address`
- [x] **color.yml** - `components`, `hsl`, `rgb`, `hsv`, `cmyk` → `ColorComponents`, `HslColor`, `RgbColor`, `HsvColor`, `CmykColor`
- [x] **aws_sts.yml** - `tags`, `assumed_role_user`, `credentials`, `federated_user` → `Tags`, `AssumedRoleUser`, `StsCredentials`, `FederatedUser`
- [x] **http_client.yml** - `headers`, `query_params`, `variables` → `Headers`, `QueryParams`, `Variables`
- [x] **log.yml** - `context`, `error`, `data` → `LogContext`, `LogError`, `LogData`
- [x] **memgraph.yml** - `properties`, `node`, `parameters`, `summary`, `stats` → `NodeProperties`, `GraphNode`, `QueryParameters`, `QuerySummary`, `GraphStats`

## No Object Types (No Changes Needed)

- [x] **compress.yml** - No object types
- [x] **crypto.yml** - No object types  
- [x] **date.yml** - No object types
- [x] **file.yml** - No object types
- [x] **image.yml** - No object types
- [x] **math.yml** - No object types
- [x] **random.yml** - No object types
- [x] **uuid.yml** - No object types
- [x] **http_response.yaml** - No object types

## Pending Files

### AWS Services

- [ ] **aws_cloudfront.yml**
  - `default_cache_behavior` → `CacheBehavior`
  - `tags` → `Tags`
  - `parameters_in_cache_key_and_forwarded_to_origin` → `CacheKeyParameters`
  - `parameters_in_cache_key` → `CacheKeyParameters`
  - `headers_config` → `HeadersConfig`
  - `cookies_config` → `CookiesConfig`
  - `query_strings_config` → `QueryStringsConfig`
  - `cors_config` → `CorsConfig`
  - `security_headers_config` → `SecurityHeadersConfig`
  - `custom_headers_config` → `CustomHeadersConfig`
  - `distribution_config` → `DistributionConfig`

- [ ] **aws_ecr.yml**
  - `tags` → `Tags`
  - `filter` → `ImageFilter`
  - `image` → `EcrImage`
  - `image_id` → `ImageId`
  - `image_scan_status` → `ImageScanStatus`

- [ ] **aws_eventbridge.yml**
  - `tags` → `Tags`
  - `condition` → `RuleCondition`
  - `destination` → `EventDestination`

- [ ] **aws_iam.yml**
  - `tags` → `Tags`
  - `permissions_boundary` → `PermissionsBoundary`
  - `summary_map` → `AccountSummary`

- [ ] **aws_s3.yml**
  - `tags` → `Tags`
  - `metadata` → `ObjectMetadata`

- [ ] **aws_secrets.yml**
  - `tags` → `Tags`
  - `rotation_rules` → `RotationRules`
  - `kms_key_ids` → `KmsKeyIds`

- [ ] **aws_ses.yml**
  - `tags` → `Tags`
  - `default_tags` → `DefaultTags`
  - `verification_attributes` → `VerificationAttributes`
  - `policies` → `IdentityPolicies`
  - `dkim_attributes` → `DkimAttributes`

- [ ] **aws_sqs.yml**
  - `attributes` → `QueueAttributes`
  - `tags` → `Tags`
  - `message_attributes` → `MessageAttributes`

### Google Services

- [ ] **google_gemini.yml**
  - `usage` → `TokenUsage`
  - `temperature` → `TemperatureConfig`
  - `top_k` → `TopKConfig`
  - `top_p` → `TopPConfig`
  - `schema` → `ResponseSchema`
  - `data` → `StructuredData`

- [ ] **google_pubsub.yml**
  - `labels` → `Labels`
  - `schema_settings` → `SchemaSettings`
  - `attributes` → `MessageAttributes`
  - `dead_letter_policy` → `DeadLetterPolicy`
  - `push_config` → `PushConfig`
  - `retry_policy` → `RetryPolicy`

### Data & Documents

- [ ] **json.yml**
  - `schema` → `JsonSchema`
  - `result` → `JsonResult`
  - `added` → `DiffChanges`
  - `changed` → `DiffChanges`
  - `removed` → `DiffChanges`

- [ ] **xml.yml**
  - `result` → `XmlResult`
  - `data` → `XmlData`
  - `namespaces` → `XmlNamespaces`

- [ ] **html.yml**
  - `result` → `HtmlResult`
  - `allowed_attributes` → `AllowedAttributes`
  - `head` → `HtmlHead`
  - `body` → `HtmlBody`

- [ ] **csv.yml**
  - `schema` → `CsvSchema`

- [ ] **markdown.yml**
  - `frontmatter` → `Frontmatter`

- [ ] **pdf.yml**
  - `margin` → `PageMargin`
  - `options` → `PdfOptions`
  - `data` → `TemplateData`
  - `permissions` → `PdfPermissions`
  - `fields` → `FormFields`

- [ ] **text.yml**
  - `data` → `TemplateData`

- [ ] **microsoft_excel.yml**
  - `rows` (in list) → `ExcelRow`
  - `data` (in list) → `ExcelRow`

- [ ] **archive.yml**
  - `files` (in list) → `ArchiveFile`

### Database & Storage

- [ ] **mongodb.yml**
  - `document` → `MongoDocument`
  - `filter` → `MongoFilter`
  - `projection` → `MongoProjection`
  - `sort` → `MongoSort`
  - `update` → `MongoUpdate`
  - `replacement` → `MongoReplacement`
  - `keys` → `IndexKeys`

- [ ] **search.yml**
  - `filters` → `SearchFilters`
  - `document` → `SearchDocument`
  - `query` → `SearchQuery`
  - `mappings` → `IndexMappings`
  - `settings` → `IndexSettings`

- [ ] **cache.yml**
  - `values` → `CacheValues`
  - `entries` → `CacheEntries`

- [ ] **blob.yml**
  - `metadata` → `BlobMetadata`

- [ ] **sql.yml**
  - `out_parameters` → `OutputParameters`

### Application Logic

- [ ] **event.yml**
  - `metadata` → `EventMetadata`

- [ ] **schedule.yml**
  - `metadata` → `ScheduleMetadata`
  - `last_execution` → `ExecutionDetails`
  - `action_config` → `ActionConfig`

- [ ] **workflow.yml**
  - `context` → `WorkflowContext`

- [ ] **form.yml**
  - `data` → `FormData`
  - `metadata` → `FormMetadata`
  - `validation_rules` → `ValidationRules`

- [ ] **validation.yml**
  - `schema` → `ValidationSchema`
  - `allowed_attributes` → `AllowedAttributes`

- [ ] **transform.yml**
  - `data` → `TransformData`
  - `json` → `JsonData`
  - `options` → `TransformOptions`

- [ ] **job.yml**
  - `jobs` (in list) → `Job`

- [ ] **queue.yml**
  - `messages` (in list) → `QueueMessage`

- [ ] **secret.yml**
  - `secrets` (in list) → `Secret`
  - `versions` (in list) → `SecretVersion`

### Authentication & Authorization

- [ ] **identity.yml**
  - `claims` → `TokenClaims`
  - `metadata` → `UserMetadata`

- [ ] **session.yml**
  - `data` → `SessionData`

- [ ] **policy.yml**
  - `conditions` → `PolicyConditions`
  - `context` → `EvaluationContext`
  - `evaluated_conditions` → `EvaluatedConditions`

- [ ] **role.yml**
  - `conditions` → `RoleConditions`

- [ ] **user.yml**
  - `conditions` → `PermissionConditions`

- [ ] **passkey.yml**
  - `authenticator_selection` → `AuthenticatorSelection`
  - `options` → `PasskeyOptions`
  - `response` → `PasskeyResponse`

- [ ] **group.yml**
  - `roles` (in list) → `GroupRole`
  - `members` (in list) → `GroupMember`

### Communication

- [ ] **email.yml**
  - `variables` → `EmailVariables`
  - `attachments` (in list) → `EmailAttachment`
  - `recipients` (in list) → `EmailRecipient`
  - `templates` (in list) → `EmailTemplate`

- [ ] **push.yml**
  - `custom_data` → `CustomData`
  - `data` → `NotificationData`
  - `subscription` → `PushSubscription`
  - `actions` (in list) → `NotificationAction`
  - `failures` (in list) → `DeliveryFailure`

- [ ] **slack.yml**
  - `attachments` (in list) → `SlackAttachment`
  - `blocks` (in list) → `SlackBlock`
  - `channels` (in list) → `SlackChannel`
  - `users` (in list) → `SlackUser`

- [ ] **microsoft_teams.yml**
  - `card` → `AdaptiveCard`
  - `from_user` → `TeamsUser`
  - `attachments` (in list) → `TeamsAttachment`
  - `mentions` (in list) → `TeamsMention`
  - `channels` (in list) → `TeamsChannel`
  - `messages` (in list) → `TeamsMessage`
  - `attendees` (in list) → `TeamsAttendee`
  - `teams` (in list) → `Team`
  - `members` (in list) → `TeamsMember`

### Infrastructure & Monitoring

- [ ] **healthcheck.yml**
  - `details` → `HealthDetails`
  - `metrics` → `HealthMetrics`

- [ ] **metric.yml**
  - `tags` → `MetricTags`
  - `datapoints` (in list) → `MetricDatapoint`
  - `metrics` (in list) → `Metric`

- [ ] **audit.yml**
  - `details` → `AuditDetails`
  - `metadata` → `AuditMetadata`
  - `filters` → `AuditFilters`
  - `conditions` → `AlertConditions`
  - `logs` (in list) → `AuditLog`
  - `activities` (in list) → `Activity`
  - `history` (in list) → `AuditHistory`
  - `alerts` (in list) → `AuditAlert`
  - `statistics` (in list) → `AuditStatistic`

- [ ] **privacy.yml**
  - `metadata` → `ConsentMetadata`
  - `consents` (in list) → `Consent`
  - `history` (in list) → `ConsentHistory`
  - `accesses` (in list) → `DataAccess`

### External Services

- [ ] **stripe.yml**
  - `metadata` → `StripeMetadata`
  - `billing_details` → `BillingDetails`
  - `card` → `CardDetails`
  - `line_items` (in list) → `LineItem`
  - `customers` (in list) → `StripeCustomer`

- [ ] **zoom.yml**
  - `recurrence` → `MeetingRecurrence`
  - `settings` → `MeetingSettings`
  - `custom_questions` (in list) → `CustomQuestion`
  - `registrants` (in list) → `Registrant`
  - `participants` (in list) → `Participant`
  - `meetings` (in list) → `ZoomMeeting`
  - `users` (in list) → `ZoomUser`
  - `recording_files` (in list) → `RecordingFile`
  - `recordings` (in list) → `Recording`

- [ ] **successfactors.yml**
  - `custom_fields` → `CustomFields`
  - `org_chart` → `OrgChart`
  - `employees` (in list) → `Employee`
  - `job_history` (in list) → `JobHistory`
  - `compensation_history` (in list) → `CompensationRecord`
  - `balances` (in list) → `LeaveBalance`
  - `requests` (in list) → `LeaveRequest`
  - `direct_reports` (in list) → `DirectReport`
  - `departments` (in list) → `Department`
  - `positions` (in list) → `Position`

### Geo & Location

- [ ] **geoip.yml**
  - `address` → `GeoAddress`

- [ ] **geolocation.yml**
  - `address` → `GeoAddress`
  - `from` → `GeoPoint`
  - `to` → `GeoPoint`
  - `results` (in list) → `GeoResult`
  - `waypoints` (in list) → `Waypoint`
  - `steps` (in list) → `RouteStep`

### AI & ML

- [ ] **ai.yml**
  - `scores` → `ClassificationScores`
  - `input_data` → `ModelInput`
  - `outputs` → `ModelOutput`
  - `messages` (in list) → `ChatMessage`
  - `entities` (in list) → `Entity`
  - `input_shapes` (in list) → `TensorShape`
  - `output_shapes` (in list) → `TensorShape`
  - `predictions` (in list) → `Prediction`
  - `detections` (in list) → `Detection`

### Feature Management

- [ ] **feature_flag.yml**
  - `context` → `EvaluationContext`
  - `variation` → `FlagVariation`
  - `percentages` → `RolloutPercentages`
  - `variation_counts` → `VariationCounts`
  - `variations` (in list) → `Variation`
  - `flags` (in list) → `FeatureFlag`
  - `conditions` (in list) → `FlagCondition`

### Networking

- [ ] **route.yml**
  - `headers` → `RouteHeaders`
  - `params` → `RouteParams`

- [ ] **url.yml**
  - `query` → `UrlQuery`

- [ ] **webhook.yml**
  - `webhooks` (in list) → `Webhook`
  - `deliveries` (in list) → `WebhookDelivery`

### Calendar

- [ ] **calendar.yml**
  - `reminders` (in list) → `Reminder`
  - `attendees` (in list) → `Attendee`
  - `events` (in list) → `CalendarEvent`
  - `calendars` (in list) → `Calendar`
  - `busy_periods` (in list) → `TimePeriod`
  - `free_periods` (in list) → `TimePeriod`
  - `available_slots` (in list) → `TimeSlot`

---

## How to Complete Tasks

### Goal

Eliminate all generic `object` types in the YAML action files located at:
```
/Users/naden/Developer/harana/framework/core/schema/actions/
```

Each `object` type should be replaced with a specific class type that defines its structure.

### Step-by-Step Instructions

#### Step 1: Identify `object` Types

Search for fields with `object` type in inputs, outputs, or class definitions:
```yaml
# Examples of what to look for:
data: object
metadata: object
config: object #required
items: list[object]
```

#### Step 2: Create a Class Definition

Add a class definition at the end of the YAML file. Classes are defined using this format:

```yaml
- name: ClassName
  class:
    field_name: type
    another_field: type
    optional_field: type
```

**Example - Simple Class:**
```yaml
- name: FormSubmission
  class:
    submission_id: string
    form_id: string
    data: FormData
    status: string
    submitted_at: datetime
    updated_at: datetime
```

**Example - Nested Class:**
```yaml
- name: FormData
  class:
    fields: list[FormField]
    values: map[string, any]

- name: FormField
  class:
    name: string
    type: string
    required: boolean
    value: any
```

#### Step 3: Replace `object` with Class Name

Update all references from `object` to the new class name:

**Before:**
```yaml
- name: Get Form Submission
  action: get_submission
  inputs:
    submission_id: string #required
  outputs:
    data: object           # <-- Generic object
    form_id: string
    status: string
```

**After:**
```yaml
- name: Get Form Submission
  action: get_submission
  inputs:
    submission_id: string #required
  outputs:
    data: FormData         # <-- Specific class
    form_id: string
    status: string
```

#### Step 4: Handle `list[object]` Types

For list types, create a class for the list item and update the type:

**Before:**
```yaml
outputs:
  submissions: list[object]
  errors: list[object]
```

**After:**
```yaml
outputs:
  submissions: list[FormSubmission]
  errors: list[ValidationError]
```

Then add the class definitions:
```yaml
- name: ValidationError
  class:
    field: string
    message: string
    code: string
```

#### Step 5: Test Your Changes

After making changes, run:
```bash
cargo sync-actions sync -f
```

Fix any errors before committing.

### Supported Field Types

| Type | Description | Example |
|------|-------------|---------|
| `string` | Text value | `name: string` |
| `integer` | Whole number | `count: integer` |
| `float` | Decimal number | `price: float` |
| `boolean` | True/false | `enabled: boolean` |
| `datetime` | Date and time | `created_at: datetime` |
| `bytes` | Binary data | `content: bytes` |
| `any` | Any value type | `value: any` |
| `list[T]` | Array of type T | `items: list[string]` |
| `map[K, V]` | Key-value pairs | `data: map[string, any]` |
| `ClassName` | Custom class | `user: User` |
| `list[ClassName]` | Array of class | `users: list[User]` |

### Naming Conventions

1. **Class names**: PascalCase (e.g., `FormSubmission`, `UserMetadata`)
2. **Field names**: snake_case (e.g., `form_id`, `submitted_at`)
3. **Prefix with context**: When a generic term is used, prefix with the domain (e.g., `FormData` not just `Data`, `EventMetadata` not just `Metadata`)

### Common Patterns

#### Pattern 1: Metadata Objects
```yaml
- name: EventMetadata
  class:
    source: string
    timestamp: datetime
    correlation_id: string
    trace_id: string
```

#### Pattern 2: Configuration Objects
```yaml
- name: CacheConfig
  class:
    ttl: integer
    max_size: integer
    eviction_policy: string
```

#### Pattern 3: Filter/Query Objects
```yaml
- name: SearchFilters
  class:
    field: string
    operator: string
    value: any
```

#### Pattern 4: Response/Result Objects
```yaml
- name: ApiResponse
  class:
    status: integer
    message: string
    data: any
    errors: list[ApiError]
```

#### Pattern 5: Credentials/Auth Objects
```yaml
- name: AwsCredentials
  class:
    access_key_id: string
    secret_access_key: string
    session_token: string
    expiration: datetime
```

### Checklist for Each File

- [ ] Identified all `object` type fields
- [ ] Created class definitions for each unique object type
- [ ] Replaced all `object` references with class names
- [ ] Replaced all `list[object]` with `list[ClassName]`
- [ ] Verified class fields match actual data structure
- [ ] Tested with `cargo sync-actions sync -f`

---

## Progress

- **Completed**: 7 files
- **No Changes Needed**: 9 files
- **Pending**: 46 files
- **Total object fields to convert**: ~250+

### Summary by Category

| Category | Files | Status |
|----------|-------|--------|
| AWS Services | 8 | Pending |
| Google Services | 2 | Pending |
| Data & Documents | 9 | Pending |
| Database & Storage | 5 | Pending |
| Application Logic | 10 | Pending |
| Authentication & Authorization | 7 | Pending |
| Communication | 4 | Pending |
| Infrastructure & Monitoring | 4 | Pending |
| External Services | 3 | Pending |
| Geo & Location | 2 | Pending |
| AI & ML | 1 | Pending |
| Feature Management | 1 | Pending |
| Networking | 3 | Pending |
| Calendar | 1 | Pending |
