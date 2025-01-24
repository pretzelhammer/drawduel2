import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import 'src/apps/global.css';
import { IndexApp } from 'src/apps/index/IndexApp.tsx';

// on prod the backend server automatically does
// this redirect, but the local vite dev server
// doesn't know about it, so we manually add it
// here so we have more prod:dev parity
if (import.meta.env.DEV) {
    if (window.location.pathname === '/mini-game') {
        window.location.pathname = '/mini-game/';
    }
    if (window.location.pathname === '/development') {
        window.location.pathname = '/development/';
    }
}

let root = createRoot(document.getElementById('index-app')!);

if (import.meta.env.DEV) {
    root.render(
        <StrictMode>
            <IndexApp />
        </StrictMode>,
    );
} else {
    root.render(<IndexApp />);
}
