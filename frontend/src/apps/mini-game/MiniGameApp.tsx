import 'src/apps/mini-game/mini-game-app.css';
// import classes from 'src/apps/mini-game/MiniGameApp.module.css';
import { type FC } from 'react';
import { TestContextApp } from './test-context/TestContextApp';
import { TestZustandApp } from './test-zustand/TestZustandApp';

export const MiniGameApp: FC = () => {
    return (
        <>
            <h1>draw duel 🎨⚔️</h1>
            <h2>mini game page 2</h2>
            <TestContextApp />
            <TestZustandApp />
        </>
    );
};
