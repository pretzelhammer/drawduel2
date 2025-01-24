import { create } from 'zustand';
import { DrawOperation } from './StoreCanvas';

// Define the type for the state and actions
export type StoreState = {
    count: number;
    drawOps: DrawOperation[];
    addDrawOp: (drawOp: DrawOperation) => void;
    incCount: () => void;
    resetCount: () => void;
    setCount: (count: number) => void;
};

// Create the store with typed state and actions
export const useStore = create<StoreState>((set) => ({
    count: 0,
    drawOps: [] as DrawOperation[],
    addDrawOp: (drawOp: DrawOperation) =>
        set((state) => ({
            drawOps: [...state.drawOps, drawOp],
        })),
    incCount: () => set((state) => ({ count: state.count + 1 })),
    resetCount: () => set({ count: 0 }),
    setCount: (count) => set({ count }),
}));
