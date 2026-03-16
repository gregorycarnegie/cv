// Scroll reveal animation using IntersectionObserver
function initScrollReveal() {
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
            }
        });
    }, { threshold: 0.1, rootMargin: '0px 0px -40px 0px' });

    document.querySelectorAll('.reveal').forEach(el => observer.observe(el));
}

// Run after WASM mounts the DOM
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => setTimeout(initScrollReveal, 100));
} else {
    setTimeout(initScrollReveal, 100);
}
