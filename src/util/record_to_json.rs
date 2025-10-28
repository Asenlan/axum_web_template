use crate::error::AppError;
use arrow::array::RecordBatch;
use arrow::json::{writer::JsonArray, WriterBuilder};
use serde_json::Value;
// 将数据转成json格式
pub fn record_to_json(record_batchs: Vec<RecordBatch>) -> Result<Vec<Value>, AppError> {
    let mut all_json = Vec::new();
    for batch in record_batchs {
        let data: Vec<u8> = Vec::new();
        let mut writer = WriterBuilder::new()
            .with_explicit_nulls(true)
            .build::<_, JsonArray>(data);
        writer.write_batches(&[&batch])?;
        writer.finish()?;
        let data = writer.into_inner();
        // deserialize the data
        let data: Vec<Value> = serde_json::from_slice(&data)?;
        all_json.extend(data);
    }
    Ok(all_json)
}
