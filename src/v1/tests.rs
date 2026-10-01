//! Wire tests use examples from the official REST API specification.
//! The mock middleware verifies requests and returns fixtures without network access.

use std::io::Read;

use serde_json::{Value, json};

use super::{apis, app, plugin, record, space};
use crate::client::{Auth, KintoneClient};
use crate::error::ApiError;
use crate::middleware::{Handler, Layer, RequestBody, ResponseBody};
use crate::model::EntityType;
use crate::model::app::{field::FieldProperty, layout::Layout, report::*, settings::*, view::*};
use crate::model::space::{CreateAppPermission, SpaceCoverType};

struct Mock {
    method: &'static str,
    path: String,
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
    path: &str,
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
            path: path.to_owned(),
            query: query.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            body,
            response,
        })
        .build()
}

macro_rules! check {
    ($request:expr, $method:expr, $path:expr, $query:expr, $body:expr, $reply:expr) => {{
        let client = client($method, $path, $query, $body, $reply, false);
        $request.send(&client).unwrap()
    }};
}

mod populated;

#[test]
fn record_evaluate_record_permissions_wire_format() {
    let response = check!(
        record::evaluate_record_permissions(8).ids([1, 2]),
        "GET",
        "/k/v1/records/acl/evaluate.json",
        &[("app", "8"), ("ids[0]", "1"), ("ids[1]", "2")],
        None,
        include_str!("../testdata/rest-api/evaluate-record-permissions.json")
    );
    assert_eq!(response.rights.len(), 2);
    assert_eq!(response.rights[0].id, 1);
    assert!(response.rights[0].record.viewable);
    assert!(!response.rights[0].record.editable);
    assert!(!response.rights[0].fields["文字列複数行_0"].viewable);
    assert_eq!(response.rights[1].id, 2);
    assert!(response.rights[1].record.deletable);
}

#[test]
fn app_get_app_wire_format() {
    let response = check!(
        app::get_app(8),
        "GET",
        "/k/v1/app.json",
        &[("id", "8")],
        None,
        include_str!("../testdata/rest-api/get-app.json")
    );
    assert_eq!(response.app_id, 1);
    assert_eq!(response.name, "案件管理");
    assert_eq!(response.space_id, Some(2));
    assert_eq!(response.thread_id, Some(3));
    assert_eq!(response.creator.code, "tanaka");
}

#[test]
fn app_get_app_statistics_wire_format() {
    let response = check!(
        app::get_app_statistics().offset(1).limit(2),
        "GET",
        "/k/v1/apps/statistics.json",
        &[("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-apps-statistics.json")
    );
    assert_eq!(response.apps.len(), 1);
    let app = &response.apps[0];
    assert_eq!(app.id, 1);
    assert_eq!(app.name, "案件管理");
    assert_eq!(app.space.as_ref().unwrap().name, "営業管理");
    assert_eq!(app.status, crate::model::app::statistics::AppStatus::Activated);
    assert_eq!(app.record_count, 100);
    assert_eq!(app.storage_usage, 47_000_000);
    assert!(app.customized);
}

#[test]
fn app_get_app_admin_notes_wire_format() {
    let response = check!(
        app::get_app_admin_notes(8),
        "GET",
        "/k/v1/app/adminNotes.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-app-admin-notes.json")
    );
    assert_eq!(response.content, "<div>アプリの管理者用メモ</div>");
    assert!(response.include_in_template_and_duplicates);
    assert_eq!(response.revision, 2);
}

#[test]
fn app_update_app_admin_notes_wire_format() {
    let response = check!(
        app::update_app_admin_notes(8),
        "PUT",
        "/k/v1/preview/app/adminNotes.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-app-admin-notes.json")
    );
    assert_eq!(response.revision, 2);
}

