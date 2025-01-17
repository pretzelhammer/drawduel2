// https://vitest.dev/api/

import {
    describe,
    it,
    expect,
    beforeEach,
    afterEach,
    beforeAll,
    afterAll,
} from 'vitest';

function add(a: number, b: number): number {
    return a + b;
}

// remove .skip to run this example
describe.skip('example test suite', () => {
    // optional per-group setup & cleanup
    // can be async
    beforeAll(() => {
        console.log('Setting up group...');
    });
    afterAll(() => {
        console.log('Cleaning up group...');
    });

    // optional per-test setup & teardown
    // can be async
    beforeEach(() => {
        console.log('Setting up test...');
    });
    afterEach(() => {
        console.log('Cleaning up test...');
    });

    it('should add two numbers correctly', () => {
        const result = add(2, 3);
        expect(result).toBe(5);
    });

    it('should handle negative numbers', () => {
        const result = add(-2, -3);
        expect(result).toBe(-5);
    });

    it('should handle async operations', async () => {
        const promise = new Promise((resolve) =>
            setTimeout(() => resolve(42), 100),
        );
        const result = await promise;
        expect(result).toBe(42);
    });
});
