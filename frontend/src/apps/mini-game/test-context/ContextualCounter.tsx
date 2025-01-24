import { useContext, type FC } from 'react';
import { TestAppContext } from 'src/apps/mini-game/test-context/TestContextAppContext';

export const ContextualCounter: FC = () => {
    let context = useContext(TestAppContext);
    let count = context.state.count;
    let addCount = () => {
        context.setState({
            ...context.state,
            count: count + 1,
        });
    };
    return (
        <div>
            <div>count: {count}</div>
            <button onClick={addCount}>add count</button>
        </div>
    );
};
