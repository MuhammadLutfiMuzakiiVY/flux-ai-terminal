package dev.fluxai.app

import android.app.Application
import dev.fluxai.app.bridge.FluxBridge

class FluxApp : Application() {
    override fun onCreate() {
        super.onCreate()
        
        try {
            // Initialize Flux core with app's data directory
            val dataDir = filesDir.absolutePath
            val success = FluxBridge.initialize(dataDir)
            if (success) {
                android.util.Log.i("FluxApp", "Flux Engine Core successfully initialized at $dataDir")
            } else {
                android.util.Log.e("FluxApp", "Flux Engine Core failed to initialize internally")
            }
        } catch (e: Throwable) {
            // CRITICAL: Prevent force close by catching ALL errors during native library init
            android.util.Log.e("FluxApp", "CRITICAL ERROR: Flux Engine Force-Close Prevented", e)
        }
    }
}
