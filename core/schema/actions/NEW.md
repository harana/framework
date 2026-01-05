# Suggested New Actions

## Authentication & Authorization

### `oauth.yml`
- **exchange_code** - Exchange OAuth authorization code for tokens
- **refresh_token** - Refresh an expired access token
- **revoke_token** - Revoke an access token or refresh token
- **validate_token** - Validate and decode a JWT or OAuth token
- **generate_pkce** - Generate PKCE code verifier and challenge

### `totp.yml` (Time-based One-Time Password)
- **generate_secret** - Generate a TOTP secret for 2FA setup
- **generate_qr** - Generate QR code for authenticator app
- **verify_code** - Verify a TOTP code
- **generate_backup_codes** - Generate backup recovery codes

### `captcha.yml`
- **verify_recaptcha** - Verify Google reCAPTCHA response
- **verify_hcaptcha** - Verify hCaptcha response
- **verify_turnstile** - Verify Cloudflare Turnstile response

---

## Content & Media

### `pdf.yml`
- **generate** - Generate PDF from HTML/template
- **merge** - Merge multiple PDFs into one
- **split** - Split PDF into multiple files
- **extract_text** - Extract text content from PDF
- **add_watermark** - Add watermark to PDF pages
- **compress** - Compress PDF file size

### `video.yml`
- **transcode** - Transcode video to different format
- **generate_thumbnail** - Generate thumbnail from video
- **extract_audio** - Extract audio track from video
- **get_metadata** - Get video metadata (duration, resolution, etc.)
- **trim** - Trim video to specific duration

### `audio.yml`
- **transcode** - Convert audio format
- **transcribe** - Transcribe audio to text (speech-to-text)
- **synthesize** - Generate audio from text (text-to-speech)
- **get_metadata** - Get audio metadata

### `qr.yml`
- **generate** - Generate QR code from data
- **decode** - Decode QR code from image
- **generate_with_logo** - Generate QR code with embedded logo

### `barcode.yml`
- **generate** - Generate barcode (Code128, EAN, UPC, etc.)
- **decode** - Decode barcode from image

---

## Communication & Notifications

### `sms.yml`
- **send** - Send SMS message
- **send_bulk** - Send SMS to multiple recipients
- **verify_phone** - Send verification code to phone
- **check_verification** - Check phone verification code
- **get_status** - Get SMS delivery status

### `voice.yml`
- **call** - Initiate a voice call
- **send_voicemail** - Send voicemail message
- **text_to_speech_call** - Make a call with TTS message

### `chat.yml`
- **send_message** - Send chat message
- **broadcast** - Broadcast message to channel/room
- **create_room** - Create chat room
- **join_room** - Join existing chat room
- **leave_room** - Leave chat room

---

## Data Processing

### `csv.yml`
- **parse** - Parse CSV to structured data
- **generate** - Generate CSV from data
- **transform** - Transform CSV columns/rows
- **validate** - Validate CSV against schema

### `excel.yml`
- **read** - Read Excel file to data
- **write** - Write data to Excel file
- **get_sheets** - List sheets in workbook
- **read_sheet** - Read specific sheet

### `xml.yml`
- **parse** - Parse XML to object
- **generate** - Generate XML from object
- **validate** - Validate XML against XSD schema
- **xpath_query** - Query XML with XPath

### `json.yml`
- **parse** - Parse JSON string to object
- **stringify** - Convert object to JSON string
- **validate** - Validate JSON against JSON Schema
- **jmespath_query** - Query JSON with JMESPath
- **merge** - Deep merge multiple JSON objects
- **diff** - Compare two JSON objects

### `html.yml`
- **parse** - Parse HTML to DOM
- **sanitize** - Sanitize HTML (remove XSS vectors)
- **extract_text** - Extract text content from HTML
- **css_select** - Query HTML with CSS selectors
- **minify** - Minify HTML content

