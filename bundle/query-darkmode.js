
if (localStorage.getItem('dark-mode') === 'true' || (!('dark-mode' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.querySelector('html').classList.add('dark');
} else {
    document.querySelector('html').classList.remove('dark');
}