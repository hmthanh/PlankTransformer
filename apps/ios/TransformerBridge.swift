import Foundation

/// Swift wrapper for the Rust transformer FFI
class TransformerBridge {
    private var context: OpaquePointer?
    
    init?() {
        context = transformer_init()
        guard context != nil else {
            return nil
        }
    }
    
    deinit {
        if let ctx = context {
            transformer_free(ctx)
        }
    }
    
    /// Create a default model for testing
    func createDefaultModel() -> Bool {
        guard let ctx = context else { return false }
        return transformer_create_default_model(ctx) == 0
    }
    
    /// Load model from bytes
    func loadModel(data: Data) -> Bool {
        guard let ctx = context else { return false }
        return data.withUnsafeBytes { ptr in
            guard let baseAddress = ptr.baseAddress else { return false }
            return transformer_load_model(ctx, baseAddress.assumingMemoryBound(to: UInt8.self), data.count) == 0
        }
    }
    
    /// Run inference
    func forward(input: [Float]) -> [Float]? {
        guard let ctx = context else { return nil }
        
        let vocabSize = Int(transformer_get_vocab_size(ctx))
        guard vocabSize > 0 else { return nil }
        
        var output = [Float](repeating: 0.0, count: vocabSize)
        
        let result = input.withUnsafeBufferPointer { inputPtr in
            output.withUnsafeMutableBufferPointer { outputPtr in
                transformer_forward(ctx, inputPtr.baseAddress, input.count, outputPtr.baseAddress, output.count)
            }
        }
        
        return result == 0 ? output : nil
    }
    
    /// Get the GPU backend being used
    func getBackend() -> String {
        guard let ctx = context else { return "Unknown" }
        guard let cString = transformer_get_backend(ctx) else { return "Unknown" }
        return String(cString: cString)
    }
    
    /// Get vocabulary size
    func getVocabSize() -> Int {
        guard let ctx = context else { return 0 }
        let size = transformer_get_vocab_size(ctx)
        return size > 0 ? Int(size) : 0
    }
}
