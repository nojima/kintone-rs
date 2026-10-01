//! Wire tests use examples from the official REST API specification.
//! The mock middleware verifies requests and returns fixtures without network access.

use std::io::Read;

use serde_json::{Value, json};

use super::{apis, app, plugin, record, space};
use crate::client::{Auth, KintoneClient};
use crate::error::ApiError;
use crate::middleware::{Handler, Layer, RequestBody, ResponseBody};

struct Mock {
    method: &'static str,
    path: &'static str,
    query: Vec<(String, String)>,
    body: Option<Value>,
    response: &'static str,
}

impl<H: Handler> Layer<H> for Mock {
    type Outer = Self;
    fn layer(self, _inner: H) -> Self {
        self
    }
}

impl Handler for Mock {
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError> {
        assert_eq!(req.method().as_str(), self.method);
        let url = url::Url::parse(&req.uri().to_string()).unwrap();
        assert_eq!(url.path(), self.path);
        assert_eq!(
            url.query_pairs()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect::<Vec<_>>(),
            self.query
        );
        assert_eq!(req.headers()["x-cybozu-api-token"], "test-token");
        if self.body.is_some() {
            assert_eq!(req.headers()["content-type"], "application/json");
        }
        let mut bytes = Vec::new();
        req.into_body().into_reader().read_to_end(&mut bytes).unwrap();
        match &self.body {
            Some(body) => assert_eq!(&serde_json::from_slice::<Value>(&bytes).unwrap(), body),
            None => assert!(bytes.is_empty()),
        }
        let body = ureq::Body::builder().data(self.response.as_bytes().to_vec());
        Ok(http::Response::new(ResponseBody::from_ureq_body(body)))
    }
}

fn client(
    method: &'static str,
    path: &'static str,
    query: &[(&str, &str)],
    body: Option<Value>,
    response: &'static str,
    guest: bool,
) -> KintoneClient {
    let builder = KintoneClient::builder(
        "https://example.cybozu.com",
        Auth::api_token("test-token".to_owned()),
    );
    let builder = if guest {
        builder.guest_space_id(42)
    } else {
        builder
    };
    builder
        .layer(Mock {
            method,
            path,
            query: query.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            body,
            response,
        })
        .build()
}

macro_rules! check {
    ($request:expr, $method:expr, $path:expr, $query:expr, $body:expr, $reply:expr) => {{
        let client = client($method, $path, $query, $body, $reply, false);
        $request.send(&client).unwrap();
    }};
}

#[test]
fn record_evaluate_record_permissions_wire_format() {
    check!(
        record::evaluate_record_permissions(8).ids([1, 2]),
        "GET",
        "/k/v1/records/acl/evaluate.json",
        &[("app", "8"), ("ids[0]", "1"), ("ids[1]", "2")],
        None,
        include_str!("../testdata/rest-api/evaluate-record-permissions.json")
    );
}

#[test]
fn app_get_app_wire_format() {
    check!(
        app::get_app(8),
        "GET",
        "/k/v1/app.json",
        &[("id", "8")],
        None,
        include_str!("../testdata/rest-api/get-app.json")
    );
}

#[test]
fn app_get_app_statistics_wire_format() {
    check!(
        app::get_app_statistics().offset(1).limit(2),
        "GET",
        "/k/v1/apps/statistics.json",
        &[("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-apps-statistics.json")
    );
}

#[test]
fn app_get_app_admin_notes_wire_format() {
    check!(
        app::get_app_admin_notes(8),
        "GET",
        "/k/v1/app/adminNotes.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-app-admin-notes.json")
    );
}

#[test]
fn app_update_app_admin_notes_wire_format() {
    check!(
        app::update_app_admin_notes(8),
        "PUT",
        "/k/v1/preview/app/adminNotes.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-app-admin-notes.json")
    );
}

#[test]
fn app_form_get_form_fields_wire_format() {
    check!(
        app::form::get_form_fields(8),
        "GET",
        "/k/v1/app/form/fields.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-form-fields.json")
    );
}

#[test]
fn app_form_get_form_layout_wire_format() {
    check!(
        app::form::get_form_layout(8),
        "GET",
        "/k/v1/app/form/layout.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-form-layout.json")
    );
}

