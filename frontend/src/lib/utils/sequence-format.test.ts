import { describe, it, expect } from 'vitest';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { renderSequence, validateSequencePattern } from './sequence-format';

type RenderCase = {
	name: string;
	pattern: string;
	prefix: string;
	year: number;
	number: number;
	want: string;
};

type ValidationCase = {
	name: string;
	pattern: string;
};

type Fixture = {
	render_cases: RenderCase[];
	validation_errors: ValidationCase[];
};

// Vitest runs with cwd = frontend/. The fixture lives in the Go testdata dir.
const fixturePath = join(
	__dirname,
	'..',
	'..',
	'..',
	'..',
	'internal',
	'format',
	'testdata',
	'render_cases.json'
);

const fixture: Fixture = JSON.parse(readFileSync(fixturePath, 'utf8'));

describe('renderSequence (parity with Go)', () => {
	for (const tc of fixture.render_cases) {
		it(tc.name, () => {
			expect(renderSequence(tc.pattern, tc.prefix, tc.year, tc.number)).toBe(tc.want);
		});
	}
});

describe('validateSequencePattern (parity with Go)', () => {
	for (const tc of fixture.render_cases) {
		it(`valid: ${tc.name}`, () => {
			expect(validateSequencePattern(tc.pattern)).toBeNull();
		});
	}
	for (const tc of fixture.validation_errors) {
		it(`invalid: ${tc.name}`, () => {
			const err = validateSequencePattern(tc.pattern);
			expect(err).not.toBeNull();
			expect(typeof err).toBe('string');
		});
	}
});
