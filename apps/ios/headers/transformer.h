#ifndef TRANSFORMER_H
#define TRANSFORMER_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Opaque handle for transformer context
typedef struct TransformerContext TransformerContext;

/// Initialize the transformer engine
/// Returns a handle to the context or NULL on failure
TransformerContext* transformer_init(void);

/// Free the transformer context
void transformer_free(TransformerContext* ctx);

/// Load a model from bytes
/// Returns 0 on success, -1 on failure
int transformer_load_model(TransformerContext* ctx, const uint8_t* model_data, size_t data_len);

/// Create a default model (for testing)
/// Returns 0 on success, -1 on failure
int transformer_create_default_model(TransformerContext* ctx);

/// Run inference
/// input: array of token IDs (as floats)
/// input_len: number of tokens
/// output: pre-allocated array for output probabilities
/// output_len: size of output array (should be >= vocab_size)
/// Returns 0 on success, -1 on failure
int transformer_forward(
    TransformerContext* ctx,
    const float* input,
    size_t input_len,
    float* output,
    size_t output_len
);

/// Get the backend name being used
/// Returns a string (do not free)
const char* transformer_get_backend(const TransformerContext* ctx);

/// Get model vocabulary size
/// Returns vocab size or -1 if no model loaded
int transformer_get_vocab_size(const TransformerContext* ctx);

#ifdef __cplusplus
}
#endif

#endif // TRANSFORMER_H