#[test]
fn app_form_get_form_fields_wire_format() {
    let response = check!(
        app::form::get_form_fields(8),
        "GET",
        "/k/v1/app/form/fields.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-form-fields.json")
    );
    assert_eq!(response.revision, 2);
    let FieldProperty::Number(number) = &response.properties["数値_0"] else {
        panic!("expected number field")
    };
    assert_eq!(number.default_value.as_ref().unwrap().to_string(), "12345");
    assert_eq!(number.max_value.as_ref().unwrap().to_string(), "64");
    assert_eq!(number.display_scale, None);
    let FieldProperty::DateTime(datetime) = &response.properties["日時_0"] else {
        panic!("expected datetime field")
    };
    assert_eq!(
        datetime.default_value,
        Some(chrono::DateTime::parse_from_rfc3339("2012-07-19T00:00:00Z").unwrap())
    );
    let FieldProperty::Subtable(table) = &response.properties["テーブル_0"] else {
        panic!("expected subtable")
    };
    assert_eq!(table.fields["文字列__1行_テーブル"].field_code(), "文字列__1行_テーブル");
    let FieldProperty::Category(category) = &response.properties["カテゴリー"] else {
        panic!("expected category field")
    };
    assert!(category.enabled);
}

#[test]
fn app_form_get_form_layout_wire_format() {
    let response = check!(
        app::form::get_form_layout(8),
        "GET",
        "/k/v1/app/form/layout.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-form-layout.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.layout.len(), 3);
    let Layout::Row { fields } = &response.layout[0] else {
        panic!("expected row")
    };
    assert_eq!(fields[1].code.as_deref(), Some("文字列複数行_0"));
    assert_eq!(fields[1].size.as_ref().unwrap().inner_height, Some(100));
    assert_eq!(fields[2].element_id.as_deref(), Some("label"));
    let Layout::Subtable { code, fields } = &response.layout[1] else {
        panic!("expected subtable")
    };
    assert_eq!(code, "テーブル_0");
    assert_eq!(fields[0].size.as_ref().unwrap().width, Some(200));
    let Layout::Group { code, layout } = &response.layout[2] else {
        panic!("expected group")
    };
    assert_eq!(code, "グループ_0");
    assert!(matches!(layout[0], Layout::Row { .. }));
}

#[test]
fn app_form_get_form_wire_format() {
    let response = check!(
        app::form::get_form(8),
        "GET",
        "/k/v1/form.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-form.json")
    );
    assert_eq!(response.properties.len(), 28);
    assert_eq!(response.properties[1]["type"], "SINGLE_LINE_TEXT");
    assert_eq!(response.properties[1]["maxLength"], "64");
    assert_eq!(response.properties[25]["fields"][0]["code"], "文字列__1行__1");
}

#[test]
fn app_form_update_form_fields_wire_format() {
    let response = check!(
        app::form::update_form_fields(8),
        "PUT",
        "/k/v1/preview/app/form/fields.json",
        &[],
        Some(json!({"app": 8, "properties": {}})),
        include_str!("../testdata/rest-api/update-form-fields.json")
    );
    assert_eq!(response.revision, 2);
}

#[test]
fn app_form_delete_form_fields_wire_format() {
    let response = check!(
        app::form::delete_form_fields(8),
        "DELETE",
        "/k/v1/preview/app/form/fields.json",
        &[],
        Some(json!({"app": 8, "fields": []})),
        include_str!("../testdata/rest-api/delete-form-fields.json")
    );
    assert_eq!(response.revision, 3);
}

#[test]
fn app_form_update_form_layout_wire_format() {
    let response = check!(
        app::form::update_form_layout(8),
        "PUT",
        "/k/v1/preview/app/form/layout.json",
        &[],
        Some(json!({"app": 8, "layout": []})),
        include_str!("../testdata/rest-api/update-form-layout.json")
    );
    assert_eq!(response.revision, 3);
}

#[test]
fn app_view_get_views_wire_format() {
    let response = check!(
        app::view::get_views(8),
        "GET",
        "/k/v1/app/views.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-views.json")
    );
    assert_eq!(response.revision, 1);
    assert_eq!(response.views.len(), 4);
    let list = &response.views["一覧1"];
    assert_eq!(list.view_type, ViewType::List);
    assert_eq!(list.name.as_deref(), Some("一覧1"));
    assert_eq!(list.id, Some(1102));
    assert_eq!(list.index, 2);
    assert_eq!(list.fields.as_ref().unwrap(), &["レコード番号", "文字列1行_0"]);
    assert_eq!(list.sort.as_deref(), Some("レコード番号 asc"));
    assert_eq!(response.views["(作業者が自分)"].builtin_type.as_deref(), Some("ASSIGNEE"));
    let calendar = &response.views["カレンダー"];
    assert_eq!(calendar.view_type, ViewType::Calendar);
    assert_eq!(calendar.date.as_deref(), Some("作成日時"));
    assert_eq!(calendar.title.as_deref(), Some("文字列1行_0"));
    let custom = &response.views["カスタマイズ"];
    assert_eq!(custom.view_type, ViewType::Custom);
    assert_eq!(custom.html.as_deref(), Some("カスタマイズされた一覧のHTML<br />"));
    assert_eq!(custom.device, Some(ViewDevice::Any));
}

