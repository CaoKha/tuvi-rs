package io.github.caokha.thienhac;

import android.app.Activity;
import android.content.ActivityNotFoundException;
import android.content.Intent;
import android.content.res.AssetManager;
import android.graphics.Insets;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.text.TextUtils;
import android.util.Log;
import android.view.View;
import android.view.ViewGroup;
import android.view.WindowInsets;
import android.webkit.WebResourceRequest;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.FrameLayout;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.Socket;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;

/**
 * Runs the tuvi-web server on the phone and shows it in a WebView.
 *
 * <p>The server is the same Rust binary as the web app, built for Android and
 * shipped as {@code libtuvi_web.so}. Its asset bundle ships in the APK assets
 * and is copied to app storage after each install or update.
 */
public final class MainActivity extends Activity {
    private static final String TAG = "ThienHac";
    private static final String HOST = "127.0.0.1";
    /** Fixed: the page's cookies and local storage belong to this origin. */
    private static final int PORT = 47613;
    private static final String HOME = "http://" + HOST + ":" + PORT + "/";
    private static final int PAGE_COLOR = 0xff120a24;
    private static final long START_TIMEOUT_MS = 15_000;

    /** Outlives the activity, so a new activity reuses the running server. */
    private static Process server;

    private WebView web;

    @Override
    protected void onCreate(Bundle state) {
        super.onCreate(state);

        web = new WebView(this);
        web.setBackgroundColor(PAGE_COLOR);
        WebSettings settings = web.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);
        settings.setMediaPlaybackRequiresUserGesture(false);
        web.setWebViewClient(new Client());

