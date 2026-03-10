pub const CHECK_STATES_JS: &str = r#"
(function(selector, states) {
    const el = document.querySelector(selector);
    if (!el) return { error: 'notconnected' };

    const rect = el.getBoundingClientRect();

    for (const state of states) {
        if (state === 'visible') {
            if (rect.width === 0 || rect.height === 0) return { missingState: 'visible' };
            const style = window.getComputedStyle(el);
            if (style.visibility === 'hidden') return { missingState: 'visible' };
        }
        if (state === 'enabled') {
            if (el.disabled || el.closest('[aria-disabled=true]'))
                return { missingState: 'enabled' };
        }
        if (state === 'editable') {
            if (el.disabled || el.readOnly) return { missingState: 'editable' };
        }
    }
    return { ok: true };
})
"#;

pub const CHECK_STABLE_JS: &str = r#"
(function(selector) {
    return new Promise((resolve) => {
        const el = document.querySelector(selector);
        if (!el) { resolve({ error: 'notconnected' }); return; }
        
        let lastRect = el.getBoundingClientRect();
        requestAnimationFrame(() => {
            const newRect = el.getBoundingClientRect();
            const stable = lastRect.x === newRect.x && lastRect.y === newRect.y
                        && lastRect.width === newRect.width && lastRect.height === newRect.height;
            resolve(stable ? { ok: true } : { missingState: 'stable' });
        });
    });
})
"#;
