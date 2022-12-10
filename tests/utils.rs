#![allow(clippy::redundant_clone)]

use actix_web::web::Data;
use log::info;
use martin::pg::config::{FunctionInfo, PgConfigBuilder};
use martin::pg::config::{PgConfig, TableInfo};
use martin::pg::configurator::resolve_pg_data;
use martin::pg::pool::Pool;
use martin::source::{IdResolver, Source};
use martin::srv::server::{AppState, Sources};
use std::collections::HashMap;
use std::env;
use tilejson::Bounds;

//
// This file is used by many tests and benchmarks using the #[path] attribute.
// Each function should allow dead_code as they might not be used by a specific test file.
//

pub type MockSource = (Sources, PgConfig);

#[allow(dead_code)]
pub async fn mock_config(
    functions: Option<Vec<(&'static str, FunctionInfo)>>,
    tables: Option<Vec<(&'static str, TableInfo)>>,
    default_srid: Option<i32>,
) -> PgConfig {
    let connection_string: String = env::var("DATABASE_URL").unwrap();
    info!("Connecting to {connection_string}");
    let config = PgConfigBuilder {
        connection_string: Some(connection_string),
        #[cfg(feature = "ssl")]
        ca_root_file: None,
        #[cfg(feature = "ssl")]
        danger_accept_invalid_certs: None,
        default_srid,
        pool_size: None,
        tables: tables.map(|s| {
            s.iter()
                .map(|v| (v.0.to_string(), v.1.clone()))
                .collect::<HashMap<_, _>>()
        }),
        functions: functions.map(|s| {
            s.iter()
                .map(|v| (v.0.to_string(), v.1.clone()))
                .collect::<HashMap<_, _>>()
        }),
    };
    config.finalize().expect("Unable to finalize config")
}

#[allow(dead_code)]
pub async fn mock_pool() -> Pool {
    let res = Pool::new(&mock_config(None, None, None).await).await;
    res.expect("Failed to create pool")
}

#[allow(dead_code)]
pub async fn mock_sources(
    functions: Option<Vec<(&'static str, FunctionInfo)>>,
    tables: Option<Vec<(&'static str, TableInfo)>>,
    default_srid: Option<i32>,
) -> MockSource {
    let cfg = mock_config(functions, tables, default_srid).await;
    let res = resolve_pg_data(cfg, IdResolver::default()).await;
    let res = res.expect("Failed to resolve pg data");
    (res.0, res.1)
}

#[allow(dead_code)]
pub async fn mock_app_data(sources: Sources) -> Data<AppState> {
    Data::new(AppState { sources })
}

#[allow(dead_code)]
pub async fn mock_unconfigured() -> MockSource {
    mock_sources(None, None, None).await
}

#[allow(dead_code)]
pub async fn mock_unconfigured_srid(default_srid: Option<i32>) -> MockSource {
    mock_sources(None, None, default_srid).await
}

#[allow(dead_code)]
pub async fn mock_configured() -> MockSource {
    mock_sources(mock_func_config(), mock_table_config(), None).await
}

#[allow(dead_code)]
pub async fn mock_configured_funcs() -> MockSource {
    mock_sources(mock_func_config(), None, None).await
}

#[allow(dead_code)]
pub async fn mock_configured_tables(default_srid: Option<i32>) -> MockSource {
    mock_sources(None, mock_table_config(), default_srid).await
}

pub fn mock_func_config() -> Option<Vec<(&'static str, FunctionInfo)>> {
    Some(mock_func_config_map().into_iter().collect())
}

pub fn mock_table_config() -> Option<Vec<(&'static str, TableInfo)>> {
    Some(mock_table_config_map().into_iter().collect())
}

pub fn mock_func_config_map() -> HashMap<&'static str, FunctionInfo> {
    let default = FunctionInfo::default();
    [
        (
            "function_zxy",
            FunctionInfo {
                schema: "public".to_string(),
                function: "function_zxy".to_string(),
                ..default.clone()
            },
        ),
        (
            "function_zxy_query_test",
            FunctionInfo {
                schema: "public".to_string(),
                function: "function_zxy_query_test".to_string(),
                ..default.clone()
            },
        ),
        (
            "function_zxy_row_key",
            FunctionInfo {
                schema: "public".to_string(),
                function: "function_zxy_row_key".to_string(),
                ..default.clone()
            },
        ),
        (
            "function_zxy_query",
            FunctionInfo {
                schema: "public".to_string(),
                function: "function_zxy_query".to_string(),
                ..default.clone()
            },
        ),
        (
            "function_zxy_row",
            FunctionInfo {
                schema: "public".to_string(),
                function: "function_zxy_row".to_string(),
                ..default.clone()
            },
        ),
        (
            // This function is created with non-lowercase name and field names
            "function_mixed_name",
            FunctionInfo {
                schema: "MixedCase".to_string(),
                function: "function_Mixed_Name".to_string(),
                ..default.clone()
            },
        ),
        (
            "function_zoom_xy",
            FunctionInfo {
                schema: "public".to_string(),
                function: "function_zoom_xy".to_string(),
                ..default.clone()
            },
        ),
        (
            "function_zxy2",
            FunctionInfo {
                schema: "public".to_string(),
                function: "function_zxy2".to_string(),
                ..default.clone()
            },
        ),
    ]
    .into_iter()
    .collect()
}

pub fn mock_table_config_map() -> HashMap<&'static str, TableInfo> {
    let default = TableInfo {
        srid: 4326,
        minzoom: Some(0),
        maxzoom: Some(30),
        bounds: Some(Bounds::MAX),
        extent: Some(4096),
        buffer: Some(64),
        clip_geom: Some(true),
        ..Default::default()
    };

    [
        (
            "points1",
            TableInfo {
                schema: "public".to_string(),
                table: "points1".to_string(),
                geometry_column: "geom".to_string(),
                geometry_type: Some("POINT".to_string()),
                properties: props(&[("gid", "int4")]),
                ..default.clone()
            },
        ),
        (
            "points2",
            TableInfo {
                schema: "public".to_string(),
                table: "points2".to_string(),
                geometry_column: "geom".to_string(),
                geometry_type: Some("POINT".to_string()),
                properties: props(&[("gid", "int4")]),
                ..default.clone()
            },
        ),
        (
            // This table is created with non-lowercase name and field names
            "MIXPOINTS",
            TableInfo {
                schema: "MIXEDCASE".to_string(),
                table: "mixPoints".to_string(),
                geometry_column: "geoM".to_string(),
                geometry_type: Some("POINT".to_string()),
                id_column: Some("giD".to_string()),
                properties: props(&[("tAble", "text")]),
                ..default.clone()
            },
        ),
        (
            "points3857",
            TableInfo {
                schema: "public".to_string(),
                table: "points3857".to_string(),
                srid: 3857,
                geometry_column: "geom".to_string(),
                geometry_type: Some("POINT".to_string()),
                properties: props(&[("gid", "int4")]),
                ..default.clone()
            },
        ),
        (
            "points_empty_srid",
            TableInfo {
                schema: "public".to_string(),
                table: "points_empty_srid".to_string(),
                srid: 900973,
                geometry_column: "geom".to_string(),
                geometry_type: Some("GEOMETRY".to_string()),
                properties: props(&[("gid", "int4")]),
                ..default.clone()
            },
        ),
        (
            "table_source",
            TableInfo {
                schema: "public".to_string(),
                table: "table_source".to_string(),
                geometry_column: "geom".to_string(),
                geometry_type: Some("GEOMETRY".to_string()),
                properties: props(&[("gid", "int4")]),
                ..default.clone()
            },
        ),
        (
            "table_source_multiple_geom.geom1",
            TableInfo {
                schema: "public".to_string(),
                table: "table_source_multiple_geom".to_string(),
                geometry_column: "geom1".to_string(),
                geometry_type: Some("POINT".to_string()),
                properties: props(&[("geom2", "geometry"), ("gid", "int4")]),
                ..default.clone()
            },
        ),
        (
            "table_source_multiple_geom.geom2",
            TableInfo {
                schema: "public".to_string(),
                table: "table_source_multiple_geom".to_string(),
                geometry_column: "geom2".to_string(),
                geometry_type: Some("POINT".to_string()),
                properties: props(&[("gid", "int4"), ("geom1", "geometry")]),
                ..default.clone()
            },
        ),
    ]
    .into_iter()
    .collect()
}

pub fn props(props: &[(&'static str, &'static str)]) -> HashMap<String, String> {
    props
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[allow(dead_code)]
pub fn table<'a>(mock: &'a MockSource, name: &str) -> &'a TableInfo {
    let (_, PgConfig { tables, .. }) = mock;
    tables.get(name).unwrap()
}

#[allow(dead_code)]
pub fn source<'a>(mock: &'a MockSource, name: &str) -> &'a dyn Source {
    let (sources, _) = mock;
    sources.get(name).unwrap().as_ref()
}