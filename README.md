# kintone-rs

[![Crates.io](https://img.shields.io/crates/v/kintone.svg)](https://crates.io/crates/kintone)
[![Documentation](https://docs.rs/kintone/badge.svg)](https://docs.rs/kintone)

**DISCLAIMER**: this OSS is my own personal work and does not have any relationship with Cybozu Inc. or any other organization which I belong to.

**WARNING**: This library is under development and is likely to undergo incompatible changes in the future.

A client library of Kintone REST APIs for Rust.

## Quick Start

```rust
use kintone::client::{Auth, KintoneClient};

// Create a client
let client = KintoneClient::new(
    "https://your-domain.cybozu.com",
    Auth::api_token("your-api-token")
);

// Get a record
let response = kintone::v1::record::get_record(app_id, record_id)
    .send(&client)?;

// Print the record
for (field_code, field_value) in response.record.fields() {
    println!("{}: {:?}", field_code, field_value);
}
```

For detailed documentation, installation instructions, and usage examples, please refer to the [API documentation](https://docs.rs/kintone).

## Middleware Support

kintone-rs supports a middleware system for handling cross-cutting concerns like retries, logging, and authentication. Middleware layers can be easily composed to add functionality to your Kintone client.

### Available Middleware

- **RetryLayer**: Automatically retries failed requests with exponential backoff
- **LoggingLayer**: Logs HTTP request and response information for debugging
- **BasicAuthLayer**: Adds HTTP Basic authentication headers

### Example: Retry

```rust
use std::time::Duration;
use kintone::client::{Auth, KintoneClient};
use kintone::middleware;

let client = KintoneClient::builder(
        "https://your-domain.cybozu.com",
        Auth::api_token("your-api-token")
    )
    .layer(middleware::RetryLayer::new())
    .build();
```

## Examples

You can find runnable examples in the `examples` directory.

The examples require environment variables to be set:

```bash
export KINTONE_BASE_URL=https://your-domain.cybozu.com
export KINTONE_API_TOKEN=your-api-token
cargo run --example get_record
```

## API Support Status

Implementation status of the [kintone REST APIs](https://cybozu.dev/ja/kintone/docs/rest-api/). Function paths are relative to `kintone::v1`.

- ✅ Implemented: 22
- ⚠️ Partially implemented: 4
- ❌ Not implemented: 58

APIs that exist for both the live and the preview environment are listed by their live endpoint.
Guest spaces are supported for all implemented APIs via `KintoneClientBuilder::guest_space_id`.

### Record

| Status | API | Method | Endpoint | Function | Notes |
|:---:|---|---|---|---|---|
| ✅ | [Get Record](https://cybozu.dev/ja/id/ca1c4e932b47856d3eb1a819/) | GET | `/k/v1/record.json` | `record::get_record` |  |
| ✅ | [Add Record](https://cybozu.dev/ja/id/e7f3125797f78aaec154b918/) | POST | `/k/v1/record.json` | `record::add_record` |  |
| ✅ | [Update Record](https://cybozu.dev/ja/id/607d2f57a86f03ca70588ab8/) | PUT | `/k/v1/record.json` | `record::update_record` |  |
| ✅ | [Get Records](https://cybozu.dev/ja/id/ba1703e9391653c667ce958b/) | GET | `/k/v1/records.json` | `record::get_records` |  |
| ✅ | [Add Records](https://cybozu.dev/ja/id/5cf0698779c8854753d5b61f/) | POST | `/k/v1/records.json` | `record::add_records` |  |
| ✅ | [Update Records](https://cybozu.dev/ja/id/0cc38cb334db46c7413c667c/) | PUT | `/k/v1/records.json` | `record::update_records` |  |
| ✅ | [Delete Records](https://cybozu.dev/ja/id/3e09998aa236ef52743338f9/) | DELETE | `/k/v1/records.json` | `record::delete_records` |  |
| ✅ | [Create Cursor](https://cybozu.dev/ja/id/4e59634133e0f320b35f8711/) | POST | `/k/v1/records/cursor.json` | `record::create_cursor` |  |
| ✅ | [Get Records by Cursor](https://cybozu.dev/ja/id/22f3376b9638cc7d94b8ad57/) | GET | `/k/v1/records/cursor.json` | `record::get_records_by_cursor` |  |
| ✅ | [Delete Cursor](https://cybozu.dev/ja/id/f2b87ac0eafb5c6fad2fdbfb/) | DELETE | `/k/v1/records/cursor.json` | `record::delete_cursor` |  |
| ⚠️ | [Get Comments](https://cybozu.dev/ja/id/b74703885198a01fe8a948ba/) | GET | `/k/v1/record/comments.json` | `record::get_comments` | Response model does not match the docs (`creator` is mapped as `user`; string `id` is parsed as a number) |
| ✅ | [Add Comment](https://cybozu.dev/ja/id/45dbf35056918f6a13549f70/) | POST | `/k/v1/record/comment.json` | `record::add_comment` |  |
| ✅ | [Delete Comment](https://cybozu.dev/ja/id/bd7c8ed72e071bc0ed6731cd/) | DELETE | `/k/v1/record/comment.json` | `record::delete_comment` |  |
| ✅ | [Update Assignees](https://cybozu.dev/ja/id/709d819774fd8c4e0960c1a7/) | PUT | `/k/v1/record/assignees.json` | `record::update_assignees` |  |
| ✅ | [Update Status](https://cybozu.dev/ja/id/ff1f30dda4461d5bb807af27/) | PUT | `/k/v1/record/status.json` | `record::update_status` |  |
| ❌ | [Update Statuses](https://cybozu.dev/ja/id/044d255131483eaf4fe66756/) | PUT | `/k/v1/records/status.json` |  |  |
| ⚠️ | [Bulk Request](https://cybozu.dev/ja/id/bc41b4cb11864e868abd2eb2/) | POST | `/k/v1/bulkRequest.json` | `record::bulk_request` | Update Statuses cannot be included; results are untyped `serde_json::Value` |
| ❌ | [Evaluate Record Permissions](https://cybozu.dev/ja/id/e5b4dc5768ba266a21dd2964/) | GET | `/k/v1/records/acl/evaluate.json` |  |  |

### File

| Status | API | Method | Endpoint | Function | Notes |
|:---:|---|---|---|---|---|
| ✅ | [Download File](https://cybozu.dev/ja/id/c5da2ff7d17ed3b5764a4a3f/) | GET | `/k/v1/file.json` | `file::download` |  |
| ✅ | [Upload File](https://cybozu.dev/ja/id/bb1225dd8d192245c3645a95/) | POST | `/k/v1/file.json` | `file::upload` |  |

### App

| Status | API | Method | Endpoint | Function | Notes |
|:---:|---|---|---|---|---|
| ❌ | [Get App](https://cybozu.dev/ja/id/f9f95e788d8e6cdbc42da01f/) | GET | `/k/v1/app.json` |  |  |
| ✅ | [Get Apps](https://cybozu.dev/ja/id/bc9738cfc60c75502dedfc20/) | GET | `/k/v1/apps.json` | `app::get_apps` |  |
| ✅ | [Add App (preview)](https://cybozu.dev/ja/id/be316bcfaa096d7943d4e669/) | POST | `/k/v1/preview/app.json` | `app::add_app` |  |
| ❌ | [Get App Statistics](https://cybozu.dev/ja/id/87b9a4d7093e6c78b1dff270/) | GET | `/k/v1/apps/statistics.json` |  |  |
| ❌ | [Get App Admin Notes](https://cybozu.dev/ja/id/6b336db6f0404cf2cb2a0056/) | GET | `/k/v1/app/adminNotes.json` |  |  |
| ❌ | [Update App Admin Notes](https://cybozu.dev/ja/id/36e66a8eaed28aab195bcc45/) | PUT | `/k/v1/preview/app/adminNotes.json` |  |  |
| ❌ | [Get Form Fields](https://cybozu.dev/ja/id/27655375153b121d98533774/) | GET | `/k/v1/app/form/fields.json` |  |  |
| ⚠️ | [Add Form Fields](https://cybozu.dev/ja/id/cdeb42b35921e6d4f137208f/) | POST | `/k/v1/preview/app/form/fields.json` | `app::form::add_form_field` | Lookup fields and `FUNCTION` entities in default values are not supported |
| ❌ | [Update Form Fields](https://cybozu.dev/ja/id/624e263fbf459af64cfb3d35/) | PUT | `/k/v1/preview/app/form/fields.json` |  |  |
| ❌ | [Delete Form Fields](https://cybozu.dev/ja/id/7217d82119f8399506fb6f25/) | DELETE | `/k/v1/preview/app/form/fields.json` |  |  |
| ❌ | [Get Form Layout](https://cybozu.dev/ja/id/c519d299340cf1606c0611cb/) | GET | `/k/v1/app/form/layout.json` |  |  |
| ❌ | [Update Form Layout](https://cybozu.dev/ja/id/ab3a01794c08a2e60573e6ba/) | PUT | `/k/v1/preview/app/form/layout.json` |  |  |
| ❌ | [Get Form Design Info](https://cybozu.dev/ja/id/2c5dd59423a6a3ba11dd063e/) | GET | `/k/v1/form.json` |  |  |
| ❌ | [Get Views](https://cybozu.dev/ja/id/c4c5befe425032f6c36b9c4b/) | GET | `/k/v1/app/views.json` |  |  |
| ❌ | [Update Views](https://cybozu.dev/ja/id/578dd5e90f674cd4e6075c17/) | PUT | `/k/v1/preview/app/views.json` |  |  |
| ❌ | [Get Graphs](https://cybozu.dev/ja/id/33ebaf1dd8e1ec09c22aa938/) | GET | `/k/v1/app/reports.json` |  |  |
| ❌ | [Update Graphs](https://cybozu.dev/ja/id/651618fce78670408b1931fb/) | PUT | `/k/v1/preview/app/reports.json` |  |  |
| ❌ | [Get General Settings](https://cybozu.dev/ja/id/8728eba2a728ae1dd0af3fec/) | GET | `/k/v1/app/settings.json` |  |  |
| ❌ | [Update General Settings](https://cybozu.dev/ja/id/65b118066af4efd01f103de5/) | PUT | `/k/v1/preview/app/settings.json` |  |  |
| ❌ | [Get Process Management Settings](https://cybozu.dev/ja/id/b74914ae9d60fb454d48fb07/) | GET | `/k/v1/app/status.json` |  |  |
| ❌ | [Update Process Management Settings](https://cybozu.dev/ja/id/3bcd41df582e8d4a8f5de017/) | PUT | `/k/v1/preview/app/status.json` |  |  |
| ✅ | [Get App Deploy Status](https://cybozu.dev/ja/id/d87cdb0253f91784ecb74a09/) | GET | `/k/v1/preview/app/deploy.json` | `app::settings::get_app_deploy_status` |  |
| ✅ | [Deploy App Settings](https://cybozu.dev/ja/id/11f6926af40fdc7907b1e3a3/) | POST | `/k/v1/preview/app/deploy.json` | `app::settings::deploy_app` |  |
| ❌ | [Get App Plugins](https://cybozu.dev/ja/id/f6f52cc42fbddf4ec1a2d783/) | GET | `/k/v1/app/plugins.json` |  |  |
| ❌ | [Add App Plugins](https://cybozu.dev/ja/id/43c9f463d7eab7285396f1a1/) | POST | `/k/v1/preview/app/plugins.json` |  |  |
| ❌ | [Get JavaScript and CSS Customization Settings](https://cybozu.dev/ja/id/9b499a476dafed45a6f62dd3/) | GET | `/k/v1/app/customize.json` |  |  |
| ❌ | [Update JavaScript and CSS Customization Settings](https://cybozu.dev/ja/id/f954b1b1daf7728c3db7f9a2/) | PUT | `/k/v1/preview/app/customize.json` |  |  |
| ❌ | [Get General Notification Settings](https://cybozu.dev/ja/id/1b335d06a15cb14a63de6c31/) | GET | `/k/v1/app/notifications/general.json` |  |  |
| ❌ | [Update General Notification Settings](https://cybozu.dev/ja/id/07dce8a40d8f19b443ece685/) | PUT | `/k/v1/preview/app/notifications/general.json` |  |  |
| ❌ | [Get Per Record Notification Settings](https://cybozu.dev/ja/id/c97b592ff7123b33f612e511/) | GET | `/k/v1/app/notifications/perRecord.json` |  |  |
| ❌ | [Update Per Record Notification Settings](https://cybozu.dev/ja/id/3897bdf9cc4e7e827ec144bd/) | PUT | `/k/v1/preview/app/notifications/perRecord.json` |  |  |
| ❌ | [Get Reminder Notification Settings](https://cybozu.dev/ja/id/1d8e27cf54cafeea137c9c9a/) | GET | `/k/v1/app/notifications/reminder.json` |  |  |
| ❌ | [Update Reminder Notification Settings](https://cybozu.dev/ja/id/40cd28137a44cc3967aafeb7/) | PUT | `/k/v1/preview/app/notifications/reminder.json` |  |  |
| ❌ | [Get App Permissions](https://cybozu.dev/ja/id/44f492901695be0e04995639/) | GET | `/k/v1/app/acl.json` |  |  |
| ❌ | [Update App Permissions](https://cybozu.dev/ja/id/44b696f92ed866eeaedbfda6/) | PUT | `/k/v1/app/acl.json` |  |  |
| ❌ | [Get Record Permissions](https://cybozu.dev/ja/id/a477d376052721249caa89ac/) | GET | `/k/v1/record/acl.json` |  |  |
| ❌ | [Update Record Permissions](https://cybozu.dev/ja/id/8146d61095ad5e89160e4483/) | PUT | `/k/v1/record/acl.json` |  |  |
| ❌ | [Get Field Permissions](https://cybozu.dev/ja/id/b5b686dfcf2f8c131c28ed6a/) | GET | `/k/v1/field/acl.json` |  |  |
| ❌ | [Update Field Permissions](https://cybozu.dev/ja/id/942e8542b200b829f8b3bac7/) | PUT | `/k/v1/field/acl.json` |  |  |
| ❌ | [Get Action Settings](https://cybozu.dev/ja/id/f7d1613d42dd28f38e7d9ae9/) | GET | `/k/v1/app/actions.json` |  |  |
| ❌ | [Update Action Settings](https://cybozu.dev/ja/id/1d8ac4b88d5a12468c0665f7/) | PUT | `/k/v1/preview/app/actions.json` |  |  |
| ❌ | [Move App to Another Space](https://cybozu.dev/ja/id/ed4007715721805df315ed47/) | POST | `/k/v1/app/move.json` |  |  |

### Space

| Status | API | Method | Endpoint | Function | Notes |
|:---:|---|---|---|---|---|
| ❌ | [Get Space](https://cybozu.dev/ja/id/6e2a365943e303fbb30f01de/) | GET | `/k/v1/space.json` |  |  |
| ❌ | [Update Space](https://cybozu.dev/ja/id/d85a81945868db8c288a3b11/) | PUT | `/k/v1/space.json` |  |  |
| ❌ | [Add Space from Template](https://cybozu.dev/ja/id/d8ea4eaea0b2fc619ba7c782/) | POST | `/k/v1/template/space.json` |  | `space::add_space` uses the API Lab endpoint `POST /k/v1/space.json` instead |
| ✅ | [Delete Space](https://cybozu.dev/ja/id/f64a35f9aad6459095e4dc17/) | DELETE | `/k/v1/space.json` | `space::delete_space` |  |
| ❌ | [Update Space Body](https://cybozu.dev/ja/id/cf580126da2b465115ded1e6/) | PUT | `/k/v1/space/body.json` |  |  |
| ❌ | [Get Space Members](https://cybozu.dev/ja/id/febe6f4eb7efa2f2c77f583d/) | GET | `/k/v1/space/members.json` |  |  |
| ❌ | [Update Space Members](https://cybozu.dev/ja/id/9227d39612c7a78c9161ad53/) | PUT | `/k/v1/space/members.json` |  |  |
| ❌ | [Get Space Statistics](https://cybozu.dev/ja/id/8d2d6e955eea07f32b8af62d/) | GET | `/k/v1/space/statistics.json` |  |  |
| ✅ | [Add Thread](https://cybozu.dev/ja/id/24a2ad8d0fb574f3617e1b92/) | POST | `/k/v1/space/thread.json` | `space::add_thread` |  |
| ❌ | [Update Thread](https://cybozu.dev/ja/id/030c46e9b685a5eee19637f0/) | PUT | `/k/v1/space/thread.json` |  |  |
| ⚠️ | [Add Thread Comment](https://cybozu.dev/ja/id/bcacb3ceff7039b39222ebb4/) | POST | `/k/v1/space/thread/comment.json` | `space::add_thread_comment` | File attachments do not work (`fileKey` is serialized as `file_key`) |
| ❌ | [Add Guest Users](https://cybozu.dev/ja/id/29318cb3da48dfeb065b884d/) | POST | `/k/v1/guests.json` |  |  |
| ❌ | [Delete Guest Users](https://cybozu.dev/ja/id/943e420d94eb3d018df68d28/) | DELETE | `/k/v1/guests.json` |  |  |
| ❌ | [Update Guest Members](https://cybozu.dev/ja/id/712234897391d75133c3d464/) | PUT | `/k/guest/{GUEST_SPACE_ID}/v1/space/guests.json` |  |  |

### Plugin

| Status | API | Method | Endpoint | Function | Notes |
|:---:|---|---|---|---|---|
| ❌ | [Get Installed Plugins](https://cybozu.dev/ja/id/ace5777d9375efed42500278/) | GET | `/k/v1/plugins.json` |  |  |
| ❌ | [Get Required Plugins](https://cybozu.dev/ja/id/56524ae67a377d756e329bd2/) | GET | `/k/v1/plugins/required.json` |  |  |
| ❌ | [Get Apps Using a Plugin](https://cybozu.dev/ja/id/d756f228bad9ca044866aed0/) | GET | `/k/v1/plugin/apps.json` |  |  |
| ❌ | [Import Plugin](https://cybozu.dev/ja/id/6806ccee84420bfaf17ae74f/) | POST | `/k/v1/plugin.json` |  |  |
| ❌ | [Update Plugin](https://cybozu.dev/ja/id/4c0e93986f6a7c08033d874f/) | PUT | `/k/v1/plugin.json` |  |  |
| ❌ | [Uninstall Plugin](https://cybozu.dev/ja/id/8c2031cc1081afd160007699/) | DELETE | `/k/v1/plugin.json` |  |  |

### API Info

| Status | API | Method | Endpoint | Function | Notes |
|:---:|---|---|---|---|---|
| ❌ | [Get API List](https://cybozu.dev/ja/id/b9eb79547d5b2eb184d56b7f/) | GET | `/k/v1/apis.json` |  |  |
| ❌ | [Get API Schema](https://cybozu.dev/ja/id/0e51c5da54396a7a916f10b5/) | GET | `/k/v1/apis/*.json` |  |  |

Additionally, `space::add_space` implements the experimental [API Lab](https://cybozu.dev/ja/kintone/docs/api-lab/rest-api/spaces/add-space-by-name/) endpoint `POST /k/v1/space.json`, which is not part of the official API list.