#[test]
fn app_form_get_form_wire_format() {
    check!(
        app::form::get_form(8),
        "GET",
        "/k/v1/form.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-form.json")
    );
}

#[test]
fn app_form_update_form_fields_wire_format() {
    check!(
        app::form::update_form_fields(8),
        "PUT",
        "/k/v1/preview/app/form/fields.json",
        &[],
        Some(json!({"app": 8, "properties": {}})),
        include_str!("../testdata/rest-api/update-form-fields.json")
    );
}

#[test]
fn app_form_delete_form_fields_wire_format() {
    check!(
        app::form::delete_form_fields(8),
        "DELETE",
        "/k/v1/preview/app/form/fields.json",
        &[],
        Some(json!({"app": 8, "fields": []})),
        include_str!("../testdata/rest-api/delete-form-fields.json")
    );
}

#[test]
fn app_form_update_form_layout_wire_format() {
    check!(
        app::form::update_form_layout(8),
        "PUT",
        "/k/v1/preview/app/form/layout.json",
        &[],
        Some(json!({"app": 8, "layout": []})),
        include_str!("../testdata/rest-api/update-form-layout.json")
    );
}

#[test]
fn app_view_get_views_wire_format() {
    check!(
        app::view::get_views(8),
        "GET",
        "/k/v1/app/views.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-views.json")
    );
}

#[test]
fn app_view_update_views_wire_format() {
    check!(
        app::view::update_views(8),
        "PUT",
        "/k/v1/preview/app/views.json",
        &[],
        Some(json!({"app": 8, "views": {}})),
        include_str!("../testdata/rest-api/update-views.json")
    );
}

#[test]
fn app_report_get_graph_settings_wire_format() {
    check!(
        app::report::get_graph_settings(8),
        "GET",
        "/k/v1/app/reports.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-graph-settings.json")
    );
}

#[test]
fn app_report_update_graph_settings_wire_format() {
    check!(
        app::report::update_graph_settings(8),
        "PUT",
        "/k/v1/preview/app/reports.json",
        &[],
        Some(json!({"app": 8, "reports": {}})),
        include_str!("../testdata/rest-api/update-graph-settings.json")
    );
}

#[test]
fn app_settings_get_general_settings_wire_format() {
    check!(
        app::settings::get_general_settings(8),
        "GET",
        "/k/v1/app/settings.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-general-settings.json")
    );
}

#[test]
fn app_settings_update_general_settings_wire_format() {
    check!(
        app::settings::update_general_settings(8),
        "PUT",
        "/k/v1/preview/app/settings.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-general-settings.json")
    );
}

#[test]
fn app_settings_get_process_management_settings_wire_format() {
    check!(
        app::settings::get_process_management_settings(8),
        "GET",
        "/k/v1/app/status.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-process-management-settings.json")
    );
}

#[test]
fn app_settings_update_process_management_settings_wire_format() {
    check!(
        app::settings::update_process_management_settings(8),
        "PUT",
        "/k/v1/preview/app/status.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-process-management-settings.json")
    );
}

#[test]
fn app_settings_get_customization_wire_format() {
    check!(
        app::settings::get_customization(8),
        "GET",
        "/k/v1/app/customize.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-customization.json")
    );
}

#[test]
fn app_settings_update_customization_wire_format() {
    check!(
        app::settings::update_customization(8),
        "PUT",
        "/k/v1/preview/app/customize.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-customization.json")
    );
}

#[test]
fn app_settings_get_general_notification_settings_wire_format() {
    check!(
        app::settings::get_general_notification_settings(8),
        "GET",
        "/k/v1/app/notifications/general.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-general-notification-settings.json")
    );
}

#[test]
fn app_settings_update_general_notification_settings_wire_format() {
    check!(
        app::settings::update_general_notification_settings(8),
        "PUT",
        "/k/v1/preview/app/notifications/general.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-general-notification-settings.json")
    );
}

#[test]
fn app_settings_get_per_record_notification_settings_wire_format() {
    check!(
        app::settings::get_per_record_notification_settings(8),
        "GET",
        "/k/v1/app/notifications/perRecord.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-per-record-notification-settings.json")
    );
}

#[test]
fn app_settings_update_per_record_notification_settings_wire_format() {
    check!(
        app::settings::update_per_record_notification_settings(8),
        "PUT",
        "/k/v1/preview/app/notifications/perRecord.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-per-record-notification-settings.json")
    );
}

