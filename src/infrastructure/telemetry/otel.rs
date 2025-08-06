use opentelemetry::sdk::Resource;
use opentelemetry::trace::TraceError;
use opentelemetry::{
    sdk::trace as sdktrace,
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use tonic::metadata::{MetadataMap, MetadataValue};

pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    let signoz_ingestion_key =
        std::env::var("SIGNOZ_INGESTION_KEY").expect("SIGNOZ_INGESTION_KEY not set");
    let mut metadata = MetadataMap::new();
    metadata.insert(
        "signoz-ingestion-key",
        MetadataValue::from_str(&signoz_ingestion_key).unwrap(),
    );

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_metadata(metadata)
                .with_endpoint(std::env::var("SIGNOZ_ENDPOINT").expect("SIGNOZ_ENDPOINT not set")),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                std::env::var("APP_NAME").expect("APP_NAME not set"),
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
}
