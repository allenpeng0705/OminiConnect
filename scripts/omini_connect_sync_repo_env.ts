/**
 * Run only from `third_party/nango/scripts/` (see sync_nango_secret_to_omini_env.sh).
 * Reads the default Nango environment API secret from Postgres and writes
 * NANGO_SECRET_KEY into the OminiConnect repo-root .env (unless --print).
 *
 * Flags:
 *   --force   Overwrite existing NANGO_SECRET_KEY in .env
 *   --print   Write only the raw secret UUID to stdout (for CI / kubectl --from-literal=…). No .env write.
 */
import { config as loadEnv } from 'dotenv';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const nangoRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(__dirname, '..', '..', '..');
const nangoEnvPath = path.join(nangoRoot, '.env');
const ominiEnvPath = path.join(repoRoot, '.env');

const force = process.argv.includes('--force');
const printOnly = process.argv.includes('--print') || process.argv.includes('--print-key');

// Quiet when printing so stdout is only the secret (for shell / kubectl capture).
loadEnv({ path: nangoEnvPath, quiet: printOnly });

if (!process.env.NANGO_ENCRYPTION_KEY) {
    console.error(`Missing NANGO_ENCRYPTION_KEY in ${nangoEnvPath}`);
    process.exit(1);
}

function readOminiNangoSecretKey(): string {
    if (!fs.existsSync(ominiEnvPath)) {
        return '';
    }
    const text = fs.readFileSync(ominiEnvPath, 'utf8');
    for (const line of text.split(/\r?\n/)) {
        const m = /^\s*NANGO_SECRET_KEY\s*=\s*(.*)$/.exec(line);
        if (m) {
            return m[1].trim().replace(/^["']|["']$/g, '');
        }
    }
    return '';
}

function upsertEnvLine(contents: string, key: string, value: string): string {
    const line = `${key}=${value}`;
    const lines = contents.split(/\r?\n/);
    let found = false;
    const next = lines.map((l) => {
        if (/^\s*#/.test(l)) {
            return l;
        }
        if (new RegExp(`^\\s*${key}\\s*=`).test(l)) {
            found = true;
            return line;
        }
        return l;
    });
    if (!found) {
        if (next.length && next[next.length - 1] !== '') {
            next.push('');
        }
        next.push(line);
    }
    return next.join('\n').replace(/\n+$/, '\n');
}

async function main(): Promise<void> {
    if (!printOnly) {
        const existing = readOminiNangoSecretKey();
        if (existing.length > 0 && !force) {
            console.error('NANGO_SECRET_KEY is already set in .env (use --force to overwrite).');
            return;
        }
    }

    const { default: db } = await import('@nangohq/database');
    const { secretService, seeders } = await import('@nangohq/shared');

    try {
        let rows: { id: number; name: string }[] = await db
            .knex('_nango_environments')
            .select('id', 'name')
            .where({ deleted: false })
            .orderBy('id', 'asc');

        if (!rows.length) {
            console.error('No Nango environments in DB; creating a local dev account + "dev" environment (no UI signup).');
            const { env } = await seeders.seedAccountEnvAndUser();
            rows = [{ id: env.id, name: env.name }];
        }

        const dev =
            rows.find((r) => r.name === 'dev') ??
            rows.find((r) => r.name !== 'prod') ??
            rows[0];

        const res = await secretService.getInternalSecretForEnv(db.knex, dev.id);
        if (res.isErr()) {
            console.error('Could not read default API secret:', res.error);
            process.exit(1);
        }

        const plain = res.value.secret;
        if (!plain) {
            console.error('Default API secret was empty.');
            process.exit(1);
        }

        if (printOnly) {
            process.stdout.write(`${plain}\n`);
            console.error(`(environment "${dev.name}", id ${dev.id})`);
            return;
        }

        const prev = fs.existsSync(ominiEnvPath) ? fs.readFileSync(ominiEnvPath, 'utf8') : '';
        const merged = upsertEnvLine(prev, 'NANGO_SECRET_KEY', plain);
        fs.writeFileSync(ominiEnvPath, merged, 'utf8');
        console.error(`Wrote NANGO_SECRET_KEY to ${ominiEnvPath} (environment "${dev.name}", id ${dev.id}).`);
    } finally {
        await db.destroy();
    }
}

main()
    .then(() => process.exit(0))
    .catch((e) => {
        console.error(e);
        process.exit(1);
    });
