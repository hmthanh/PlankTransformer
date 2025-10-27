//! Web (WASM) bindings for the Tiny Transformer

use wasm_bindgen::prelude::*;
use transformer_core::{GpuContext, TransformerModel, ModelConfig};
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<GpuContext>> = RefCell::new(None);
    static MODEL: RefCell<Option<TransformerModel>> = RefCell::new(None);
}

/// Initialize the transformer engine
#[wasm_bindgen]
pub async fn init() -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"Initializing transformer engine...".into());
    
    match GpuContext::new() {
        Ok(ctx) => {
            let backend = ctx.backend_name().to_string();
            web_sys::console::log_1(&format!("Using backend: {}", backend).into());
            CONTEXT.with(|c| *c.borrow_mut() = Some(ctx));
            Ok(JsValue::from_str(&backend))
        }
        Err(e) => {
            Err(JsValue::from_str(&format!("Failed to initialize: {}", e)))
        }
    }
}

/// Create a default model for testing
#[wasm_bindgen]
pub fn create_default_model() -> Result<JsValue, JsValue> {
    let config = ModelConfig {
        vocab_size: 1000,
        hidden_size: 128,
        num_layers: 2,
        num_heads: 4,
        seq_length: 32,
    };
    
    let model = TransformerModel::new(config);
    MODEL.with(|m| *m.borrow_mut() = Some(model));
    
    Ok(JsValue::from_str("Model created successfully"))
}

/// Load model from bytes
#[wasm_bindgen]
pub fn load_model(data: &[u8]) -> Result<JsValue, JsValue> {
    match TransformerModel::from_bytes(data) {
        Ok(model) => {
            MODEL.with(|m| *m.borrow_mut() = Some(model));
            Ok(JsValue::from_str("Model loaded successfully"))
        }
        Err(e) => {
            Err(JsValue::from_str(&format!("Failed to load model: {}", e)))
        }
    }
}

/// Run forward pass
#[wasm_bindgen]
pub fn forward(input: Vec<f32>) -> Result<Vec<f32>, JsValue> {
    CONTEXT.with(|c| {
        MODEL.with(|m| {
            let ctx = c.borrow();
            let model = m.borrow();
            
            match (&*ctx, &*model) {
                (Some(context), Some(model)) => {
                    match transformer_core::run_inference(context, model, &input) {
                        Ok(output) => Ok(output),
                        Err(e) => Err(JsValue::from_str(&format!("Inference failed: {}", e))),
                    }
                }
                _ => Err(JsValue::from_str("Context or model not initialized")),
            }
        })
    })
}

/// Get vocabulary size
#[wasm_bindgen]
pub fn get_vocab_size() -> i32 {
    MODEL.with(|m| {
        m.borrow()
            .as_ref()
            .map(|model| model.config.vocab_size as i32)
            .unwrap_or(-1)
    })
}

/// Get model info as JSON string
#[wasm_bindgen]
pub fn get_model_info() -> String {
    MODEL.with(|m| {
        m.borrow()
            .as_ref()
            .map(|model| {
                serde_json::to_string(&model.config).unwrap_or_else(|_| "{}".to_string())
            })
            .unwrap_or_else(|| "{}".to_string())
    })
}