        FrameLayout root = new FrameLayout(this);
        root.setBackgroundColor(PAGE_COLOR);
        root.addView(web, new FrameLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT));
        keepClearOfSystemBars(root);
        setContentView(root);

        new Thread(this::boot, "tuvi-boot").start();
    }

    @Override
    @SuppressWarnings("deprecation")
    public void onBackPressed() {
        if (web.canGoBack()) {
            web.goBack();
        } else {
            super.onBackPressed();
        }
    }

    @Override
    protected void onDestroy() {
        web.destroy();
        if (isFinishing()) {
            stopServer();
        }
        super.onDestroy();
    }

    /** Starts the server, waits for it, then opens the home page. */
    private void boot() {
        try {
            startServer();
            waitForServer();
            runOnUiThread(() -> web.loadUrl(HOME));
        } catch (Exception error) {
            Log.e(TAG, "cannot start the server", error);
            String html = "<body style=\"background:#120a24;color:#eee;font:16px sans-serif;padding:24px\">"
                    + "<h3>Thiên Hạc cannot start</h3><p>"
                    + TextUtils.htmlEncode(String.valueOf(error.getMessage()))
                    + "</p></body>";
            runOnUiThread(() -> web.loadDataWithBaseURL(null, html, "text/html", "utf-8", null));
        }
    }

    private void startServer() throws IOException {
        synchronized (MainActivity.class) {
            if (server != null && server.isAlive()) {
                return;
            }
            File assets = installAssets();
            File exe = new File(getApplicationInfo().nativeLibraryDir, "libtuvi_web.so");
            ProcessBuilder builder = new ProcessBuilder(exe.getAbsolutePath())
                    .directory(getFilesDir())
                    .redirectErrorStream(true);
            builder.environment().put("HOST", HOST);
            builder.environment().put("PORT", Integer.toString(PORT));
            builder.environment().put("TUVI_ASSETS", assets.getAbsolutePath());
            server = builder.start();
            logOutput(server);
        }
    }

    private static void stopServer() {
        synchronized (MainActivity.class) {
            if (server != null) {
                server.destroy();
                server = null;
            }
        }
    }

    /** Waits until the server accepts connections. */
    private static void waitForServer() throws IOException, InterruptedException {
        long deadline = System.currentTimeMillis() + START_TIMEOUT_MS;
        while (true) {
            try (Socket socket = new Socket()) {
                socket.connect(new InetSocketAddress(HOST, PORT), 200);
                return;
            } catch (IOException notYet) {
                Process process = server;
                if (process == null || !process.isAlive()) {
                    throw new IOException("the server stopped (see logcat, tag " + TAG + ")");
                }
                if (System.currentTimeMillis() > deadline) {
                    throw new IOException("the server did not start in time");
                }
                Thread.sleep(50);
            }
        }
    }

    /** Copies the server's output to logcat, so its pipe never fills up. */
    private static void logOutput(Process process) {
        Thread thread = new Thread(() -> {
            try (BufferedReader reader = new BufferedReader(
                    new InputStreamReader(process.getInputStream(), StandardCharsets.UTF_8))) {
                String line;
                while ((line = reader.readLine()) != null) {
                    Log.i(TAG, line);
                }
            } catch (IOException ignored) {
                // The server stopped.
            }
        }, "tuvi-log");
        thread.setDaemon(true);
        thread.start();
    }

    /**
     * Copies the asset bundle from the APK to app storage, once per install
     * or update.
     */
    private File installAssets() throws IOException {
        File dir = new File(getFilesDir(), "web");
        File stamp = new File(getFilesDir(), "web.stamp");
        String version;
        try {
            version = Long.toString(getPackageManager()
                    .getPackageInfo(getPackageName(), 0).lastUpdateTime);
        } catch (Exception error) {
            throw new IOException("cannot read the app version", error);
        }
        if (dir.isDirectory() && stamp.isFile()
                && version.equals(new String(Files.readAllBytes(stamp.toPath()), StandardCharsets.UTF_8))) {
            return dir;
        }
        deleteTree(dir);
        copyAsset(getAssets(), "web", dir);
        Files.write(stamp.toPath(), version.getBytes(StandardCharsets.UTF_8));
        return dir;
    }

    private static void copyAsset(AssetManager assets, String path, File to) throws IOException {
        String[] children = assets.list(path);
        if (children != null && children.length > 0) {
            if (!to.isDirectory() && !to.mkdirs()) {
                throw new IOException("cannot create " + to);
            }
            for (String child : children) {
                copyAsset(assets, path + "/" + child, new File(to, child));
            }
            return;
        }
        try (InputStream in = assets.open(path); OutputStream out = new FileOutputStream(to)) {
            byte[] buffer = new byte[64 * 1024];
            int read;
            while ((read = in.read(buffer)) != -1) {
                out.write(buffer, 0, read);
            }
        }
    }

    private static void deleteTree(File file) {
        File[] children = file.listFiles();
        if (children != null) {
            for (File child : children) {
                deleteTree(child);
            }
        }
        // A missing file is fine: the copy starts from scratch.
        //noinspection ResultOfMethodCallIgnored
        file.delete();
    }

    /** Android 15 draws apps under the system bars; pad the page clear of them. */
    private static void keepClearOfSystemBars(View root) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.VANILLA_ICE_CREAM) {
            return;
        }
        root.setOnApplyWindowInsetsListener((view, insets) -> {
            Insets bars = insets.getInsets(WindowInsets.Type.systemBars() | WindowInsets.Type.ime());
            view.setPadding(bars.left, bars.top, bars.right, bars.bottom);
            return WindowInsets.CONSUMED;
        });
    }

    /** Keeps the app's own pages in the WebView and opens other links in the browser. */
    private final class Client extends WebViewClient {
        @Override
        public boolean shouldOverrideUrlLoading(WebView view, WebResourceRequest request) {
            Uri url = request.getUrl();
            if (HOST.equals(url.getHost())) {
                return false;
            }
            try {
                startActivity(new Intent(Intent.ACTION_VIEW, url));
            } catch (ActivityNotFoundException error) {
                Log.w(TAG, "no app opens " + url, error);
            }
            return true;
        }
    }
}
