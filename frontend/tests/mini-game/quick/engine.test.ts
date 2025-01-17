import { describe, test, expect, beforeAll } from 'vitest';
import { Game, ServerEvents } from '../../../src/game/mini/engine';

describe(
    'mini game engine',
    {
        timeout: 1000,
    },
    () => {
        beforeAll(async () => {});

        test('is true', () => {
            expect(true).toBeTruthy();
        });
    },
);
