(function () {
    if (window.__monochromeDownloadInjected) return;
    window.__monochromeDownloadInjected = true;

    // ── Blob registry ──
    // We need to capture blob references before they are revoked,
    // because the frontend does: createObjectURL → a.click() → revokeObjectURL
    // all synchronously, and our async handler runs after revocation.
    const blobRegistry = new Map();

    const _createObjectURL = URL.createObjectURL.bind(URL);
    URL.createObjectURL = function (obj) {
        const url = _createObjectURL(obj);
        if (obj instanceof Blob) {
            blobRegistry.set(url, obj);
        }
        return url;
    };

    const _revokeObjectURL = URL.revokeObjectURL.bind(URL);
    URL.revokeObjectURL = function (url) {
        _revokeObjectURL(url);
        // Keep the blob reference a bit longer so our async handler can use it
        setTimeout(() => blobRegistry.delete(url), 30000);
    };

    // ── Helpers ──

    function getInvoke() {
        if (window.__TAURI__?.core?.invoke) {
            return window.__TAURI__.core.invoke.bind(window.__TAURI__.core);
        }
        if (window.__TAURI__?.tauri?.invoke) {
            return window.__TAURI__.tauri.invoke.bind(window.__TAURI__.tauri);
        }
        return null;
    }

    function showToast(message, isError) {
        const existing = document.getElementById('__dl-toast');
        if (existing) existing.remove();

        const toast = document.createElement('div');
        toast.id = '__dl-toast';
        toast.textContent = message;
        Object.assign(toast.style, {
            position: 'fixed',
            bottom: '120px',
            left: '50%',
            transform: 'translateX(-50%)',
            background: isError ? '#ef4444' : '#22c55e',
            color: '#fff',
            padding: '10px 20px',
            borderRadius: '10px',
            zIndex: '999999',
            fontFamily: '-apple-system, BlinkMacSystemFont, sans-serif',
            fontSize: '13px',
            fontWeight: '600',
            boxShadow: '0 4px 16px rgba(0,0,0,0.25)',
            opacity: '0',
            transition: 'opacity 0.25s ease',
            pointerEvents: 'none',
            maxWidth: '80vw',
            textAlign: 'center',
        });

        document.body.appendChild(toast);
        requestAnimationFrame(() => {
            toast.style.opacity = '1';
        });

        setTimeout(() => {
            toast.style.opacity = '0';
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    }

    function blobToBase64(blob) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onloadend = () => {
                const result = reader.result;
                resolve(result.substring(result.indexOf(',') + 1));
            };
            reader.onerror = reject;
            reader.readAsDataURL(blob);
        });
    }

    // ── Download interception ──
    // Captures clicks on <a download="filename" href="blob:..."> elements
    // which are programmatically created by the frontend's triggerDownload().

    document.addEventListener(
        'click',
        async (event) => {
            const invoke = getInvoke();
            if (!invoke) return;

            let anchor = event.target;
            if (anchor && anchor.closest) {
                anchor = anchor.closest('a[download]');
            }
            if (!anchor || !anchor.download) return;
            if (!anchor.href || !anchor.href.startsWith('blob:')) return;

            event.preventDefault();
            event.stopImmediatePropagation();

            const filename = anchor.download;
            const blob = blobRegistry.get(anchor.href);

            if (!blob) {
                showToast('Download failed: file data not available', true);
                return;
            }

            try {
                showToast('Saving ' + filename + '...');
                const base64 = await blobToBase64(blob);
                const savedName = await invoke('save_download', {
                    filename: filename,
                    dataBase64: base64,
                });
                showToast('Saved to Files: ' + savedName);
            } catch (err) {
                console.error('[Monochrome] Download save failed:', err);
                showToast('Save failed: ' + (err.message || err), true);
            }
        },
        true,
    );
})();
