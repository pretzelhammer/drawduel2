import { type FC } from 'react';
import { useStore } from './TestZustandAppStore';

export const StoreCounter: FC = () => {
    // let state = useStore((state) => ({
    //     count: state.count,
    //     incCount: state.incCount,
    // }));
    let count = useStore((state) => state.count);
    let incCount = useStore((state) => state.incCount);
    return (
        <div>
            <div>count: {count}</div>
            <button onClick={incCount}>add count</button>
        </div>
    );
};
