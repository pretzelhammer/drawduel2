import React, { createContext, useContext, useState, ReactNode } from 'react';
import { type DrawOperation } from './ContextualCanvas';

type Nullish<T> = T | null | undefined;

export interface TestAppState {
    count: number;
    drawOps: DrawOperation[];
}

function defaultTestAppState() {
    return {
        count: 0,
        drawOps: [] as DrawOperation[],
    };
}

export interface TestAppStateContext {
    state: TestAppState;
    setState: (state: TestAppState) => void;
}

function defaultTestAppStateContext() {
    return {
        state: defaultTestAppState(),
        setState: () => {}, // no-op
    };
}

// 1. Create a context object.
// This is where we store the shared state and make it accessible to other components.
export const TestAppContext = createContext<TestAppStateContext>(
    defaultTestAppStateContext(),
);

// 2. Create a provider component.
// This wraps part of the component tree and provides the context value to its descendants.
export const TestAppContextProvider = ({
    children,
}: {
    children: ReactNode;
}) => {
    const [state, setState] = useState(defaultTestAppState()); // Local state to share

    return (
        <TestAppContext.Provider value={{ state, setState }}>
            {children} {/* Render any child components inside the provider */}
        </TestAppContext.Provider>
    );
};
