import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import 'src/apps/global.css';
import { MiniGameApp } from 'src/apps/mini-game/MiniGameApp.tsx';

let root = createRoot(document.getElementById('mini-game-app')!);

if (import.meta.env.DEV) {
    root.render(
        <StrictMode>
            <MiniGameApp />
        </StrictMode>,
    );
} else {
    root.render(<MiniGameApp />);
}