#[test]
fn app_settings_get_reminder_notification_settings_wire_format() {
    check!(
        app::settings::get_reminder_notification_settings(8),
        "GET",
        "/k/v1/app/notifications/reminder.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-reminder-notification-settings.json")
    );
}

#[test]
fn app_settings_update_reminder_notification_settings_wire_format() {
    check!(
        app::settings::update_reminder_notification_settings(8),
        "PUT",
        "/k/v1/preview/app/notifications/reminder.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-reminder-notification-settings.json")
    );
}

#[test]
fn app_settings_get_app_permissions_wire_format() {
    check!(
        app::settings::get_app_permissions(8),
        "GET",
        "/k/v1/app/acl.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-app-permissions.json")
    );
}

#[test]
fn app_settings_update_app_permissions_wire_format() {
    check!(
        app::settings::update_app_permissions(8),
        "PUT",
        "/k/v1/app/acl.json",
        &[],
        Some(json!({"app": 8, "rights": []})),
        include_str!("../testdata/rest-api/update-app-permissions.json")
    );
}

#[test]
fn app_settings_get_record_permissions_wire_format() {
    check!(
        app::settings::get_record_permissions(8),
        "GET",
        "/k/v1/record/acl.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-record-permissions.json")
    );
}

#[test]
fn app_settings_update_record_permissions_wire_format() {
    check!(
        app::settings::update_record_permissions(8),
        "PUT",
        "/k/v1/record/acl.json",
        &[],
        Some(json!({"app": 8, "rights": []})),
        include_str!("../testdata/rest-api/update-record-permissions.json")
    );
}

#[test]
fn app_settings_get_field_permissions_wire_format() {
    check!(
        app::settings::get_field_permissions(8),
        "GET",
        "/k/v1/field/acl.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-field-permissions.json")
    );
}

#[test]
fn app_settings_update_field_permissions_wire_format() {
    check!(
        app::settings::update_field_permissions(8),
        "PUT",
        "/k/v1/field/acl.json",
        &[],
        Some(json!({"app": 8, "rights": []})),
        include_str!("../testdata/rest-api/update-field-permissions.json")
    );
}

#[test]
fn app_settings_get_action_settings_wire_format() {
    check!(
        app::settings::get_action_settings(8),
        "GET",
        "/k/v1/app/actions.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-action-settings.json")
    );
}

#[test]
fn app_settings_update_action_settings_wire_format() {
    check!(
        app::settings::update_action_settings(8),
        "PUT",
        "/k/v1/preview/app/actions.json",
        &[],
        Some(json!({"app": 8, "actions": {}})),
        include_str!("../testdata/rest-api/update-action-settings.json")
    );
}

#[test]
fn app_settings_get_app_plugins_wire_format() {
    check!(
        app::settings::get_app_plugins(8),
        "GET",
        "/k/v1/app/plugins.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-app-plugins.json")
    );
}

#[test]
fn app_settings_add_app_plugins_wire_format() {
    check!(
        app::settings::add_app_plugins(8),
        "POST",
        "/k/v1/preview/app/plugins.json",
        &[],
        Some(json!({"app": 8, "ids": []})),
        include_str!("../testdata/rest-api/add-app-plugins.json")
    );
}

#[test]
fn app_settings_move_app_wire_format() {
    check!(
        app::settings::move_app(8),
        "POST",
        "/k/v1/app/move.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/move-app.json")
    );
}

#[test]
fn space_get_space_wire_format() {
    check!(
        space::get_space(8),
        "GET",
        "/k/v1/space.json",
        &[("id", "8")],
        None,
        include_str!("../testdata/rest-api/get-space.json")
    );
}

#[test]
fn space_update_space_wire_format() {
    check!(
        space::update_space(8),
        "PUT",
        "/k/v1/space.json",
        &[],
        Some(json!({"id": 8})),
        include_str!("../testdata/rest-api/update-space.json")
    );
}

#[test]
fn space_add_space_from_template_wire_format() {
    check!(
        space::add_space_from_template(8, "Space"),
        "POST",
        "/k/v1/template/space.json",
        &[],
        Some(json!({"id": 8, "name": "Space", "members": []})),
        include_str!("../testdata/rest-api/add-space-from-template.json")
    );
}

