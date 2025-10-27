//! FFI bindings for cross-platform integration
//! 
//! This crate provides C-compatible FFI bindings for iOS, Android, and other platforms
//! to interact with the Rust transformer core.

use std::os::raw::{c_char, c_float, c_int};
use std::ptr;
use std::slice;
use transformer_core::{GpuContext, TransformerModel, ModelConfig};

/// Opaque handle for GPU context
pub struct TransformerContext {
    gpu: GpuContext,
    model: Option<TransformerModel>,
}

/// Initialize the transformer engine
/// Returns a handle to the context or null on failure
#[no_mangle]
pub extern "C" fn transformer_init() -> *mut TransformerContext {
    match GpuContext::new() {
        Ok(gpu) => {
            let context = Box::new(TransformerContext {
                gpu,
                model: None,
            });
            Box::into_raw(context)
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Free the transformer context
#[no_mangle]
pub extern "C" fn transformer_free(ctx: *mut TransformerContext) {
    if !ctx.is_null() {
        unsafe {
            let _ = Box::from_raw(ctx);
        }
    }
}

/// Load a model from file path
#[no_mangle]
pub extern "C" fn transformer_load_model(
    ctx: *mut TransformerContext,
    model_data: *const u8,
    data_len: usize,
) -> c_int {
    if ctx.is_null() || model_data.is_null() {
        return -1;
    }

    unsafe {
        let context = &mut *ctx;
        let data = slice::from_raw_parts(model_data, data_len);
        
        match TransformerModel::from_bytes(data) {
            Ok(model) => {
                context.model = Some(model);
                0
            }
            Err(_) => -1,
        }
    }
}

/// Create a default model (for testing)
#[no_mangle]
pub extern "C" fn transformer_create_default_model(ctx: *mut TransformerContext) -> c_int {
    if ctx.is_null() {
        return -1;
    }

    unsafe {
        let context = &mut *ctx;
        let config = ModelConfig::default();
        context.model = Some(TransformerModel::new(config));
        0
    }
}

/// Run inference
/// input: array of token IDs
/// input_len: number of tokens
/// output: pre-allocated array for output probabilities
/// output_len: size of output array (should be >= vocab_size)
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn transformer_forward(
    ctx: *mut TransformerContext,
    input: *const c_float,
    input_len: usize,
    output: *mut c_float,
    output_len: usize,
) -> c_int {
    if ctx.is_null() || input.is_null() || output.is_null() {
        return -1;
    }

    unsafe {
        let context = &mut *ctx;
        let model = match &context.model {
            Some(m) => m,
            None => return -1,
        };

        let input_slice = slice::from_raw_parts(input, input_len);
        
        match transformer_core::run_inference(&context.gpu, model, input_slice) {
            Ok(result) => {
                let copy_len = result.len().min(output_len);
                let output_slice = slice::from_raw_parts_mut(output, output_len);
                output_slice[..copy_len].copy_from_slice(&result[..copy_len]);
                0
            }
            Err(_) => -1,
        }
    }
}

/// Get the backend name being used
/// Returns a null-terminated string (caller must NOT free)
#[no_mangle]
pub extern "C" fn transformer_get_backend(ctx: *const TransformerContext) -> *const c_char {
    if ctx.is_null() {
        return ptr::null();
    }

    unsafe {
        let context = &*ctx;
        let name = context.gpu.backend_name();
        // This is a static string from the enum, safe to return
        name.as_ptr() as *const c_char
    }
}

/// Get model vocabulary size
#[no_mangle]
pub extern "C" fn transformer_get_vocab_size(ctx: *const TransformerContext) -> c_int {
    if ctx.is_null() {
        return -1;
    }

    unsafe {
        let context = &*ctx;
        match &context.model {
            Some(model) => model.config.vocab_size as c_int,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Ignore in CI - requires GPU
    fn test_ffi_init_free() {
        let ctx = transformer_init();
        assert!(!ctx.is_null());
        transformer_free(ctx);
    }

    #[test]
    #[ignore] // Ignore in CI - requires GPU
    fn test_ffi_create_model() {
        let ctx = transformer_init();
        assert!(!ctx.is_null());
        
        let result = transformer_create_default_model(ctx);
        assert_eq!(result, 0);
        
        let vocab_size = transformer_get_vocab_size(ctx);
        assert_eq!(vocab_size, 1000);
        
        transformer_free(ctx);
    }

    #[test]
    #[ignore] // Ignore in CI - requires GPU
    fn test_ffi_forward() {
        let ctx = transformer_init();
        assert!(!ctx.is_null());
        
        transformer_create_default_model(ctx);
        
        let input = vec![1.0, 2.0, 3.0];
        let mut output = vec![0.0; 1000];
        
        let result = transformer_forward(
            ctx,
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        );
        
        assert_eq!(result, 0);
        
        transformer_free(ctx);
    }
}