#[test]
fn app_view_update_views_wire_format() {
    let response = check!(
        app::view::update_views(8),
        "PUT",
        "/k/v1/preview/app/views.json",
        &[],
        Some(json!({"app": 8, "views": {}})),
        include_str!("../testdata/rest-api/update-views.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.views.len(), 3);
    assert_eq!(response.views["カレンダー"].id, 1320);
    assert_eq!(response.views["一覧1"].id, 1321);
    assert_eq!(response.views["カスタマイズ"].id, 1322);
}

#[test]
fn app_report_get_graph_settings_wire_format() {
    let response = check!(
        app::report::get_graph_settings(8),
        "GET",
        "/k/v1/app/reports.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-graph-settings.json")
    );
    assert_eq!(response.revision, 77);
    assert_eq!(response.reports.len(), 3);
    let report = &response.reports["様々なグラフの設定"];
    assert_eq!(report.id, Some(7321));
    assert_eq!(report.name.as_deref(), Some("様々なグラフの設定"));
    assert_eq!(report.chart_type, ChartType::Table);
    assert_eq!(report.groups.as_ref().unwrap()[1].per, Some(TimeUnit::Year));
    assert_eq!(report.aggregations.as_ref().unwrap()[2].aggregation_type, AggregationType::Average);
    assert_eq!(report.aggregations.as_ref().unwrap()[2].code.as_deref(), Some("計算_0"));
    let periodic = response.reports["定期レポートON"].periodic_report.as_ref().unwrap();
    assert_eq!(periodic.active, Some(true));
    let period = periodic.period.as_ref().unwrap();
    assert_eq!(period.every, ReportInterval::Quarter);
    assert_eq!(period.pattern, Some(QuarterPattern::JanAprJulOct));
    assert_eq!(period.day_of_month.as_deref(), Some("END_OF_MONTH"));
    assert_eq!(period.time.as_deref(), Some("23:30"));
    assert!(response.reports["初期設定"].periodic_report.is_none());
}

#[test]
fn app_report_update_graph_settings_wire_format() {
    let response = check!(
        app::report::update_graph_settings(8),
        "PUT",
        "/k/v1/preview/app/reports.json",
        &[],
        Some(json!({"app": 8, "reports": {}})),
        include_str!("../testdata/rest-api/update-graph-settings.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.reports.len(), 3);
    assert_eq!(response.reports["初期設定"].id, 7319);
    assert_eq!(response.reports["定期レポートON"].id, 7323);
}

#[test]
fn app_settings_get_general_settings_wire_format() {
    let response = check!(
        app::settings::get_general_settings(8),
        "GET",
        "/k/v1/app/settings.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-general-settings.json")
    );
    assert_eq!(response.name, "案件管理");
    assert_eq!(
        response.icon,
        AppIcon::Preset {
            key: "APP60".to_owned()
        }
    );
    assert_eq!(response.theme, Theme::White);
    assert_eq!(response.title_field.selection_mode, SelectionMode::Manual);
    assert_eq!(response.title_field.code.as_deref(), Some("文字列1行_0"));
    assert!(response.enable_thumbnails);
    assert!(!response.enable_bulk_deletion);
    assert_eq!(response.number_precision.digits, Some(16));
    assert_eq!(response.number_precision.decimal_places, Some(4));
    assert_eq!(response.number_precision.rounding_mode, Some(RoundingMode::HalfEven));
    assert_eq!(response.first_month_of_fiscal_year, 4);
    assert_eq!(response.revision, 24);
}

#[test]
fn app_settings_update_general_settings_wire_format() {
    let response = check!(
        app::settings::update_general_settings(8),
        "PUT",
        "/k/v1/preview/app/settings.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-general-settings.json")
    );
    assert_eq!(response.revision, 2);
}

#[test]
fn app_settings_get_process_management_settings_wire_format() {
    let response = check!(
        app::settings::get_process_management_settings(8),
        "GET",
        "/k/v1/app/status.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-process-management-settings.json")
    );
    assert!(response.enable);
    assert_eq!(response.revision, 3);
    let states = response.states.as_ref().unwrap();
    assert_eq!(states.len(), 3);
    assert_eq!(states["処理中"].name.as_deref(), Some("処理中"));
    assert_eq!(states["処理中"].index, 1);
    let assignee = states["処理中"].assignee.as_ref().unwrap();
    assert_eq!(assignee.assignee_type, AssigneeType::All);
    assert_eq!(assignee.entities[1].entity.entity_type, SettingsEntityType::FieldEntity);
    assert_eq!(assignee.entities[2].entity.code.as_deref(), Some("supervisor"));
    let actions = response.actions.as_ref().unwrap();
    assert_eq!(actions.len(), 3);
    assert_eq!(actions[2].action_type, Some(ProcessActionType::Secondary));
    assert_eq!(
        actions[2].executable_user.as_ref().unwrap().entities[0].entity.code.as_deref(),
        Some("creator")
    );
}

