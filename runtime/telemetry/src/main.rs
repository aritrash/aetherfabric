// runtime/telemetry/src/main.rs

mod aggregator;
mod analyzer;
mod history;
mod metric;
mod snapshot;

use aggregator::TelemetryAggregator;

use analyzer::TelemetryAnalyzer;

use history::TelemetryHistory;

use metric::{MetricCategory, TelemetryMetric};

use snapshot::TelemetrySnapshot;

use tracing::info;

fn main() {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    info!("========================================");
    info!("AetherFabric Telemetry Runtime");
    info!("========================================");

    // -------------------------------------------------
    // Create Telemetry Infrastructure
    // -------------------------------------------------

    let mut aggregator = TelemetryAggregator::new();

    let mut history = TelemetryHistory::new();

    // -------------------------------------------------
    // Simulated Node Snapshots
    // -------------------------------------------------

    let mut pi_01 = TelemetrySnapshot::new("PI-01".to_string());

    let mut pi_02 = TelemetrySnapshot::new("PI-02".to_string());

    let mut pi_03 = TelemetrySnapshot::new("PI-03".to_string());

    // -------------------------------------------------
    // PI-01 Metrics
    // -------------------------------------------------

    pi_01.add_metric(TelemetryMetric::float(
        "cpu_usage".to_string(),
        MetricCategory::Compute,
        22.4,
        "%".to_string(),
    ));

    pi_01.add_metric(TelemetryMetric::float(
        "cpu_temperature".to_string(),
        MetricCategory::Thermal,
        47.8,
        "C".to_string(),
    ));

    pi_01.add_metric(TelemetryMetric::float(
        "available_memory".to_string(),
        MetricCategory::Memory,
        3100.0,
        "MB".to_string(),
    ));

    // -------------------------------------------------
    // PI-02 Metrics
    // -------------------------------------------------

    pi_02.add_metric(TelemetryMetric::float(
        "cpu_usage".to_string(),
        MetricCategory::Compute,
        87.6,
        "%".to_string(),
    ));

    pi_02.add_metric(TelemetryMetric::float(
        "cpu_temperature".to_string(),
        MetricCategory::Thermal,
        76.2,
        "C".to_string(),
    ));

    pi_02.add_metric(TelemetryMetric::float(
        "available_memory".to_string(),
        MetricCategory::Memory,
        420.0,
        "MB".to_string(),
    ));

    // -------------------------------------------------
    // PI-03 Metrics
    // -------------------------------------------------

    pi_03.add_metric(TelemetryMetric::float(
        "cpu_usage".to_string(),
        MetricCategory::Compute,
        18.1,
        "%".to_string(),
    ));

    pi_03.add_metric(TelemetryMetric::float(
        "cpu_temperature".to_string(),
        MetricCategory::Thermal,
        43.5,
        "C".to_string(),
    ));

    pi_03.add_metric(TelemetryMetric::float(
        "available_memory".to_string(),
        MetricCategory::Memory,
        3560.0,
        "MB".to_string(),
    ));

    // -------------------------------------------------
    // Aggregation
    // -------------------------------------------------

    aggregator.update_snapshot(pi_01.clone());

    aggregator.update_snapshot(pi_02.clone());

    aggregator.update_snapshot(pi_03.clone());

    info!("\n{}", aggregator.summary());

    // -------------------------------------------------
    // Historical Retention
    // -------------------------------------------------

    history.push_snapshot(pi_01.clone());

    history.push_snapshot(pi_02.clone());

    history.push_snapshot(pi_03.clone());

    info!("\n{}", history.summary());

    // -------------------------------------------------
    // Telemetry Analysis
    // -------------------------------------------------

    let anomalies = TelemetryAnalyzer::analyze_fabric(&aggregator);

    TelemetryAnalyzer::report(&anomalies);

    // -------------------------------------------------
    // Simulated Historical Trend
    // -------------------------------------------------

    let mut pi_02_hotter = TelemetrySnapshot::new("PI-02".to_string());

    pi_02_hotter.add_metric(TelemetryMetric::float(
        "cpu_usage".to_string(),
        MetricCategory::Compute,
        94.0,
        "%".to_string(),
    ));

    pi_02_hotter.add_metric(TelemetryMetric::float(
        "cpu_temperature".to_string(),
        MetricCategory::Thermal,
        84.0,
        "C".to_string(),
    ));

    pi_02_hotter.add_metric(TelemetryMetric::float(
        "available_memory".to_string(),
        MetricCategory::Memory,
        220.0,
        "MB".to_string(),
    ));

    history.push_snapshot(pi_02_hotter);

    // -------------------------------------------------
    // Trend Analysis
    // -------------------------------------------------

    if let Some(rising) = history.metric_rising("PI-02", "cpu_temperature") {
        info!("PI-02 thermal trend rising: {}", rising);
    }

    // -------------------------------------------------
    // Average Metric Analysis
    // -------------------------------------------------

    if let Some(avg_temp) = history.average_metric("PI-02", "cpu_temperature") {
        info!("PI-02 average temperature: {:.2} C", avg_temp);
    }

    info!("========================================");
    info!("Telemetry Runtime Complete");
    info!("========================================");
}
