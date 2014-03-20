'use strict';
const fs = require('node:fs');
const path = require('node:path');
const childProcess = require('node:child_process');
process.chdir(path.resolve(__dirname, '..'));
const files = ["index.html"];
const token = "__AIWORKS_EVENT_TRACKING_TICKET__";
const ticket = process.env.AIWORKS_EVENT_TRACKING_TICKET;
console.log('[aiworks:event-tracking] AIWORKS_EVENT_TRACKING_TICKET=' + (ticket ? 'present' : 'missing'));
if (ticket) {
  if (!/^[A-Za-z0-9_-]{1,256}$/.test(ticket)) throw new Error('invalid event tracking ticket');
  const updates = files.map(file => {
  const stat = fs.lstatSync(file);
  if (!stat.isFile() || stat.isSymbolicLink()) throw new Error('invalid event tracking target');
  const text = fs.readFileSync(file, 'utf8');
  const parts = text.split(token);
  if (parts.length !== 2) throw new Error('event tracking ticket placeholder mismatch');
  return { file, stat, parts };
  });
  for (const { file, stat, parts } of updates) {
  const temporary = file + '.aiworks-ticket.' + process.pid + '.tmp';
  try {
    fs.writeFileSync(temporary, parts[0] + ticket + parts[1], { encoding: 'utf8', mode: stat.mode, flag: 'wx' });
    fs.renameSync(temporary, file);
  } finally {
    try { fs.unlinkSync(temporary); } catch (error) { if (error.code !== 'ENOENT') throw error; }
  }
  }
}
const build = Buffer.from("bnBtIHJ1biBidWlsZA==", 'base64').toString('utf8');
const result = childProcess.spawnSync(build, { shell: true, stdio: 'inherit' });
if (result.error) throw result.error;
if (result.status !== 0) process.exit(result.status === null ? 1 : result.status);