#[test]
fn space_update_space_body_wire_format() {
    check!(
        space::update_space_body(8, "<p>Body</p>"),
        "PUT",
        "/k/v1/space/body.json",
        &[],
        Some(json!({"id": 8, "body": "<p>Body</p>"})),
        include_str!("../testdata/rest-api/update-space-body.json")
    );
}

#[test]
fn space_get_space_members_wire_format() {
    check!(
        space::get_space_members(8),
        "GET",
        "/k/v1/space/members.json",
        &[("id", "8")],
        None,
        include_str!("../testdata/rest-api/get-space-members.json")
    );
}

#[test]
fn space_update_space_members_wire_format() {
    check!(
        space::update_space_members(8),
        "PUT",
        "/k/v1/space/members.json",
        &[],
        Some(json!({"id": 8, "members": []})),
        include_str!("../testdata/rest-api/update-space-members.json")
    );
}

#[test]
fn space_get_space_statistics_wire_format() {
    check!(
        space::get_space_statistics().offset(1).limit(2),
        "GET",
        "/k/v1/spaces/statistics.json",
        &[("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-spaces-statistics.json")
    );
}

#[test]
fn space_update_thread_wire_format() {
    check!(
        space::update_thread(8),
        "PUT",
        "/k/v1/space/thread.json",
        &[],
        Some(json!({"id": 8})),
        include_str!("../testdata/rest-api/update-thread.json")
    );
}

#[test]
fn space_add_guest_users_wire_format() {
    check!(
        space::add_guest_users(),
        "POST",
        "/k/v1/guests.json",
        &[],
        Some(json!({"guests": []})),
        include_str!("../testdata/rest-api/add-guests.json")
    );
}

#[test]
fn space_delete_guest_users_wire_format() {
    check!(
        space::delete_guest_users().guests(["user@example.com".to_owned()]),
        "DELETE",
        "/k/v1/guests.json",
        &[],
        Some(json!({"guests": ["user@example.com"]})),
        include_str!("../testdata/rest-api/delete-guests.json")
    );
}

#[test]
fn plugin_get_plugins_wire_format() {
    check!(
        plugin::get_plugins().ids(["plugin-id".to_owned()]).offset(1).limit(2),
        "GET",
        "/k/v1/plugins.json",
        &[("ids[0]", "plugin-id"), ("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-plugins.json")
    );
}

#[test]
fn plugin_get_required_plugins_wire_format() {
    check!(
        plugin::get_required_plugins().offset(1).limit(2),
        "GET",
        "/k/v1/plugins/required.json",
        &[("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-required-plugins.json")
    );
}

#[test]
fn plugin_get_plugin_apps_wire_format() {
    check!(
        plugin::get_plugin_apps("plugin-id"),
        "GET",
        "/k/v1/plugin/apps.json",
        &[("id", "plugin-id")],
        None,
        include_str!("../testdata/rest-api/get-plugin-apps.json")
    );
}

#[test]
fn plugin_add_plugin_wire_format() {
    check!(
        plugin::add_plugin("file-key"),
        "POST",
        "/k/v1/plugin.json",
        &[],
        Some(json!({"fileKey": "file-key"})),
        include_str!("../testdata/rest-api/add-plugin.json")
    );
}

#[test]
fn plugin_update_plugin_wire_format() {
    check!(
        plugin::update_plugin("plugin-id", "file-key"),
        "PUT",
        "/k/v1/plugin.json",
        &[],
        Some(json!({"id": "plugin-id", "fileKey": "file-key"})),
        include_str!("../testdata/rest-api/update-plugin.json")
    );
}

#[test]
fn plugin_delete_plugin_wire_format() {
    check!(
        plugin::delete_plugin("plugin-id"),
        "DELETE",
        "/k/v1/plugin.json",
        &[],
        Some(json!({"id": "plugin-id"})),
        include_str!("../testdata/rest-api/delete-plugin.json")
    );
}

#[test]
fn apis_get_apis_wire_format() {
    check!(
        apis::get_apis(),
        "GET",
        "/k/v1/apis.json",
        &[],
        None,
        include_str!("../testdata/rest-api/get-apis.json")
    );
}

#[test]
fn apis_get_api_schema_wire_format() {
    check!(
        apis::get_api_schema("record/get"),
        "GET",
        "/k/v1/apis/record/get.json",
        &[],
        None,
        include_str!("../testdata/rest-api/get-api-schema.json")
    );
}

#[test]
fn guest_members_use_guest_space_url() {
    let client = client(
        "PUT",
        "/k/guest/42/v1/space/guests.json",
        &[],
        Some(json!({"id":42,"guests":["user@example.com"]})),
        include_str!("../testdata/rest-api/update-guest-members.json"),
        true,
    );
    space::update_guest_members(42)
        .guests(["user@example.com".to_owned()])
        .send(&client)
        .unwrap();
}

#[test]
fn preview_reads_preserve_query_and_guest_space() {
    macro_rules! preview {
        ($request:expr, $path:expr, $reply:expr) => {{
            let client = client("GET", $path, &[("app", "8")], None, $reply, true);
            $request.preview(true).preview(false).preview(true).send(&client).unwrap();
        }};
    }
    preview!(
        app::get_app_admin_notes(8),
        "/k/guest/42/v1/preview/app/adminNotes.json",
        include_str!("../testdata/rest-api/get-app-admin-notes.json")
    );
    preview!(
        app::form::get_form_fields(8),
        "/k/guest/42/v1/preview/app/form/fields.json",
        include_str!("../testdata/rest-api/get-form-fields.json")
    );
    preview!(
        app::form::get_form_layout(8),
        "/k/guest/42/v1/preview/app/form/layout.json",
        include_str!("../testdata/rest-api/get-form-layout.json")
    );
    preview!(
        app::form::get_form(8),
        "/k/guest/42/v1/preview/form.json",
        include_str!("../testdata/rest-api/get-form.json")
    );
    preview!(
        app::view::get_views(8),
        "/k/guest/42/v1/preview/app/views.json",
        include_str!("../testdata/rest-api/get-views.json")
    );
    preview!(
        app::report::get_graph_settings(8),
        "/k/guest/42/v1/preview/app/reports.json",
        include_str!("../testdata/rest-api/get-graph-settings.json")
    );
    preview!(
        app::settings::get_general_settings(8),
        "/k/guest/42/v1/preview/app/settings.json",
        include_str!("../testdata/rest-api/get-general-settings.json")
    );
    preview!(
        app::settings::get_process_management_settings(8),
        "/k/guest/42/v1/preview/app/status.json",
        include_str!("../testdata/rest-api/get-process-management-settings.json")
    );
    preview!(
        app::settings::get_customization(8),
        "/k/guest/42/v1/preview/app/customize.json",
        include_str!("../testdata/rest-api/get-customization.json")
    );
    preview!(
        app::settings::get_general_notification_settings(8),
        "/k/guest/42/v1/preview/app/notifications/general.json",
        include_str!("../testdata/rest-api/get-general-notification-settings.json")
    );
    preview!(
        app::settings::get_per_record_notification_settings(8),
        "/k/guest/42/v1/preview/app/notifications/perRecord.json",
        include_str!("../testdata/rest-api/get-per-record-notification-settings.json")
    );
    preview!(
        app::settings::get_reminder_notification_settings(8),
        "/k/guest/42/v1/preview/app/notifications/reminder.json",
        include_str!("../testdata/rest-api/get-reminder-notification-settings.json")
    );
    preview!(
        app::settings::get_app_permissions(8),
        "/k/guest/42/v1/preview/app/acl.json",
        include_str!("../testdata/rest-api/get-app-permissions.json")
    );
    preview!(
        app::settings::get_record_permissions(8),
        "/k/guest/42/v1/preview/record/acl.json",
        include_str!("../testdata/rest-api/get-record-permissions.json")
    );
    preview!(
        app::settings::get_field_permissions(8),
        "/k/guest/42/v1/preview/field/acl.json",
        include_str!("../testdata/rest-api/get-field-permissions.json")
    );
    preview!(
        app::settings::get_action_settings(8),
        "/k/guest/42/v1/preview/app/actions.json",
        include_str!("../testdata/rest-api/get-action-settings.json")
    );
    preview!(
        app::settings::get_app_plugins(8),
        "/k/guest/42/v1/preview/app/plugins.json",
        include_str!("../testdata/rest-api/get-app-plugins.json")
    );
    let client = client(
        "GET",
        "/k/guest/42/v1/preview/app/form/fields.json",
        &[("app", "8"), ("lang", "ja")],
        None,
        include_str!("../testdata/rest-api/get-form-fields.json"),
        true,
    );
    app::form::get_form_fields(8).lang("ja").preview(true).send(&client).unwrap();
}

#[test]
fn access_control_updates_support_preview_and_revision() {
    macro_rules! preview {
        ($request:expr, $path:expr) => {{
            let client = client("PUT", $path, &[], Some(json!({"app":8,"rights":[],"revision":12})), "{\"revision\":\"13\"}", true);
            let response = $request.preview(true).revision(Some(12)).send(&client).unwrap();
            assert_eq!(response.revision, 13);
        }};
    }
    preview!(app::settings::update_app_permissions(8), "/k/guest/42/v1/preview/app/acl.json");
    preview!(app::settings::update_record_permissions(8), "/k/guest/42/v1/preview/record/acl.json");
    preview!(app::settings::update_field_permissions(8), "/k/guest/42/v1/preview/field/acl.json");
}

#[test]
fn partial_updates_omit_unspecified_properties_and_keep_false_and_empty_arrays() {
    use crate::model::app::field::FieldPropertyUpdate;
    use crate::model::record::FieldType;
    let client = client(
        "PUT",
        "/k/v1/preview/app/settings.json",
        &[],
        Some(json!({"app":8,"enableComments":false,"revision":12})),
        "{\"revision\":\"13\"}",
        false,
    );
    app::settings::update_general_settings(8)
        .enable_comments(false)
        .revision(Some(12))
        .send(&client)
        .unwrap();
    let client = self::client(
        "PUT",
        "/k/v1/preview/app/notifications/general.json",
        &[],
        Some(json!({"app":8,"notifications":[],"notifyToCommenter":false})),
        "{\"revision\":\"13\"}",
        false,
    );
    app::settings::update_general_notification_settings(8)
        .notifications([])
        .notify_to_commenter(false)
        .send(&client)
        .unwrap();
    let mut field = FieldPropertyUpdate::new(FieldType::SingleLineText);
    field.code = Some("new_code".to_owned());
    field.label = Some("New label".to_owned());
    let client = self::client(
        "PUT",
        "/k/v1/preview/app/form/fields.json",
        &[],
        Some(
            json!({"app":8,"properties":{"old_code":{"type":"SINGLE_LINE_TEXT","code":"new_code","label":"New label"}}}),
        ),
        "{\"revision\":\"13\"}",
        false,
    );
    app::form::update_form_fields(8).field("old_code", field).send(&client).unwrap();
    let client = self::client(
        "PUT",
        "/k/v1/preview/app/form/fields.json",
        &[],
        Some(
            json!({"app":8,"properties":{"amount":{"type":"NUMBER","maxValue":"","displayScale":""}}}),
        ),
        "{\"revision\":\"13\"}",
        false,
    );
    app::form::update_form_fields(8)
        .raw_field("amount", json!({"type":"NUMBER","maxValue":"","displayScale":""}))
        .send(&client)
        .unwrap();
}

#[test]
fn file_references_omit_response_metadata() {
    use crate::model::app::settings::{AppIcon, Customization, CustomizationResource};
    use crate::model::file_body;
    let client = client(
        "PUT",
        "/k/v1/preview/app/settings.json",
        &[],
        Some(json!({"app":8,"icon":{"type":"FILE","file":{"fileKey":"icon-file"}}})),
        "{\"revision\":\"13\"}",
        false,
    );
    app::settings::update_general_settings(8)
        .icon(AppIcon::File {
            file: file_body("icon-file").build(),
        })
        .send(&client)
        .unwrap();
    let client = self::client(
        "PUT",
        "/k/v1/preview/app/customize.json",
        &[],
        Some(json!({"app":8,"desktop":{"js":[{"type":"FILE","file":{"fileKey":"js-file"}}]}})),
        "{\"revision\":\"13\"}",
        false,
    );
    app::settings::update_customization(8)
        .desktop(Customization {
            js: Some(vec![CustomizationResource::File {
                file: file_body("js-file").build(),
            }]),
            css: None,
        })
        .send(&client)
        .unwrap();
}

#[test]
fn nullable_settings_and_number_or_string_statistics() {
    let response: app::settings::GetProcessManagementSettingsResponse =
        serde_json::from_value(json!({"enable":false,"states":null,"actions":null,"revision":"1"}))
            .unwrap();
    assert!(response.states.is_none());
    assert!(response.actions.is_none());
    let mut response: Value =
        serde_json::from_str(include_str!("../testdata/rest-api/get-space.json")).unwrap();
    for key in [
        "body",
        "showAnnouncement",
        "showThreadList",
        "showAppList",
        "showMemberList",
        "showRelatedLinkList",
    ] {
        response[key] = Value::Null;
    }
    let response: space::GetSpaceResponse = serde_json::from_value(response).unwrap();
    assert!(response.body.is_none());
    assert!(response.show_announcement.is_none());
    let mut response: Value =
        serde_json::from_str(include_str!("../testdata/rest-api/get-spaces-statistics.json"))
            .unwrap();
    response["spaces"][0]["id"] = json!(42);
    response["spaces"][0]["memberCount"] = json!(3);
    let response: space::GetSpaceStatisticsResponse = serde_json::from_value(response).unwrap();
    assert_eq!(response.spaces[0].id, 42);
    assert_eq!(response.spaces[0].member_count, 3);
}

#[test]
fn lookup_fields_and_function_defaults_are_supported() {
    use crate::model::app::field::{FieldProperty, user_select_field_property};
    use crate::model::{Entity, EntityType};
    let value = json!({"type":"SINGLE_LINE_TEXT","code":"customer","label":"Customer","noLabel":false,"required":true,
        "lookup":{"relatedApp":{"app":"2","code":""},"relatedKeyField":"name","fieldMappings":[],"lookupPickerFields":[],"filterCond":"","sort":"name asc"}});
    let field: FieldProperty = serde_json::from_value(value.clone()).unwrap();
    assert!(matches!(field, FieldProperty::Lookup(_)));
    assert_eq!(field.field_code(), "customer");
    assert_eq!(serde_json::to_value(&field).unwrap(), value);
    let client = client(
        "POST",
        "/k/v1/preview/app/form/fields.json",
        &[],
        Some(json!({"app":"8","properties":{"customer":value},"revision":null})),
        include_str!("../testdata/rest-api/add-form-fields.json"),
        false,
    );
    app::form::add_form_field(8).field(field).send(&client).unwrap();
    let field = user_select_field_property("user")
        .default_value(vec![Entity {
            entity_type: EntityType::FUNCTION,
            code: "LOGINUSER()".to_owned(),
        }])
        .build();
    assert_eq!(serde_json::to_value(field).unwrap()["defaultValue"][0]["type"], "FUNCTION");
    let field: FieldProperty = serde_json::from_value(json!({
        "type":"USER_SELECT", "code":"assignee", "label":"Assignee", "noLabel":false,
        "required":false, "entities":"", "defaultValue":""
    }))
    .unwrap();
    let encoded = serde_json::to_value(field).unwrap();
    assert_eq!(encoded["entities"], json!([]));
    assert_eq!(encoded["defaultValue"], json!([]));
}

#[test]
fn restricted_related_fields_preserve_nulls() {
    use crate::model::app::field::FieldProperty;
    for value in [
        json!({"type":"SINGLE_LINE_TEXT","code":"lookup","label":"Lookup","noLabel":false,"required":false,"lookup":null}),
        json!({"type":"REFERENCE_TABLE","code":"related","label":"Related","noLabel":false,"referenceTable":null}),
    ] {
        let field: FieldProperty = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(field).unwrap(), value);
    }
}

#[test]
fn field_defaults_accept_empty_strings_and_minute_precision() {
    use crate::model::app::field::FieldProperty;
    for (kind, value) in [
        ("DATE", ""),
        ("TIME", ""),
        ("TIME", "09:00"),
        ("DATETIME", ""),
        ("DATETIME", "2012-07-19T00:00Z"),
    ] {
        let field: FieldProperty = serde_json::from_value(json!({"type":kind,"code":"field","label":"Field","noLabel":false,"required":false,"unique":false,"defaultNowValue":false,"defaultValue":value})).unwrap();
        let encoded = serde_json::to_value(field).unwrap();
        assert_eq!(encoded["type"], kind);
        if kind == "TIME" && !value.is_empty() {
            assert_eq!(encoded["defaultValue"], "09:00");
        }
    }
}