#[test]
fn app_settings_update_process_management_settings_wire_format() {
    let response = check!(
        app::settings::update_process_management_settings(8),
        "PUT",
        "/k/v1/preview/app/status.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-process-management-settings.json")
    );
    assert_eq!(response.revision, 3);
}

#[test]
fn app_settings_get_customization_wire_format() {
    let response = check!(
        app::settings::get_customization(8),
        "GET",
        "/k/v1/app/customize.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-customization.json")
    );
    assert_eq!(response.scope, CustomizationScope::All);
    assert_eq!(response.revision, 15);
    let js = response.desktop.js.as_ref().unwrap();
    assert!(
        matches!(&js[0], CustomizationResource::Url { url } if url == "https://sample.com/example.js")
    );
    let CustomizationResource::File { file } = &js[1] else {
        panic!("expected uploaded file")
    };
    assert_eq!(file.name.as_deref(), Some("sample.js"));
    assert_eq!(file.size, Some(12345));
    let mobile_js = response.mobile.js.as_ref().unwrap();
    assert_eq!(mobile_js.len(), 2);
    assert!(
        matches!(&mobile_js[1], CustomizationResource::Url { url } if url == "https://sample.com/example-mobile.js")
    );
}

#[test]
fn app_settings_update_customization_wire_format() {
    let response = check!(
        app::settings::update_customization(8),
        "PUT",
        "/k/v1/preview/app/customize.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-customization.json")
    );
    assert_eq!(response.revision, 3);
}

#[test]
fn app_settings_get_general_notification_settings_wire_format() {
    let response = check!(
        app::settings::get_general_notification_settings(8),
        "GET",
        "/k/v1/app/notifications/general.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-general-notification-settings.json")
    );
    assert_eq!(response.revision, 2);
    assert!(response.notify_to_commenter);
    assert_eq!(response.notifications.len(), 1);
    let notification = &response.notifications[0];
    assert_eq!(notification.entity.entity_type, SettingsEntityType::User);
    assert_eq!(notification.entity.code.as_deref(), Some("user1"));
    assert_eq!(notification.record_added, Some(true));
    assert_eq!(notification.comment_added, Some(false));
}

#[test]
fn app_settings_update_general_notification_settings_wire_format() {
    let response = check!(
        app::settings::update_general_notification_settings(8),
        "PUT",
        "/k/v1/preview/app/notifications/general.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-general-notification-settings.json")
    );
    assert_eq!(response.revision, 2);
}

#[test]
fn app_settings_get_per_record_notification_settings_wire_format() {
    let response = check!(
        app::settings::get_per_record_notification_settings(8),
        "GET",
        "/k/v1/app/notifications/perRecord.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-per-record-notification-settings.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.notifications.len(), 1);
    let notification = &response.notifications[0];
    assert_eq!(notification.title.as_deref(), Some("user1が選択されました"));
    assert_eq!(notification.filter_cond.as_deref(), Some("ユーザー選択_0 in (\"user1\")"));
    assert_eq!(notification.targets[0].entity.code.as_deref(), Some("user1"));
    assert_eq!(notification.targets[0].include_subs, Some(false));
}

#[test]
fn app_settings_update_per_record_notification_settings_wire_format() {
    let response = check!(
        app::settings::update_per_record_notification_settings(8),
        "PUT",
        "/k/v1/preview/app/notifications/perRecord.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-per-record-notification-settings.json")
    );
    assert_eq!(response.revision, 2);
}

