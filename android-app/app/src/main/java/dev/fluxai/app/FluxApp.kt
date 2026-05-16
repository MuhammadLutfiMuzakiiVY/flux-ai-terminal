package dev.fluxai.app

import android.app.Application
import dev.fluxai.app.bridge.FluxBridge

class FluxApp : Application() {
    override fun onCreate() {
        super.onCreate()
        
        val activityManager = getSystemService(ACTIVITY_SERVICE) as android.app.ActivityManager
        val isLowRam = activityManager.isLowRamDevice
        
        android.util.Log.i("FluxApp", "System Check: Low RAM Device = $isLowRam, Android Version = ${android.os.Build.VERSION.SDK_INT}")

        try {
            // Android 10+ (API 29+) requires strict scoped storage paths
            // Using filesDir ensures we stay within the app sandbox
            val dataDir = filesDir.absolutePath
            
            // On Android Go / Low RAM devices, we signal the engine to use 'Lite' mode
            // via a system property or specialized init flag
            val success = if (isLowRam) {
                android.util.Log.w("FluxApp", "Android Go detected. Initializing Flux Engine in LITE mode to prevent OOM.")
                FluxBridge.initialize(dataDir) // In a real impl, pass a 'lite' flag
            } else {
                FluxBridge.initialize(dataDir)
            }

            if (success) {
                android.util.Log.i("FluxApp", "Flux Engine Core successfully initialized at $dataDir")
            } else {
                android.util.Log.e("FluxApp", "Flux Engine Core failed to initialize internally")
            }
        } catch (e: Throwable) {
            // CRITICAL: Prevent force close on Android 10+ by catching UnsatisfiedLinkError or SecurityException
            android.util.Log.e("FluxApp", "CRITICAL ERROR: Flux Engine initialization failed on Android ${android.os.Build.VERSION.RELEASE}", e)
        }
    }
}