### `markdown.yml`
- **to_html** - Convert Markdown to HTML
- **from_html** - Convert HTML to Markdown
- **extract_frontmatter** - Extract YAML frontmatter

### `url.yml`
- **parse** - Parse URL to components
- **build** - Build URL from components
- **encode** - URL encode string
- **decode** - URL decode string
- **shorten** - Create short URL
- **expand** - Expand short URL to original
- **validate** - Validate URL format

### `uuid.yml`
- **generate_v4** - Generate UUID v4 (random)
- **generate_v7** - Generate UUID v7 (time-ordered)
- **validate** - Validate UUID format
- **parse** - Parse UUID to components

### `address.yml`
- **validate** - Validate postal address
- **normalize** - Normalize/standardize address format
- **parse** - Parse address string to components
- **autocomplete** - Get address autocomplete suggestions

### `geocode.yml`
- **forward** - Convert address to coordinates
- **reverse** - Convert coordinates to address
- **distance** - Calculate distance between two points
- **route** - Get route between two points
- **timezone** - Get timezone for coordinates
---

## Payment & Commerce

### `payment.yml` (generic payment processing)
- **create_checkout** - Create checkout session
- **capture_payment** - Capture authorized payment
- **refund** - Process refund
- **get_payment_status** - Get payment status

### `invoice.yml`
- **generate** - Generate invoice PDF
- **send** - Send invoice to customer
- **mark_paid** - Mark invoice as paid
- **calculate_tax** - Calculate tax for invoice items

### `subscription.yml`
- **create** - Create subscription
- **cancel** - Cancel subscription
- **pause** - Pause subscription
- **resume** - Resume paused subscription
- **upgrade** - Upgrade subscription plan
- **downgrade** - Downgrade subscription plan

---

## Location & Geography

### `url.yml`
- **parse** - Parse URL to components
- **build** - Build URL from components
- **encode** - URL encode string
- **decode** - URL decode string
- **shorten** - Create short URL
- **expand** - Expand short URL to original
- **validate** - Validate URL format

### `uuid.yml`
- **generate_v4** - Generate UUID v4 (random)
- **generate_v7** - Generate UUID v7 (time-ordered)
- **validate** - Validate UUID format
- **parse** - Parse UUID to components

### `address.yml`
- **validate** - Validate postal address
- **normalize** - Normalize/standardize address format
- **parse** - Parse address string to components
- **autocomplete** - Get address autocomplete suggestions

### `geocode.yml`
- **forward** - Convert address to coordinates
- **reverse** - Convert coordinates to address
- **distance** - Calculate distance between two points
- **route** - Get route between two points
- **timezone** - Get timezone for coordinates

---

## Security & Compliance

### `hash.yml`
- **generate** - Generate hash (SHA256, SHA512, MD5, etc.)
- **verify** - Verify hash matches content
- **hmac** - Generate HMAC signature

### `password.yml`
- **hash** - Hash password (bcrypt, argon2)
- **verify** - Verify password against hash
- **generate** - Generate random secure password
- **check_strength** - Check password strength
- **check_breach** - Check if password is in known breaches

### `rate_limit.yml`
- **check** - Check if rate limit exceeded
- **increment** - Increment rate limit counter
- **reset** - Reset rate limit for key
- **get_remaining** - Get remaining requests

### `sanitize.yml`
- **html** - Sanitize HTML input
- **sql** - Escape SQL special characters
- **filename** - Sanitize filename
- **url** - Sanitize URL

### `audit.yml`
- **log_event** - Log audit event
- **query_events** - Query audit log
- **export_report** - Export audit report

---

## Integration & External Services

### `openai.yml`
- **chat_completion** - Generate chat completion
- **embedding** - Generate text embeddings
- **moderate** - Check content moderation

### `anthropic.yml`
- **message** - Send message to Claude
- **stream_message** - Stream message response

### `google.yml`
- **translate** - Translate text
- **detect_language** - Detect text language
- **analyze_sentiment** - Analyze text sentiment

