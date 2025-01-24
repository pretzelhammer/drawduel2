import { FC } from 'react';
import { StoreCounter } from './StoreCounter';
import { StoreCanvas } from './StoreCanvas';

export const TestZustandApp: FC = () => {
    return (
        <div>
            <StoreCounter />
            <div style={{ width: '50vw', padding: '16px 0' }}>
                <StoreCanvas lineType="smooth" mode="draw" />
            </div>
            <div style={{ width: '25vw', padding: '16px 0' }}>
                <StoreCanvas lineType="pixelated" mode="view" />
            </div>
        </div>
    );
};