#[test]
fn app_settings_get_reminder_notification_settings_wire_format() {
    let response = check!(
        app::settings::get_reminder_notification_settings(8),
        "GET",
        "/k/v1/app/notifications/reminder.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-reminder-notification-settings.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.timezone, "Asia/Tokyo");
    assert_eq!(response.notifications.len(), 2);
    assert_eq!(response.notifications[0].timing.days_later, 1);
    assert_eq!(response.notifications[0].timing.hours_later, Some(2));
    assert_eq!(response.notifications[1].timing.days_later, -3);
    assert_eq!(response.notifications[1].timing.time.as_deref(), Some("08:30"));
    assert_eq!(response.notifications[1].title.as_deref(), Some("リマインドです"));
}

#[test]
fn app_settings_update_reminder_notification_settings_wire_format() {
    let response = check!(
        app::settings::update_reminder_notification_settings(8),
        "PUT",
        "/k/v1/preview/app/notifications/reminder.json",
        &[],
        Some(json!({"app": 8})),
        include_str!("../testdata/rest-api/update-reminder-notification-settings.json")
    );
    assert_eq!(response.revision, 2);
}

#[test]
fn app_settings_get_app_permissions_wire_format() {
    let response = check!(
        app::settings::get_app_permissions(8),
        "GET",
        "/k/v1/app/acl.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-app-permissions.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.rights.len(), 4);
    assert_eq!(response.rights[0].entity.code.as_deref(), Some("user1"));
    assert_eq!(response.rights[0].app_editable, Some(true));
    assert_eq!(response.rights[1].record_viewable, Some(false));
    assert_eq!(response.rights[2].include_subs, Some(true));
    assert_eq!(response.rights[3].entity.entity_type, SettingsEntityType::Creator);
    assert_eq!(response.rights[3].entity.code, None);
}

#[test]
fn app_settings_update_app_permissions_wire_format() {
    let response = check!(
        app::settings::update_app_permissions(8),
        "PUT",
        "/k/v1/app/acl.json",
        &[],
        Some(json!({"app": 8, "rights": []})),
        include_str!("../testdata/rest-api/update-app-permissions.json")
    );
    assert_eq!(response.revision, 3);
}

#[test]
fn app_settings_get_record_permissions_wire_format() {
    let response = check!(
        app::settings::get_record_permissions(8),
        "GET",
        "/k/v1/record/acl.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-record-permissions.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.rights.len(), 1);
    let entities = &response.rights[0].entities;
    assert_eq!(entities.len(), 2);
    assert_eq!(entities[0].entity.entity_type, SettingsEntityType::Organization);
    assert_eq!(entities[0].viewable, Some(false));
    assert_eq!(entities[0].include_subs, Some(true));
    assert_eq!(entities[1].entity.code.as_deref(), Some("更新者"));
    assert_eq!(entities[1].editable, Some(true));
}

#[test]
fn app_settings_update_record_permissions_wire_format() {
    let response = check!(
        app::settings::update_record_permissions(8),
        "PUT",
        "/k/v1/record/acl.json",
        &[],
        Some(json!({"app": 8, "rights": []})),
        include_str!("../testdata/rest-api/update-record-permissions.json")
    );
    assert_eq!(response.revision, 3);
}

#[test]
fn app_settings_get_field_permissions_wire_format() {
    let response = check!(
        app::settings::get_field_permissions(8),
        "GET",
        "/k/v1/field/acl.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-field-permissions.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.rights.len(), 2);
    assert_eq!(response.rights[0].code, "文字列1行_0");
    assert_eq!(response.rights[0].entities[0].accessibility, Accessibility::Write);
    assert_eq!(response.rights[0].entities[1].accessibility, Accessibility::Read);
    assert_eq!(response.rights[1].entities[0].accessibility, Accessibility::None);
    assert_eq!(response.rights[1].entities[0].include_subs, Some(true));
}

#[test]
fn app_settings_update_field_permissions_wire_format() {
    let response = check!(
        app::settings::update_field_permissions(8),
        "PUT",
        "/k/v1/field/acl.json",
        &[],
        Some(json!({"app": 8, "rights": []})),
        include_str!("../testdata/rest-api/update-field-permissions.json")
    );
    assert_eq!(response.revision, 3);
}

#[test]
fn app_settings_get_action_settings_wire_format() {
    let response = check!(
        app::settings::get_action_settings(8),
        "GET",
        "/k/v1/app/actions.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-action-settings.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.actions.len(), 1);
    let action = &response.actions["アクションA"];
    assert_eq!(action.name.as_deref(), Some("アクションA"));
    assert_eq!(action.id, Some(1));
    assert_eq!(action.index, 0);
    assert_eq!(action.dest_app.as_ref().unwrap().app, Some(2));
    assert_eq!(action.dest_app.as_ref().unwrap().code.as_deref(), Some("APPB"));
    let mappings = action.mappings.as_ref().unwrap();
    assert_eq!(mappings[0].src_type, ActionSourceType::Field);
    assert_eq!(mappings[0].src_field.as_deref(), Some("数値_0"));
    assert_eq!(mappings[1].src_type, ActionSourceType::RecordUrl);
    assert_eq!(mappings[1].dest_field, "リンク_0");
    assert_eq!(action.entities.as_ref().unwrap()[0].code, "userA");
}

#[test]
fn app_settings_update_action_settings_wire_format() {
    let response = check!(
        app::settings::update_action_settings(8),
        "PUT",
        "/k/v1/preview/app/actions.json",
        &[],
        Some(json!({"app": 8, "actions": {}})),
        include_str!("../testdata/rest-api/update-action-settings.json")
    );
    assert_eq!(response.revision, 2);
}

#[test]
fn app_settings_get_app_plugins_wire_format() {
    let response = check!(
        app::settings::get_app_plugins(8),
        "GET",
        "/k/v1/app/plugins.json",
        &[("app", "8")],
        None,
        include_str!("../testdata/rest-api/get-app-plugins.json")
    );
    assert_eq!(response.revision, 2);
    assert_eq!(response.plugins.len(), 1);
    assert_eq!(response.plugins[0].name, "プラグイン名");
    assert_eq!(response.plugins[0].id, "djmhffjhfgmebgnmcggopedaofckljlj");
    assert!(response.plugins[0].enabled);
}

#[test]
fn app_settings_add_app_plugins_wire_format() {
    let response = check!(
        app::settings::add_app_plugins(8),
        "POST",
        "/k/v1/preview/app/plugins.json",
        &[],
        Some(json!({"app": 8, "ids": []})),
        include_str!("../testdata/rest-api/add-app-plugins.json")
    );
    assert_eq!(response.revision, 2);
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
    let response = check!(
        space::get_space(8),
        "GET",
        "/k/v1/space.json",
        &[("id", "8")],
        None,
        include_str!("../testdata/rest-api/get-space.json")
    );
    assert_eq!(response.id, 1);
    assert_eq!(response.name, "全体連絡スペース");
    assert_eq!(response.default_thread, 3);
    assert_eq!(response.member_count, 10);
    assert_eq!(response.cover_type, SpaceCoverType::Preset);
    assert_eq!(response.body.as_deref(), Some("<b>全体</b>のスペースです。"));
    assert_eq!(response.show_thread_list, Some(true));
    assert_eq!(response.permissions.create_app, CreateAppPermission::Everyone);
    assert_eq!(response.attached_apps.len(), 3);
    assert_eq!(response.attached_apps[0].app_id, 1);
    assert_eq!(response.attached_apps[0].creator.code, "sato");
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
    let response = check!(
        space::add_space_from_template(8, "Space"),
        "POST",
        "/k/v1/template/space.json",
        &[],
        Some(json!({"id": 8, "name": "Space", "members": []})),
        include_str!("../testdata/rest-api/add-space-from-template.json")
    );
    assert_eq!(response.id, 1);
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
    let response = check!(
        space::get_space_members(8),
        "GET",
        "/k/v1/space/members.json",
        &[("id", "8")],
        None,
        include_str!("../testdata/rest-api/get-space-members.json")
    );
    assert_eq!(response.members.len(), 4);
    assert_eq!(response.members[0].entity.entity_type, EntityType::USER);
    assert_eq!(response.members[0].is_implicit, Some(true));
    assert!(response.members[1].is_admin);
    assert_eq!(response.members[2].entity.entity_type, EntityType::GROUP);
    assert_eq!(response.members[3].include_subs, Some(true));
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
    let response = check!(
        space::get_space_statistics().offset(1).limit(2),
        "GET",
        "/k/v1/spaces/statistics.json",
        &[("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-spaces-statistics.json")
    );
    assert_eq!(response.spaces.len(), 1);
    assert_eq!(response.spaces[0].id, 1);
    assert_eq!(response.spaces[0].name, "第1営業部");
    assert_eq!(response.spaces[0].administrator_count, 2);
    assert_eq!(response.spaces[0].member_count, 10);
    assert!(response.spaces[0].is_private);
    assert!(!response.spaces[0].is_guest);
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
    let response = check!(
        plugin::get_plugins().ids(["plugin-id".to_owned()]).offset(1).limit(2),
        "GET",
        "/k/v1/plugins.json",
        &[("ids[0]", "plugin-id"), ("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-plugins.json")
    );
    assert_eq!(response.plugins.len(), 1);
    assert_eq!(response.plugins[0].id, "djmhffjhfgmebgnmcggopedaofckljlj");
    assert_eq!(response.plugins[0].name, "サンプルプラグイン");
    assert_eq!(response.plugins[0].description, "プラグインの説明");
    assert_eq!(response.plugins[0].version, "1.0.0");
    assert!(!response.plugins[0].is_market_plugin);
}

#[test]
fn plugin_get_required_plugins_wire_format() {
    let response = check!(
        plugin::get_required_plugins().offset(1).limit(2),
        "GET",
        "/k/v1/plugins/required.json",
        &[("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-required-plugins.json")
    );
    assert_eq!(response.plugins.len(), 1);
    assert_eq!(response.plugins[0].name, "サンプルプラグイン");
    assert!(!response.plugins[0].is_market_plugin);
}

#[test]
fn plugin_get_plugin_apps_wire_format() {
    let response = check!(
        plugin::get_plugin_apps("plugin-id").offset(1).limit(2),
        "GET",
        "/k/v1/plugin/apps.json",
        &[("id", "plugin-id"), ("offset", "1"), ("limit", "2")],
        None,
        include_str!("../testdata/rest-api/get-plugin-apps.json")
    );
    assert_eq!(response.apps.len(), 1);
    assert_eq!(response.apps[0].id, 1);
    assert_eq!(response.apps[0].name, "サンプルアプリ");
}

#[test]
fn plugin_add_plugin_wire_format() {
    let response = check!(
        plugin::add_plugin("file-key"),
        "POST",
        "/k/v1/plugin.json",
        &[],
        Some(json!({"fileKey": "file-key"})),
        include_str!("../testdata/rest-api/add-plugin.json")
    );
    assert_eq!(response.id, "djmhffjhfgmebgnmcggopedaofckljlj");
    assert_eq!(response.version, "1");
}

#[test]
fn plugin_update_plugin_wire_format() {
    let response = check!(
        plugin::update_plugin("plugin-id", "file-key"),
        "PUT",
        "/k/v1/plugin.json",
        &[],
        Some(json!({"id": "plugin-id", "fileKey": "file-key"})),
        include_str!("../testdata/rest-api/update-plugin.json")
    );
    assert_eq!(response.id, "djmhffjhfgmebgnmcggopedaofckljlj");
    assert_eq!(response.version, "1");
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
    let response = check!(
        apis::get_apis(),
        "GET",
        "/k/v1/apis.json",
        &[],
        None,
        include_str!("../testdata/rest-api/get-apis.json")
    );
    assert_eq!(response.base_url, "https://sample.cybozu.com/k/v1/");
    assert_eq!(response.apis.len(), 1);
    assert_eq!(response.apis["records/get"].link, "apis/records/get.json");
}

#[test]
fn apis_get_api_schema_wire_format() {
    let response = check!(
        apis::get_api_schema("records/get"),
        "GET",
        "/k/v1/apis/records/get.json",
        &[],
        None,
        include_str!("../testdata/rest-api/get-api-schema.json")
    );
    assert_eq!(response.id, "records/get");
    assert_eq!(response.path, "records.json");
    assert_eq!(response.http_method, "GET");
    assert_eq!(response.request["required"], json!(["app"]));
    assert_eq!(response.response["properties"]["records"]["type"], "array");
    assert_eq!(
        response.schemas["SingleLineTextSimpleValue"]["properties"]["value"]["type"],
        "string"
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
fn preview_reads_switch_environments_preserving_query_and_guest_space() {
    macro_rules! preview {
        ($request:expr, $path:expr, $reply:expr) => {{
            for guest in [false, true] {
                for preview in [false, true] {
                    let path = if guest {
                        $path.to_owned()
                    } else {
                        $path.replace("/guest/42", "")
                    };
                    let path = if preview {
                        path
                    } else {
                        path.replace("/preview/", "/")
                    };
                    let client = client("GET", &path, &[("app", "8")], None, $reply, guest);
                    $request.preview(true).preview(false).preview(preview).send(&client).unwrap();
                }
            }
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
    for (guest, path) in [
        (false, "/k/v1/app/form/fields.json"),
        (true, "/k/guest/42/v1/app/form/fields.json"),
    ] {
        let client = client(
            "GET",
            path,
            &[("app", "8"), ("lang", "ja")],
            None,
            include_str!("../testdata/rest-api/get-form-fields.json"),
            guest,
        );
        app::form::get_form_fields(8)
            .lang("ja")
            .preview(true)
            .preview(false)
            .send(&client)
            .unwrap();
    }
}

#[test]
fn access_control_updates_support_preview_and_revision() {
    macro_rules! preview {
        ($request:expr, $path:expr) => {{
            for guest in [false, true] {
                for preview in [false, true] {
                    let path = if guest { $path.to_owned() } else { $path.replace("/guest/42", "") };
                    let path = if preview { path } else { path.replace("/preview/", "/") };
                    let client = client("PUT", &path, &[], Some(json!({"app":8,"rights":[],"revision":12})), "{\"revision\":\"13\"}", guest);
                    let response = $request.preview(true).preview(false).preview(preview).revision(Some(12)).send(&client).unwrap();
                    assert_eq!(response.revision, 13);
                }
            }
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
        ("DATE", "2026-10-02"),
        ("TIME", ""),
        ("TIME", "09:00"),
        ("TIME", "09:00:00"),
        ("DATETIME", ""),
        ("DATETIME", "2012-07-19T00:00Z"),
        ("DATETIME", "2012-07-19T00:00:00Z"),
    ] {
        let field: FieldProperty = serde_json::from_value(json!({"type":kind,"code":"field","label":"Field","noLabel":false,"required":false,"unique":false,"defaultNowValue":false,"defaultValue":value})).unwrap();
        let encoded = serde_json::to_value(field).unwrap();
        assert_eq!(encoded["type"], kind);
        let expected = match (kind, value) {
            (_, "") => Value::Null,
            ("DATE", _) => json!("2026-10-02"),
            ("TIME", _) => json!("09:00"),
            ("DATETIME", _) => json!("2012-07-19T00:00:00Z"),
            _ => unreachable!(),
        };
        assert_eq!(encoded["defaultValue"], expected, "{kind}: {value}");
    }
}

#[test]
fn response_revisions_accept_numbers_and_strings_without_losing_precision() {
    for revision in [
        json!(0),
        json!("0"),
        json!(u64::MAX),
        json!(u64::MAX.to_string()),
    ] {
        let response: app::settings::RevisionResponse =
            serde_json::from_value(json!({"revision":revision.clone()})).unwrap();
        let expected = if revision == json!(0) || revision == json!("0") {
            0
        } else {
            u64::MAX
        };
        assert_eq!(response.revision, expected);
    }
    for revision in [
        json!(null),
        json!(false),
        json!(-1),
        json!(1.5),
        json!(""),
        json!("abc"),
        json!("18446744073709551616"),
    ] {
        assert!(
            serde_json::from_value::<app::settings::RevisionResponse>(
                json!({"revision":revision.clone()})
            )
            .is_err(),
            "revision: {revision}"
        );
    }
}

#[test]
fn malformed_view_responses_return_json_errors() {
    for reply in [
        "not JSON",
        r#"{"revision":"1"}"#,
        r#"{"views":{},"revision":"invalid"}"#,
        r#"{"views":{"list":{"type":"UNKNOWN","index":"0"}},"revision":"1"}"#,
    ] {
        let client = client("GET", "/k/v1/app/views.json", &[("app", "8")], None, reply, false);
        assert!(
            matches!(app::view::get_views(8).send(&client), Err(ApiError::Json(_))),
            "response: {reply}"
        );
    }
}
