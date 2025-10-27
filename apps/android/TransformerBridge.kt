package com.transformer.demo

import android.util.Log
import java.nio.ByteBuffer

/**
 * Kotlin bridge to Rust transformer library via JNI
 */
class TransformerBridge {
    
    private var nativeHandle: Long = 0
    
    companion object {
        private const val TAG = "TransformerBridge"
        
        init {
            try {
                System.loadLibrary("transformer_ffi")
                Log.d(TAG, "Loaded native library")
            } catch (e: UnsatisfiedLinkError) {
                Log.e(TAG, "Failed to load native library", e)
            }
        }
        
        // Native methods
        @JvmStatic
        external fun nativeInit(): Long
        
        @JvmStatic
        external fun nativeFree(handle: Long)
        
        @JvmStatic
        external fun nativeCreateDefaultModel(handle: Long): Int
        
        @JvmStatic
        external fun nativeLoadModel(handle: Long, data: ByteArray): Int
        
        @JvmStatic
        external fun nativeForward(handle: Long, input: FloatArray): FloatArray?
        
        @JvmStatic
        external fun nativeGetBackend(handle: Long): String
        
        @JvmStatic
        external fun nativeGetVocabSize(handle: Long): Int
    }
    
    /**
     * Initialize the transformer engine
     */
    fun init(): Boolean {
        nativeHandle = nativeInit()
        val success = nativeHandle != 0L
        Log.d(TAG, "Initialization: ${if (success) "success" else "failed"}")
        return success
    }
    
    /**
     * Free resources
     */
    fun free() {
        if (nativeHandle != 0L) {
            nativeFree(nativeHandle)
            nativeHandle = 0
        }
    }
    
    /**
     * Create a default model for testing
     */
    fun createDefaultModel(): Boolean {
        if (nativeHandle == 0L) return false
        val result = nativeCreateDefaultModel(nativeHandle)
        return result == 0
    }
    
    /**
     * Load model from bytes
     */
    fun loadModel(data: ByteArray): Boolean {
        if (nativeHandle == 0L) return false
        val result = nativeLoadModel(nativeHandle, data)
        return result == 0
    }
    
    /**
     * Run inference
     */
    fun forward(input: FloatArray): FloatArray? {
        if (nativeHandle == 0L) return null
        return nativeForward(nativeHandle, input)
    }
    
    /**
     * Get the GPU backend being used
     */
    fun getBackend(): String {
        if (nativeHandle == 0L) return "Unknown"
        return nativeGetBackend(nativeHandle)
    }
    
    /**
     * Get vocabulary size
     */
    fun getVocabSize(): Int {
        if (nativeHandle == 0L) return 0
        return nativeGetVocabSize(nativeHandle)
    }
    
    override fun finalize() {
        free()
    }
}
