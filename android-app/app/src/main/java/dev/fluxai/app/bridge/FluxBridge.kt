package dev.fluxai.app.bridge

import android.util.Log

object FluxBridge {
    private const val TAG = "FluxBridge"

    init {
        try {
            System.loadLibrary("flux_shared_bindings")
            Log.d(TAG, "Successfully loaded flux core library")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Failed to load flux core library", e)
        }
    }

    external fun initialize(dataDir: String): Boolean
    external fun sendMessage(json: String): String
}
