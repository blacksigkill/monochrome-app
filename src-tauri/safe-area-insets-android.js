(function() {
    if (window.__monochromeSafeAreaInsetsAndroidInit) {
        return;
    }
    window.__monochromeSafeAreaInsetsAndroidInit = true;

    function ensureStyle() {
        if (document.getElementById('monochrome-safe-area-style')) {
            return;
        }
        const style = document.createElement('style');
        style.id = 'monochrome-safe-area-style';
        style.textContent = `
            body {
                padding-top: var(--safe-area-inset-top, 0px);
                padding-bottom: var(--safe-area-inset-bottom, 0px);
                box-sizing: border-box;
                margin: 0;
            }
        `;
        (document.head || document.documentElement).appendChild(style);
    }

    function applyInsets(top, bottom) {
        const root = document.documentElement;
        if (!root) return;
        root.style.setProperty('--safe-area-inset-top', `${top}px`);
        root.style.setProperty('--safe-area-inset-bottom', `${bottom}px`);
        if (document.body) {
            document.body.style.paddingTop = 'var(--safe-area-inset-top)';
            document.body.style.paddingBottom = 'var(--safe-area-inset-bottom)';
            document.body.style.boxSizing = 'border-box';
            document.body.style.margin = '0';
        }
    }

    function readEnvInset() {
        const el = document.createElement('div');
        el.style.position = 'absolute';
        el.style.visibility = 'hidden';
        el.style.paddingTop = 'env(safe-area-inset-top, 0px)';
        el.style.paddingBottom = 'env(safe-area-inset-bottom, 0px)';
        (document.body || document.documentElement).appendChild(el);
        const styles = window.getComputedStyle(el);
        const top = parseFloat(styles.paddingTop) || 0;
        const bottom = parseFloat(styles.paddingBottom) || 0;
        el.remove();
        return { top, bottom };
    }

    function refreshInsets() {
        const envInset = readEnvInset();
        ensureStyle();
        applyInsets(envInset.top, envInset.bottom);
    }

    function init() {
        refreshInsets();
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', refreshInsets, { once: true });
        }
        window.addEventListener('load', refreshInsets, { once: true });
        window.addEventListener('resize', refreshInsets);
        window.addEventListener('orientationchange', refreshInsets);
    }

    init();
})();
