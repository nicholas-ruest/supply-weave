use datafusion::{
    arrow::{
        array::{Float64Array, StringArray, UInt32Array},
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    },
    datasource::MemTable,
    prelude::SessionContext,
};
use std::sync::Arc;
use sw_errors::{Result, WeaveError};
use sw_types::Supplier;
pub async fn regional_capacity(suppliers: &[Supplier], region: &str) -> Result<(u64, f64)> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("region", DataType::Utf8, false),
        Field::new("capacity", DataType::UInt32, false),
        Field::new("cost", DataType::Float64, false),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(
                suppliers
                    .iter()
                    .map(|s| s.region.as_str())
                    .collect::<Vec<_>>(),
            )),
            Arc::new(UInt32Array::from(
                suppliers.iter().map(|s| s.capacity).collect::<Vec<_>>(),
            )),
            Arc::new(Float64Array::from(
                suppliers.iter().map(|s| s.unit_cost).collect::<Vec<_>>(),
            )),
        ],
    )
    .map_err(|e| WeaveError::Dependency(format!("Arrow batch: {e}")))?;
    let ctx = SessionContext::new();
    ctx.register_table(
        "suppliers",
        Arc::new(
            MemTable::try_new(schema, vec![vec![batch]])
                .map_err(|e| WeaveError::Dependency(format!("DataFusion table: {e}")))?,
        ),
    )
    .map_err(|e| WeaveError::Dependency(format!("DataFusion register: {e}")))?;
    let safe = region.replace('\'', "''");
    let batches = ctx
        .sql(&format!(
            "SELECT SUM(capacity) AS c, AVG(cost) AS cost FROM suppliers WHERE region='{safe}'"
        ))
        .await
        .map_err(|e| WeaveError::Dependency(format!("DataFusion SQL: {e}")))?
        .collect()
        .await
        .map_err(|e| WeaveError::Dependency(format!("DataFusion collect: {e}")))?;
    let b = batches
        .first()
        .ok_or_else(|| WeaveError::Dependency("empty DataFusion result".into()))?;
    let cap = datafusion::arrow::util::display::array_value_to_string(b.column(0).as_ref(), 0)
        .map_err(|e| WeaveError::Dependency(e.to_string()))?
        .parse()
        .unwrap_or(0);
    let cost = datafusion::arrow::util::display::array_value_to_string(b.column(1).as_ref(), 0)
        .map_err(|e| WeaveError::Dependency(e.to_string()))?
        .parse()
        .unwrap_or(0.0);
    Ok((cap, cost))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn executes_sql() {
        let s = Supplier {
            id: "s".into(),
            name: "S".into(),
            region: "EU".into(),
            certified: true,
            capacity: 42,
            lead_days: 3,
            unit_cost: 2.0,
        };
        assert_eq!(regional_capacity(&[s], "EU").await.unwrap().0, 42);
    }
}
