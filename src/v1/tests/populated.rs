//! Populated update requests exercise setters and nested JSON representations.

use super::*;
use crate::model::Entity;
use crate::model::app::field::{FieldPropertyUpdate, RelatedApp, UnitPosition};
use crate::model::app::layout::{LayoutField, LayoutSize};
use crate::model::record::FieldType;
use crate::model::space::{GuestUser, SpaceMember, SpacePermissions};

macro_rules! update {
    ($request:expr, $path:expr, $body:expr) => {{
        update!($request, "PUT", $path, $body);
    }};
    ($request:expr, $method:expr, $path:expr, $body:expr) => {{
        let response = check!($request, $method, $path, &[], Some($body), r#"{"revision":"13"}"#);
        assert_eq!(response.revision, 13);
    }};
}

fn user_target() -> EntityTarget {
    EntityTarget {
        entity: SettingsEntity {
            entity_type: SettingsEntityType::User,
            code: Some("user1".to_owned()),
        },
        include_subs: Some(false),
    }
}

fn space_member() -> SpaceMember {
    SpaceMember {
        entity: Entity {
            entity_type: EntityType::USER,
            code: "user1".to_owned(),
        },
        is_admin: true,
        include_subs: None,
        is_implicit: Some(false), // Response metadata must not be sent back.
    }
}

#[test]
fn form_field_updates_send_nested_properties() {
    let mut amount = FieldPropertyUpdate::new(FieldType::Number);
    amount.label = Some("Amount".to_owned());
    amount.required = Some(false);
    amount.min_value = Some("-1.25".parse().unwrap());
    amount.max_value = Some("999.99".parse().unwrap());
    amount.display_scale = Some(2);
    amount.default_value = Some(json!("0"));
    amount.unit = Some("JPY".to_owned());
    amount.unit_position = Some(UnitPosition::After);
    let mut table = FieldPropertyUpdate::new(FieldType::Subtable);
    table.fields = Some([("amount".to_owned(), amount)].into());
    let mut text = FieldPropertyUpdate::new(FieldType::SingleLineText);
    text.max_length = Some(64);
    text.min_length = Some(0);
    text.unique = Some(true);
    update!(
        app::form::update_form_fields(8)
            .properties([("items".to_owned(), table)])
            .field("name", text)
            .raw_field("total", json!({"type":"NUMBER","maxValue":""}))
            .revision(Some(7)),
        "/k/v1/preview/app/form/fields.json",
        json!({"app":8,"revision":7,"properties":{
            "items":{"type":"SUBTABLE","fields":{"amount":{
                "type":"NUMBER","label":"Amount","required":false,
                "minValue":"-1.25","maxValue":"999.99","displayScale":"2",
                "defaultValue":"0","unit":"JPY","unitPosition":"AFTER"
            }}},
            "name":{"type":"SINGLE_LINE_TEXT","maxLength":"64","minLength":"0","unique":true},
            "total":{"type":"NUMBER","maxValue":""}
        }})
    );
}

#[test]
fn form_field_deletion_sends_codes_and_revision() {
    update!(
        app::form::delete_form_fields(8)
            .fields(["obsolete".to_owned(), "items".to_owned()])
            .revision(Some(7)),
        "DELETE",
        "/k/v1/preview/app/form/fields.json",
        json!({"app":8,"fields":["obsolete","items"],"revision":7})
    );
}

#[test]
fn form_layout_updates_send_rows_tables_and_groups() {
    let number = LayoutField {
        field_type: FieldType::Number,
        code: Some("amount".to_owned()),
        label: None,
        element_id: None,
        size: Some(LayoutSize {
            width: Some(200),
            ..Default::default()
        }),
    };
    let spacer = LayoutField {
        field_type: FieldType::Spacer,
        code: None,
        label: None,
        element_id: Some("spacer".to_owned()),
        size: Some(LayoutSize {
            width: Some(100),
            height: Some(50),
            inner_height: None,
        }),
    };
    update!(
        app::form::update_form_layout(8)
            .layout([
                Layout::Row {
                    fields: vec![spacer]
                },
                Layout::Subtable {
                    code: "items".to_owned(),
                    fields: vec![number.clone()]
                },
                Layout::Group {
                    code: "details".to_owned(),
                    layout: vec![Layout::Row {
                        fields: vec![number]
                    }]
                },
            ])
            .revision(Some(7)),
        "/k/v1/preview/app/form/layout.json",
        json!({"app":8,"revision":7,"layout":[
            {"type":"ROW","fields":[{"type":"SPACER","elementId":"spacer","size":{"width":"100","height":"50"}}]},
            {"type":"SUBTABLE","code":"items","fields":[{"type":"NUMBER","code":"amount","size":{"width":"200"}}]},
            {"type":"GROUP","code":"details","layout":[{"type":"ROW","fields":[{"type":"NUMBER","code":"amount","size":{"width":"200"}}]}]}
        ]})
    );
}

#[test]
fn view_updates_send_list_calendar_and_custom_settings() {
    let list = View {
        view_type: ViewType::List,
        index: 0,
        name: Some("Renamed list".to_owned()),
        id: Some(10),
        builtin_type: Some("ASSIGNEE".to_owned()),
        fields: Some(vec!["name".to_owned(), "amount".to_owned()]),
        filter_cond: Some("amount > 0".to_owned()),
        sort: Some("amount desc".to_owned()),
        ..Default::default()
    };
    let calendar = View {
        view_type: ViewType::Calendar,
        index: 1,
        date: Some("date".to_owned()),
        title: Some("name".to_owned()),
        ..Default::default()
    };
    let custom = View {
        view_type: ViewType::Custom,
        index: 2,
        html: Some("<p>Custom view</p>".to_owned()),
        pager: Some(false),
        device: Some(ViewDevice::Any),
        ..Default::default()
    };
    let response = check!(
        app::view::update_views(8)
            .views([("list".to_owned(), list)])
            .view("calendar", calendar)
            .view("custom", custom)
            .revision(Some(7)),
        "PUT",
        "/k/v1/preview/app/views.json",
        &[],
        Some(json!({"app":8,"revision":7,"views":{
            "list":{"type":"LIST","index":"0","name":"Renamed list","fields":["name","amount"],"filterCond":"amount > 0","sort":"amount desc"},
            "calendar":{"type":"CALENDAR","index":"1","date":"date","title":"name"},
            "custom":{"type":"CUSTOM","index":"2","html":"<p>Custom view</p>","pager":false,"device":"ANY"}
        }})),
        r#"{"revision":"13","views":{"list":{"id":"10"},"calendar":{"id":"11"},"custom":{"id":"12"}}}"#
    );
    assert_eq!(response.revision, 13);
    assert_eq!(response.views["custom"].id, 12);
}

#[test]
fn view_revision_check_can_be_cleared() {
    let response = check!(
        app::view::update_views(8)
            .view("list", View::default())
            .revision(Some(7))
            .revision(None),
        "PUT",
        "/k/v1/preview/app/views.json",
        &[],
        Some(json!({"app":8,"views":{"list":{"type":"LIST","index":"0"}}})),
        r#"{"revision":"13","views":{"list":{"id":"10"}}}"#
    );
    assert_eq!(response.views["list"].id, 10);
}

#[test]
fn graph_updates_send_aggregations_and_periodic_reports() {
    let sales = Graph {
        chart_type: ChartType::Column,
        chart_mode: Some(ChartMode::Stacked),
        index: 0,
        id: Some(10),
        name: Some("Sales".to_owned()),
        groups: Some(vec![GraphGroup {
            code: "date".to_owned(),
            per: Some(TimeUnit::Month),
        }]),
        aggregations: Some(vec![GraphAggregation {
            aggregation_type: AggregationType::Sum,
            code: Some("amount".to_owned()),
        }]),
        filter_cond: Some("amount > 0".to_owned()),
        sorts: Some(vec![GraphSort {
            by: ReportSortBy::Total,
            order: ReportSortOrder::Desc,
        }]),
        periodic_report: Some(PeriodicReport {
            active: Some(true),
            period: Some(ReportPeriod {
                every: ReportInterval::Month,
                day_of_month: Some("END_OF_MONTH".to_owned()),
                time: Some("09:00".to_owned()),
                ..Default::default()
            }),
        }),
    };
    let hourly = Graph {
        chart_type: ChartType::Line,
        index: 1,
        periodic_report: Some(PeriodicReport {
            active: Some(false),
            period: Some(ReportPeriod {
                every: ReportInterval::Hour,
                minute: Some(30),
                ..Default::default()
            }),
        }),
        ..Default::default()
    };
    let response = check!(
        app::report::update_graph_settings(8)
            .reports([("sales".to_owned(), sales)])
            .report("hourly", hourly)
            .revision(Some(7)),
        "PUT",
        "/k/v1/preview/app/reports.json",
        &[],
        Some(json!({"app":8,"revision":7,"reports":{
            "sales":{"chartType":"COLUMN","chartMode":"STACKED","index":"0","name":"Sales",
                "groups":[{"code":"date","per":"MONTH"}],"aggregations":[{"type":"SUM","code":"amount"}],
                "filterCond":"amount > 0","sorts":[{"by":"TOTAL","order":"DESC"}],
                "periodicReport":{"active":true,"period":{"every":"MONTH","dayOfMonth":"END_OF_MONTH","time":"09:00"}}},
            "hourly":{"chartType":"LINE","index":"1","periodicReport":{"active":false,"period":{"every":"HOUR","minute":"30"}}}
        }})),
        r#"{"revision":"13","reports":{"sales":{"id":"10"},"hourly":{"id":"11"}}}"#
    );
    assert_eq!(response.revision, 13);
    assert_eq!(response.reports["sales"].id, 10);
}

#[test]
fn admin_note_updates_send_content_and_false_flag() {
    update!(
        app::update_app_admin_notes(8)
            .content("<p>Notes</p>")
            .include_in_template_and_duplicates(false)
            .revision(Some(7)),
        "/k/v1/preview/app/adminNotes.json",
        json!({"app":8,"content":"<p>Notes</p>","includeInTemplateAndDuplicates":false,"revision":7})
    );
}

#[test]
fn general_setting_updates_send_precision_and_flags() {
    update!(
        app::settings::update_general_settings(8)
            .name("Sales")
            .description("<p>Sales app</p>")
            .icon(AppIcon::Preset {
                key: "APP60".to_owned()
            })
            .theme(Theme::Blue)
            .title_field(TitleField {
                selection_mode: SelectionMode::Manual,
                code: Some("name".to_owned())
            })
            .enable_thumbnails(false)
            .enable_bulk_deletion(true)
            .enable_comments(false)
            .enable_duplicate_record(false)
            .enable_inline_record_editing(true)
            .number_precision(NumberPrecision {
                digits: Some(16),
                decimal_places: Some(4),
                rounding_mode: Some(RoundingMode::Down)
            })
            .first_month_of_fiscal_year(4)
            .revision(Some(7)),
        "/k/v1/preview/app/settings.json",
        json!({"app":8,"name":"Sales","description":"<p>Sales app</p>","icon":{"type":"PRESET","key":"APP60"},"theme":"BLUE",
            "titleField":{"selectionMode":"MANUAL","code":"name"},"enableThumbnails":false,
            "enableBulkDeletion":true,"enableComments":false,"enableDuplicateRecord":false,
            "enableInlineRecordEditing":true,"numberPrecision":{"digits":"16","decimalPlaces":"4","roundingMode":"DOWN"},
            "firstMonthOfFiscalYear":4,"revision":7})
    );
}

#[test]
fn process_updates_send_states_actions_and_executable_users() {
    let open = ProcessState {
        name: Some("Open".to_owned()),
        index: 0,
        assignee: Some(ProcessAssignee {
            assignee_type: AssigneeType::One,
            entities: vec![user_target()],
        }),
    };
    let done = ProcessState {
        index: 1,
        ..Default::default()
    };
    let action = ProcessAction {
        name: "Complete".to_owned(),
        from: "Open".to_owned(),
        to: "Done".to_owned(),
        filter_cond: Some("amount > 0".to_owned()),
        action_type: Some(ProcessActionType::Secondary),
        executable_user: Some(ExecutableUser {
            entities: vec![user_target()],
        }),
    };
    update!(
        app::settings::update_process_management_settings(8)
            .enable(true)
            .states([("Open".to_owned(), open)])
            .state("Done", done)
            .actions([action])
            .revision(Some(7)),
        "/k/v1/preview/app/status.json",
        json!({"app":8,"enable":true,"revision":7,"states":{
            "Open":{"name":"Open","index":"0","assignee":{"type":"ONE","entities":[{"entity":{"type":"USER","code":"user1"},"includeSubs":false}]}},
            "Done":{"index":"1"}},
            "actions":[{"name":"Complete","from":"Open","to":"Done","filterCond":"amount > 0","type":"SECONDARY",
                "executableUser":{"entities":[{"entity":{"type":"USER","code":"user1"},"includeSubs":false}]}}]})
    );
}

#[test]
fn customization_updates_send_desktop_mobile_urls_and_empty_arrays() {
    update!(
        app::settings::update_customization(8)
            .scope(CustomizationScope::Admin)
            .desktop(Customization {
                js: Some(vec![CustomizationResource::Url {
                    url: "https://example.com/app.js".to_owned()
                }]),
                css: Some(vec![])
            })
            .mobile(Customization {
                js: Some(vec![]),
                css: Some(vec![CustomizationResource::Url {
                    url: "https://example.com/mobile.css".to_owned()
                }])
            })
            .revision(Some(7)),
        "/k/v1/preview/app/customize.json",
        json!({"app":8,"scope":"ADMIN","revision":7,
            "desktop":{"js":[{"type":"URL","url":"https://example.com/app.js"}],"css":[]},
            "mobile":{"js":[],"css":[{"type":"URL","url":"https://example.com/mobile.css"}]}})
    );
}

#[test]
fn general_notifications_send_event_flags() {
    let notification = GeneralNotification {
        entity: user_target().entity,
        include_subs: Some(false),
        record_added: Some(true),
        record_edited: Some(false),
        comment_added: Some(true),
        status_changed: Some(false),
        file_imported: Some(true),
    };
    update!(
        app::settings::update_general_notification_settings(8)
            .notifications([notification])
            .notify_to_commenter(true)
            .revision(Some(7)),
        "/k/v1/preview/app/notifications/general.json",
        json!({"app":8,"revision":7,"notifyToCommenter":true,"notifications":[{
            "entity":{"type":"USER","code":"user1"},"includeSubs":false,
            "recordAdded":true,"recordEdited":false,"commentAdded":true,"statusChanged":false,"fileImported":true}]})
    );
}

#[test]
fn per_record_notifications_send_conditions_and_targets() {
    let notification = PerRecordNotification {
        filter_cond: Some("amount > 0".to_owned()),
        title: Some("New sale".to_owned()),
        targets: vec![user_target()],
    };
    update!(
        app::settings::update_per_record_notification_settings(8)
            .notifications([notification])
            .revision(Some(7)),
        "/k/v1/preview/app/notifications/perRecord.json",
        json!({"app":8,"revision":7,"notifications":[{"filterCond":"amount > 0","title":"New sale",
            "targets":[{"entity":{"type":"USER","code":"user1"},"includeSubs":false}]}]})
    );
}

#[test]
fn reminder_notifications_send_signed_offsets_and_timezone() {
    let before = ReminderNotification {
        timing: ReminderTiming {
            code: "due_date".to_owned(),
            days_later: -3,
            hours_later: None,
            time: Some("08:30".to_owned()),
        },
        filter_cond: Some("amount > 0".to_owned()),
        title: Some("Due soon".to_owned()),
        targets: vec![user_target()],
    };
    let after = ReminderNotification {
        timing: ReminderTiming {
            code: "created_at".to_owned(),
            days_later: 1,
            hours_later: Some(2),
            time: None,
        },
        targets: vec![user_target()],
        ..Default::default()
    };
    update!(
        app::settings::update_reminder_notification_settings(8)
            .notifications([before, after])
            .timezone("Asia/Tokyo")
            .revision(Some(7)),
        "/k/v1/preview/app/notifications/reminder.json",
        json!({"app":8,"revision":7,"timezone":"Asia/Tokyo","notifications":[
            {"timing":{"code":"due_date","daysLater":"-3","time":"08:30"},"filterCond":"amount > 0","title":"Due soon",
                "targets":[{"entity":{"type":"USER","code":"user1"},"includeSubs":false}]},
            {"timing":{"code":"created_at","daysLater":"1","hoursLater":"2"},
                "targets":[{"entity":{"type":"USER","code":"user1"},"includeSubs":false}]}
        ]})
    );
}

#[test]
fn app_permissions_send_explicit_false_and_creator_without_code() {
    let user = AppRight {
        entity: user_target().entity,
        include_subs: Some(false),
        app_editable: Some(false),
        record_viewable: Some(true),
        record_addable: Some(true),
        record_editable: Some(false),
        record_deletable: Some(false),
        record_importable: Some(false),
        record_exportable: Some(true),
    };
    let creator = AppRight {
        entity: SettingsEntity {
            entity_type: SettingsEntityType::Creator,
            code: None,
        },
        app_editable: Some(true),
        ..Default::default()
    };
    update!(
        app::settings::update_app_permissions(8)
            .rights([user, creator])
            .revision(Some(7)),
        "/k/v1/app/acl.json",
        json!({"app":8,"revision":7,"rights":[
            {"entity":{"type":"USER","code":"user1"},"includeSubs":false,"appEditable":false,
                "recordViewable":true,"recordAddable":true,"recordEditable":false,"recordDeletable":false,
                "recordImportable":false,"recordExportable":true},
            {"entity":{"type":"CREATOR"},"appEditable":true}
        ]})
    );
}

#[test]
fn record_permissions_send_conditions_and_nested_rights() {
    let right = RecordRight {
        filter_cond: "amount > 0".to_owned(),
        entities: vec![RecordRightEntity {
            entity: SettingsEntity {
                entity_type: SettingsEntityType::Organization,
                code: Some("org1".to_owned()),
            },
            viewable: Some(true),
            editable: Some(false),
            deletable: Some(false),
            include_subs: Some(true),
        }],
    };
    update!(
        app::settings::update_record_permissions(8).rights([right]).revision(Some(7)),
        "/k/v1/record/acl.json",
        json!({"app":8,"revision":7,"rights":[{"filterCond":"amount > 0","entities":[{
            "entity":{"type":"ORGANIZATION","code":"org1"},"viewable":true,"editable":false,"deletable":false,"includeSubs":true}]}]})
    );
}

#[test]
fn field_permissions_send_accessibility_and_field_entities() {
    let right = FieldRight {
        code: "amount".to_owned(),
        entities: vec![FieldRightEntity {
            accessibility: Accessibility::None,
            entity: SettingsEntity {
                entity_type: SettingsEntityType::FieldEntity,
                code: Some("assignee".to_owned()),
            },
            include_subs: Some(false),
        }],
    };
    update!(
        app::settings::update_field_permissions(8).rights([right]).revision(Some(7)),
        "/k/v1/field/acl.json",
        json!({"app":8,"revision":7,"rights":[{"code":"amount","entities":[{
            "accessibility":"NONE","entity":{"type":"FIELD_ENTITY","code":"assignee"},"includeSubs":false}]}]})
    );
}

#[test]
fn action_updates_send_field_and_record_url_mappings() {
    let action = AppAction {
        name: Some("Copy sale".to_owned()),
        id: Some(10),
        index: 0,
        dest_app: Some(RelatedApp {
            app: Some(9),
            code: Some("DEST".to_owned()),
        }),
        mappings: Some(vec![
            ActionMapping {
                src_type: ActionSourceType::Field,
                src_field: Some("amount".to_owned()),
                dest_field: "amount".to_owned(),
            },
            ActionMapping {
                src_type: ActionSourceType::RecordUrl,
                src_field: None,
                dest_field: "source".to_owned(),
            },
        ]),
        entities: Some(vec![Entity {
            entity_type: EntityType::USER,
            code: "user1".to_owned(),
        }]),
        filter_cond: Some("amount > 0".to_owned()),
    };
    update!(
        app::settings::update_action_settings(8)
            .actions([("copy".to_owned(), action)])
            .action(
                "link",
                AppAction {
                    index: 1,
                    ..Default::default()
                }
            )
            .revision(Some(7)),
        "/k/v1/preview/app/actions.json",
        json!({"app":8,"revision":7,"actions":{
            "copy":{"name":"Copy sale","index":"0","destApp":{"app":"9","code":"DEST"},
                "mappings":[{"srcType":"FIELD","srcField":"amount","destField":"amount"},{"srcType":"RECORD_URL","destField":"source"}],
                "entities":[{"type":"USER","code":"user1"}],"filterCond":"amount > 0"},
            "link":{"index":"1"}
        }})
    );
}

#[test]
fn app_plugin_addition_sends_ids_and_revision() {
    let response = check!(
        app::settings::add_app_plugins(8)
            .ids(["plugin-a".to_owned(), "plugin-b".to_owned()])
            .revision(Some(7)),
        "POST",
        "/k/v1/preview/app/plugins.json",
        &[],
        Some(json!({"app":8,"ids":["plugin-a","plugin-b"],"revision":7})),
        r#"{"revision":"13"}"#
    );
    assert_eq!(response.revision, 13);
}

#[test]
fn app_moves_send_destination_space() {
    check!(
        app::settings::move_app(8).space(42),
        "POST",
        "/k/v1/app/move.json",
        &[],
        Some(json!({"app":8,"space":42})),
        "{}"
    );
}

#[test]
fn space_updates_send_visibility_flags_and_permissions() {
    check!(
        space::update_space(8)
            .name("Sales")
            .is_private(false)
            .fixed_member(true)
            .use_multi_thread(true)
            .show_announcement(false)
            .show_thread_list(true)
            .show_app_list(false)
            .show_member_list(true)
            .show_related_link_list(false)
            .permissions(SpacePermissions {
                create_app: CreateAppPermission::Admin
            }),
        "PUT",
        "/k/v1/space.json",
        &[],
        Some(
            json!({"id":8,"name":"Sales","isPrivate":false,"fixedMember":true,"useMultiThread":true,
            "showAnnouncement":false,"showThreadList":true,"showAppList":false,"showMemberList":true,"showRelatedLinkList":false,
            "permissions":{"createApp":"ADMIN"}})
        ),
        "{}"
    );
}

#[test]
fn space_templates_send_members_and_guest_flags() {
    let response = check!(
        space::add_space_from_template(8, "Sales")
            .members([space_member()])
            .is_private(true)
            .is_guest(true)
            .fixed_member(false),
        "POST",
        "/k/v1/template/space.json",
        &[],
        Some(
            json!({"id":8,"name":"Sales","members":[{"entity":{"type":"USER","code":"user1"},"isAdmin":true}],
            "isPrivate":true,"isGuest":true,"fixedMember":false})
        ),
        r#"{"id":"9"}"#
    );
    assert_eq!(response.id, 9);
}

#[test]
fn space_members_send_users_and_organizations_without_response_metadata() {
    let organization = SpaceMember {
        entity: Entity {
            entity_type: EntityType::ORGANIZATION,
            code: "org1".to_owned(),
        },
        is_admin: false,
        include_subs: Some(true),
        is_implicit: None,
    };
    check!(
        space::update_space_members(8).members([space_member(), organization]),
        "PUT",
        "/k/v1/space/members.json",
        &[],
        Some(json!({"id":8,"members":[
            {"entity":{"type":"USER","code":"user1"},"isAdmin":true},
            {"entity":{"type":"ORGANIZATION","code":"org1"},"isAdmin":false,"includeSubs":true}
        ]})),
        "{}"
    );
}

#[test]
fn thread_updates_send_names_and_html_bodies() {
    check!(
        space::update_thread(8).name("Sales").body("<p>Thread body</p>"),
        "PUT",
        "/k/v1/space/thread.json",
        &[],
        Some(json!({"id":8,"name":"Sales","body":"<p>Thread body</p>"})),
        "{}"
    );
}

#[test]
fn guest_creation_sends_required_and_optional_profile_fields() {
    let minimal = GuestUser::new("user1@example.com", "password-1", "Asia/Tokyo", "User One");
    let mut full = GuestUser::new("user2@example.com", "password-2", "Asia/Tokyo", "User Two");
    full.locale = Some("ja".to_owned());
    full.image = Some("image-key".to_owned());
    full.sur_name_reading = Some("とうきょう".to_owned());
    full.given_name_reading = Some("じろう".to_owned());
    full.company = Some("Company".to_owned());
    full.division = Some("Sales".to_owned());
    full.phone = Some("03-0000-0000".to_owned());
    full.callto = Some("guest2".to_owned());
    check!(
        space::add_guest_users().guests([minimal, full]),
        "POST",
        "/k/v1/guests.json",
        &[],
        Some(json!({"guests":[
            {"code":"user1@example.com","password":"password-1","timezone":"Asia/Tokyo","name":"User One"},
            {"code":"user2@example.com","password":"password-2","timezone":"Asia/Tokyo","name":"User Two",
                "locale":"ja","image":"image-key","surNameReading":"とうきょう","givenNameReading":"じろう",
                "company":"Company","division":"Sales","phone":"03-0000-0000","callto":"guest2"}
        ]})),
        "{}"
    );
}
