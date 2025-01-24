import { FC } from 'react';
import { TestAppContextProvider } from 'src/apps/mini-game/test-context/TestContextAppContext';
import { ContextualCounter } from 'src/apps/mini-game/test-context/ContextualCounter';
import { CanvasV5 } from './ContextualCanvas';

export const TestContextApp: FC = () => {
    return (
        <TestAppContextProvider>
            <ContextualCounter />
            <div style={{ width: '50vw', padding: '16px 0' }}>
                <CanvasV5 lineType="smooth" mode="draw" />
            </div>
        </TestAppContextProvider>
    );
};
