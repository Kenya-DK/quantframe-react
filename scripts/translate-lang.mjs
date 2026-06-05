#!/usr/bin/env node
import crypto from "node:crypto";
import fs from "node:fs/promises";
import path from "node:path";

const langDir = path.resolve("public/lang");
const cacheFile = path.join(langDir, ".translation-cache.json");
const sourceLang = "en";
const targetMap = { pt: "pt-BR", tc: "zh-TW", zh: "zh-CN" };
const protectedPattern = /(<[^>]+>|\{\{[^}]+\}\})/g;
const sleepMs = 250;

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

const source = JSON.parse(await fs.readFile(path.join(langDir, `${sourceLang}.json`), "utf8"));
const targetLangs = (await fs.readdir(langDir))
  .filter((file) => file.endsWith(".json") && file !== `${sourceLang}.json` && !file.startsWith("."))
  .map((file) => path.basename(file, ".json"))
  .sort();

async function readJson(file, fallback = {}) {
  try {
    return JSON.parse(await fs.readFile(file, "utf8"));
  } catch (error) {
    if (error.code === "ENOENT") return fallback;
    throw error;
  }
}

async function writeCache(cache) {
  await fs.writeFile(cacheFile, `${JSON.stringify(cache, null, 2)}\n`);
}

function protect(value) {
  const tokens = [];
  const text = value.replace(protectedPattern, (match) => {
    const token = `__QF_${tokens.length}__`;
    tokens.push([token, match]);
    return token;
  });
  return { text, tokens };
}

function restore(value, tokens) {
  return tokens.reduce((text, [token, original]) => text.replaceAll(token, original), value);
}

function flatten(node, prefix = "", out = []) {
  if (typeof node === "string") {
    out.push({ key: prefix, value: node, ...protect(node) });
  } else if (Array.isArray(node)) {
    node.forEach((value, index) => flatten(value, `${prefix}[${index}]`, out));
  } else if (node && typeof node === "object") {
    for (const [key, value] of Object.entries(node)) flatten(value, prefix ? `${prefix}.${key}` : key, out);
  }
  return out;
}

function getByPath(node, key) {
  const parts = key.replaceAll("]", "").split(/[.[\]]/).filter(Boolean);
  let current = node;
  for (const part of parts) current = current?.[part];
  return typeof current === "string" ? current : undefined;
}

function applyMap(node, translations, index = { value: 0 }) {
  if (typeof node === "string") return translations[index.value++];
  if (Array.isArray(node)) return node.map((value) => applyMap(value, translations, index));
  if (node && typeof node === "object") {
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, applyMap(value, translations, index)]));
  }
  return node;
}

async function translateString(entry, targetLang, attempt = 1) {
  const params = new URLSearchParams({
    client: "gtx",
    sl: sourceLang,
    tl: targetMap[targetLang] ?? targetLang,
    dt: "t",
    q: entry.text,
  });

  const res = await fetch(`https://translate.googleapis.com/translate_a/single?${params}`);
  if (!res.ok) {
    if (attempt < 5) {
      await sleep(1000 * attempt);
      return translateString(entry, targetLang, attempt + 1);
    }
    throw new Error(`Google Translate failed (${res.status}) for ${targetLang}`);
  }

  const data = await res.json();
  return restore(data[0].map((item) => item[0]).join(""), entry.tokens);
}

async function translateLang(targetLang, entries, cache) {
  const targetFile = path.join(langDir, `${targetLang}.json`);
  const existing = await readJson(targetFile);
  const langCache = cache[targetLang] ??= {};
  const translations = [];
  let changed = 0;

  for (const entry of entries) {
    const entryHash = hash(entry.value);
    const cached = langCache[entry.key];
    const existingValue = getByPath(existing, entry.key);

    if (cached?.hash === entryHash && typeof existingValue === "string") {
      translations.push(existingValue);
      continue;
    }

    const translated = await translateString(entry, targetLang);
    translations.push(translated);
    langCache[entry.key] = { hash: entryHash };
    changed++;
    process.stdout.write(`${targetLang}: ${changed} changed / ${entries.length} keys\r`);
    await sleep(sleepMs);
  }

  const output = applyMap(source, translations);
  await fs.writeFile(targetFile, `${JSON.stringify(output, null, 2)}\n`);
  await writeCache(cache);
  console.log(`${targetLang}: done (${changed} translated, ${entries.length - changed} cached)`);
}

const cache = await readJson(cacheFile);
const entries = flatten(source);

for (const lang of targetLangs) await translateLang(lang, entries, cache);
await writeCache(cache);
