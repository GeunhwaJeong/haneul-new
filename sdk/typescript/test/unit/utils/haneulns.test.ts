// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, expect, test } from 'vitest';

import { isValidHaneulNSName, normalizeHaneulNSName } from '../../../src/utils';

describe('isValidHaneulNSName', () => {
	test('valid HaneulNS names', () => {
		expect(isValidHaneulNSName('example.haneul')).toBe(true);
		expect(isValidHaneulNSName('EXAMPLE.haneul')).toBe(true);
		expect(isValidHaneulNSName('@example')).toBe(true);
		expect(isValidHaneulNSName('1.example.haneul')).toBe(true);
		expect(isValidHaneulNSName('1@example')).toBe(true);
		expect(isValidHaneulNSName('a.b.c.example.haneul')).toBe(true);
		expect(isValidHaneulNSName('A.B.c.123@Example')).toBe(true);
		expect(isValidHaneulNSName('1-a@1-b')).toBe(true);
		expect(isValidHaneulNSName('1-a.1-b.haneul')).toBe(true);
		expect(isValidHaneulNSName('-@test')).toBe(false);
		expect(isValidHaneulNSName('-1@test')).toBe(false);
		expect(isValidHaneulNSName('test@-')).toBe(false);
		expect(isValidHaneulNSName('test@-1')).toBe(false);
		expect(isValidHaneulNSName('test@-a')).toBe(false);
		expect(isValidHaneulNSName('test.haneul2')).toBe(false);
		expect(isValidHaneulNSName('.haneul2')).toBe(false);
		expect(isValidHaneulNSName('test@')).toBe(false);
		expect(isValidHaneulNSName('@@')).toBe(false);
		expect(isValidHaneulNSName('@@test')).toBe(false);
		expect(isValidHaneulNSName('test@test.test')).toBe(false);
		expect(isValidHaneulNSName('@test.test')).toBe(false);
		expect(isValidHaneulNSName('#@test')).toBe(false);
		expect(isValidHaneulNSName('test@#')).toBe(false);
		expect(isValidHaneulNSName('test.#.haneul')).toBe(false);
		expect(isValidHaneulNSName('#.haneul')).toBe(false);
		expect(isValidHaneulNSName('@.test.sue')).toBe(false);
	});
});

describe('normalizeHaneulNSName', () => {
	test('normalize HaneulNS names', () => {
		expect(normalizeHaneulNSName('example.haneul')).toMatch('@example');
		expect(normalizeHaneulNSName('EXAMPLE.haneul')).toMatch('@example');
		expect(normalizeHaneulNSName('@example')).toMatch('@example');
		expect(normalizeHaneulNSName('1.example.haneul')).toMatch('1@example');
		expect(normalizeHaneulNSName('1@example')).toMatch('1@example');
		expect(normalizeHaneulNSName('a.b.c.example.haneul')).toMatch('a.b.c@example');
		expect(normalizeHaneulNSName('A.B.c.123@Example')).toMatch('a.b.c.123@example');
		expect(normalizeHaneulNSName('1-a@1-b')).toMatch('1-a@1-b');
		expect(normalizeHaneulNSName('1-a.1-b.haneul')).toMatch('1-a@1-b');

		expect(normalizeHaneulNSName('example.haneul', 'dot')).toMatch('example.haneul');
		expect(normalizeHaneulNSName('EXAMPLE.haneul', 'dot')).toMatch('example.haneul');
		expect(normalizeHaneulNSName('@example', 'dot')).toMatch('example.haneul');
		expect(normalizeHaneulNSName('1.example.haneul', 'dot')).toMatch('1.example.haneul');
		expect(normalizeHaneulNSName('1@example', 'dot')).toMatch('1.example.haneul');
		expect(normalizeHaneulNSName('a.b.c.example.haneul', 'dot')).toMatch('a.b.c.example.haneul');
		expect(normalizeHaneulNSName('A.B.c.123@Example', 'dot')).toMatch('a.b.c.123.example.haneul');
		expect(normalizeHaneulNSName('1-a@1-b', 'dot')).toMatch('1-a.1-b.haneul');
		expect(normalizeHaneulNSName('1-a.1-b.haneul', 'dot')).toMatch('1-a.1-b.haneul');

		expect(() => normalizeHaneulNSName('-@test')).toThrowError('Invalid HaneulNS name -@test');
		expect(normalizeHaneulNSName('1-a@1-b')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(normalizeHaneulNSName('1-a.1-b.haneul')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(() => normalizeHaneulNSName('-@test')).toThrowError('Invalid HaneulNS name -@test');
		expect(() => normalizeHaneulNSName('-1@test')).toThrowError('Invalid HaneulNS name -1@test');
		expect(() => normalizeHaneulNSName('test@-')).toThrowError('Invalid HaneulNS name test@-');
		expect(() => normalizeHaneulNSName('test@-1')).toThrowError('Invalid HaneulNS name test@-1');
		expect(() => normalizeHaneulNSName('test@-a')).toThrowError('Invalid HaneulNS name test@-a');
		expect(() => normalizeHaneulNSName('test.haneul2')).toThrowError('Invalid HaneulNS name test.haneul2');
		expect(() => normalizeHaneulNSName('.haneul2')).toThrowError('Invalid HaneulNS name .haneul2');
		expect(() => normalizeHaneulNSName('test@')).toThrowError('Invalid HaneulNS name test@');
		expect(() => normalizeHaneulNSName('@@')).toThrowError('Invalid HaneulNS name @@');
		expect(() => normalizeHaneulNSName('@@test')).toThrowError('Invalid HaneulNS name @@test');
		expect(() => normalizeHaneulNSName('test@test.test')).toThrowError(
			'Invalid HaneulNS name test@test.test',
		);
		expect(() => normalizeHaneulNSName('@test.test')).toThrowError('Invalid HaneulNS name @test.test');
		expect(() => normalizeHaneulNSName('#@test')).toThrowError('Invalid HaneulNS name #@test');
		expect(() => normalizeHaneulNSName('test@#')).toThrowError('Invalid HaneulNS name test@#');
		expect(() => normalizeHaneulNSName('test.#.haneul')).toThrowError('Invalid HaneulNS name test.#.haneul');
		expect(() => normalizeHaneulNSName('#.haneul')).toThrowError('Invalid HaneulNS name #.haneul');
		expect(() => normalizeHaneulNSName('@.test.sue')).toThrowError('Invalid HaneulNS name @.test.sue');
	});
});