### `twitter.yml`
- **post_tweet** - Post a tweet
- **get_user** - Get user profile
- **search_tweets** - Search tweets

### `discord.yml`
- **send_message** - Send message to channel
- **send_webhook** - Send webhook message
- **create_embed** - Create rich embed message

### `telegram.yml`
- **send_message** - Send Telegram message
- **send_photo** - Send photo message
- **send_document** - Send document

---

## Utility & Helpers

### `url.yml`
- **parse** - Parse URL to components
- **build** - Build URL from components
- **encode** - URL encode string
- **decode** - URL decode string
- **shorten** - Create short URL
- **expand** - Expand short URL to original
- **validate** - Validate URL format

### `uuid.yml`
- **generate_v4** - Generate UUID v4 (random)
- **generate_v7** - Generate UUID v7 (time-ordered)
- **validate** - Validate UUID format
- **parse** - Parse UUID to components

### `slug.yml`
- **generate** - Generate URL-friendly slug from text
- **unique** - Generate unique slug (with suffix if exists)

### `currency.yml`
- **convert** - Convert between currencies
- **format** - Format currency for display
- **get_rate** - Get exchange rate

### `unit.yml`
- **convert** - Convert between units (length, weight, temp, etc.)

### `diff.yml`
- **text** - Generate text diff
- **patch** - Apply diff patch
- **three_way_merge** - Three-way merge

### `compress.yml`
- **gzip** - Gzip compress data
- **gunzip** - Gzip decompress data
- **deflate** - Deflate compress
- **inflate** - Inflate decompress

### `base64.yml`
- **encode** - Base64 encode
- **decode** - Base64 decode

### `color.yml`
- **convert** - Convert color formats (hex, rgb, hsl)
- **generate_palette** - Generate color palette
- **contrast_ratio** - Calculate contrast ratio

---

## Analytics & Tracking

### `analytics.yml`
- **track_event** - Track custom event
- **track_pageview** - Track page view
- **identify_user** - Identify user for tracking
- **get_stats** - Get aggregated statistics

### `ab_test.yml`
- **get_variant** - Get A/B test variant for user
- **track_conversion** - Track conversion for variant
- **get_results** - Get test results

---

## DevOps & Infrastructure

### `dns.yml`
- **lookup** - DNS lookup
- **reverse_lookup** - Reverse DNS lookup
- **check_propagation** - Check DNS propagation

### `ssl.yml`
- **check_certificate** - Check SSL certificate validity
- **get_certificate_info** - Get certificate details
- **generate_csr** - Generate certificate signing request

### `docker.yml`
- **build_image** - Build Docker image
- **push_image** - Push image to registry
- **pull_image** - Pull image from registry
- **run_container** - Run container
- **stop_container** - Stop running container

---

## E-commerce Specific

### `inventory.yml`
- **check_stock** - Check stock availability
- **reserve_stock** - Reserve stock for order
- **release_stock** - Release reserved stock
- **update_stock** - Update stock quantity

### `shipping.yml`
- **get_rates** - Get shipping rates
- **create_label** - Create shipping label
- **track** - Track shipment
- **validate_address** - Validate shipping address

### `cart.yml`
- **add_item** - Add item to cart
- **remove_item** - Remove item from cart
- **update_quantity** - Update item quantity
- **apply_coupon** - Apply coupon/discount code
- **calculate_total** - Calculate cart total

---

## Document Generation

### `template.yml`
- **render** - Render document template
- **merge_data** - Merge data into template
- **generate_batch** - Generate batch documents

### `report.yml`
- **generate** - Generate report
- **schedule** - Schedule recurring report
- **export** - Export report (PDF, Excel, CSV)

---

## Real-time & Async

### `sse.yml`
- **publish** - Publish Server-Sent Event
- **broadcast** - Broadcast to all connections

### `websocket.yml`
- **send** - Send WebSocket message
- **broadcast** - Broadcast to room/channel
- **close_connection** - Close WebSocket connection
