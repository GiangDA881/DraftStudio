'use strict';
/* Draft Studio — UI. Vanilla JS, no build step. The timeline maths mirror
   capcut-core/src/build.rs so what you see is what CapCut gets. */

// ---------------------------------------------------------------- helpers
const $ = (s, r = document) => r.querySelector(s);
function h(tag, attrs, ...kids) {
  const el = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs || {})) {
    if (v == null || v === false) continue;
    if (k === 'class') el.className = v;
    else if (k === 'style' && typeof v === 'object') { for (const [sk, sv] of Object.entries(v)) sk.startsWith('--') ? el.style.setProperty(sk, sv) : (el.style[sk] = sv); }
    else if (k.startsWith('on')) el.addEventListener(k.slice(2), v);
    else if (k === 'html') el.innerHTML = v;
    else if (k in el && k !== 'list' && k !== 'type') el[k] = v;
    else el.setAttribute(k, v === true ? '' : v);
  }
  for (const c of kids.flat(Infinity)) if (c != null && c !== false) el.append(c instanceof Node ? c : document.createTextNode(String(c)));
  return el;
}
const P = {
  folder: 'M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z',
  image: 'M4 5h16v14H4z M4 15l4-4 5 5 3-3 4 4 M15.5 9.5h.01',
  layers: 'M12 3l9 5-9 5-9-5z M3 13l9 5 9-5',
  spark: 'M12 3v4 M12 17v4 M3 12h4 M17 12h4 M6 6l2.5 2.5 M15.5 15.5L18 18 M6 18l2.5-2.5 M15.5 8.5L18 6',
  music: 'M9 18V6l11-2v12 M9 18a3 3 0 1 1-3-3 3 3 0 0 1 3 3z M20 16a3 3 0 1 1-3-3 3 3 0 0 1 3 3z',
  gear: 'M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1.1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1.1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z',
  play: 'M7 4.5v15l13-7.5z', pause: 'M7 4h3.5v16H7z M13.5 4H17v16h-3.5z',
  plus: 'M12 5v14 M5 12h14', trash: 'M4 7h16 M9 7V4h6v3 M6 7l1 13h10l1-13',
  up: 'M12 19V5 M6 11l6-6 6 6', down: 'M12 5v14 M6 13l6 6 6-6',
  export: 'M12 3v12 M7 8l5-5 5 5 M5 15v4a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-4',
  refresh: 'M20 11a8 8 0 1 0-2.3 5.7 M20 4v7h-7', wrench: 'M14.7 6.3a4 4 0 0 0-5.4 5.4L3 18l3 3 6.3-6.3a4 4 0 0 0 5.4-5.4l-2.5 2.5-2.4-.6-.6-2.4z',
  film: 'M4 4h16v16H4z M8 4v16 M16 4v16 M4 8h4 M4 12h4 M4 16h4 M16 8h4 M16 12h4 M16 16h4',
  check: 'M5 12l5 5 9-10', x: 'M6 6l12 12 M18 6L6 18', copy: 'M9 9h11v11H9z M5 15V4h11',
  capcut: 'M4 7l8-4 8 4v10l-8 4-8-4z M8 9.5l4 2.5 4-2.5 M12 12v5',
  skipb: 'M6 5v14 M19 5l-10 7 10 7z', save: 'M5 3h11l3 3v15H5z M8 3v5h7 M8 21v-7h8v7',
  open: 'M14 3h7v7 M10 14L21 3 M19 14v6H4V5h6', voice: 'M12 3a3 3 0 0 1 3 3v6a3 3 0 0 1-6 0V6a3 3 0 0 1 3-3z M5 11a7 7 0 0 0 14 0 M12 18v3',
  dup: 'M8 8h12v12H8z M4 16V4h12', eye: 'M2 12s4-7 10-7 10 7 10 7-4 7-10 7S2 12 2 12z M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z',
  agent: 'M4 5h16v14H4z M8 10l3 2-3 2 M13 14h3',
};
function icon(name, cls = '') {
  const s = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
  s.setAttribute('viewBox', '0 0 24 24'); s.setAttribute('class', 'i ' + cls); s.setAttribute('aria-hidden', 'true');
  for (const d of (P[name] || '').split(/ (?=M)/)) { const p = document.createElementNS('http://www.w3.org/2000/svg', 'path'); p.setAttribute('d', d); s.append(p); }
  return s;
}
/** replaceChildren that skips null / false (the DOM would print them as text). */
function put(el, ...kids) { el.replaceChildren(...kids.flat(Infinity).filter(k => k != null && k !== false)); }
const fileUrl = p => '/api/file?path=' + encodeURIComponent(p);
const base = p => (p || '').split(/[\\/]/).pop();
const fmtT = s => { s = Math.max(0, s || 0); const m = Math.floor(s / 60); return `${String(m).padStart(2, '0')}:${(s - m * 60).toFixed(1).padStart(4, '0')}`; };
const fmtD = s => s >= 60 ? `${Math.floor(s / 60)}:${String(Math.round(s % 60)).padStart(2, '0')}` : `${(+s || 0).toFixed(1)}s`;
const clamp = (v, a, b) => Math.min(b, Math.max(a, v));
const num = (v, d) => { const n = parseFloat(v); return Number.isFinite(n) ? n : d; };
const clone = o => JSON.parse(JSON.stringify(o));

async function call(method, path, body) {
  const r = await fetch(path, { method, headers: body ? { 'content-type': 'application/json' } : {}, body: body ? JSON.stringify(body) : undefined });
  const t = await r.text();
  let v = null; try { v = t ? JSON.parse(t) : null; } catch { v = { error: t }; }
  if (!r.ok) throw new Error((v && v.error) || `HTTP ${r.status}`);
  return v;
}
const api = { get: p => call('GET', p), post: (p, b) => call('POST', p, b || {}), put: (p, b) => call('PUT', p, b), del: p => call('DELETE', p) };

function toast(msg, kind = '', act) {
  const t = h('div', { class: 'toast ' + kind }, kind === 'err' ? icon('x', 'sm') : icon(kind === 'agent' ? 'agent' : 'check', 'sm'), h('div', { class: 'grow' }, msg),
    act ? h('button', { class: 'btn sm', onclick: () => { t.remove(); act.onclick(); } }, act.label) : null);
  $('#toasts').append(t);
  setTimeout(() => t.remove(), kind === 'err' ? 9000 : act ? 9000 : 3500);
}

// ---------------------------------------------------------------- state
const CANVAS = [
  ['1920x1080', 'YouTube 16:9 · 1080p'], ['1080x1920', 'Shorts / TikTok 9:16'], ['1080x1080', 'Vuông 1:1'],
  ['3840x2160', '4K 16:9'], ['1280x720', '720p 16:9'],
];
const S = {
  status: null, catalog: { graphics: [], motion: [], transitions: [], luts: [], video_effects: [] }, drafts: null, specs: [], settings: {},
  mcp: { mcp_exe: '', clients: [] },
  spec: null, slug: null, saved: true, tab: 'media', sel: null,
  t: 0, playing: false, zoom: 40, info: {}, gfx: {}, motion: {}, building: null,
};
/** Agent activity (see "agent activity" below). on: agent mode; stage: build call on the stage;
 *  sel: call whose log is shown; home: the user's project while in agent mode. */
const A = {
  calls: [], byKey: {}, seq: {}, seen: {}, lastId: 0, ready: false, es: null, sig: '',
  on: false, stage: null, sel: null, home: null, preview: false, term: null, userSeek: false, autoSeek: false, flashClip: null,
  follow: (() => { try { return localStorage.getItem('ccds.followAgent') !== '0'; } catch { return true; } })(),
};
function newSpec(name = 'Dự án mới') {
  return { project_name: name, width: 1920, height: 1080, fps: 30, sections: [], scenes: [], overlays: [], voiceovers: [], bgm: [],
    fit_scenes_to_voice: true, register_to_capcut: true, film_preview: false, studio: { accent: '#E8B04B', font: 'Segoe UI' } };
}
/** Scenes live in a section (`sec` = its index) or, with sec null, in the free list after the sections. */
const sceneList = sec => sec == null ? S.spec.scenes : S.spec.sections[sec].scenes;
const allScenes = () => [...S.spec.sections.flatMap(x => x.scenes), ...S.spec.scenes];
const secLabel = (x, i) => `Đoạn ${i + 1}` + ((x.desc || x.voice.desc || base(x.voice.path)) ? ' · ' + (x.desc || x.voice.desc || base(x.voice.path)) : '');
const craft = id => (S.status && S.status.crafts || []).find(c => c.id === id) || { ready: false, name: id };
const tpl = id => S.catalog.graphics.find(t => t.id === id);

// ---------------------------------------------------------------- media info
const probing = {};
function info(p) { return S.info[p]; }
const MAX_DUR = 6 * 3600; // a media length past this is a misread; never let it reach the layout
async function probe(p) {
  if (!p || S.info[p] || probing[p]) return;
  probing[p] = true;
  try { const r = await api.get('/api/probe?path=' + encodeURIComponent(p)); const d = (r.info.duration_us || 0) / 1e6; S.info[p] = { kind: r.kind, exists: r.exists, duration: d > 0 && d < MAX_DUR ? d : 0, width: r.info.width, height: r.info.height }; }
  catch { S.info[p] = { kind: 'other', exists: false, duration: 0 }; }
  delete probing[p];
  scheduleRender();
}
function probeAll() {
  const sp = S.spec; if (!sp) return;
  for (const x of [...sp.sections.map(s => s.voice), ...allScenes(), ...sp.voiceovers, ...sp.bgm, ...sp.overlays.filter(o => o.path)]) probe(x.path);
}
const kindOf = p => { const e = (p || '').split('.').pop().toLowerCase();
  if (['jpg', 'jpeg', 'png', 'webp', 'bmp', 'gif'].includes(e)) return 'image';
  if (['mp4', 'mov', 'mkv', 'webm', 'avi', 'm4v'].includes(e)) return 'video';
  if (['mp3', 'wav', 'm4a', 'aac', 'flac', 'ogg', 'opus'].includes(e)) return 'audio'; return 'other'; };

// ---------------------------------------------------------------- timeline (mirror of build.rs)
// Scene entries: { s, sec, k, n, start, dur, auto } (sec null = free scene, k = index in its list,
// n = position on the main track). Voice entries: { a, sec, i, start, dur } (sec set = section voice).
const MIN_PHOTO = 2, MIN_CLIP = 0.1;
function compute() {
  const sp = S.spec, out = { scenes: [], voice: [], overlays: [], lanes: 1, bgm: [], total: 0, mainEnd: 0, voiceEnd: 0, sectionStarts: [], sectionsEnd: 0, errors: [] };
  if (!sp) return out;
  const vlen = a => a.duration_sec > 0 ? a.duration_sec : (info(a.path) || {}).duration || 0;
  // 1. sections: voice N+1 starts where section N (voice + gap) ends; its scenes are fitted to it
  let t = 0;
  sp.sections.forEach((x, si) => {
    const d = vlen(x.voice), target = d + Math.max(0, x.gap_after_sec || 0);
    out.sectionStarts.push(t); out.voice.push({ a: x.voice, sec: si, start: t, dur: d });
    const items = x.scenes.map(s => {
      const inf = info(s.path), trim = s.trim_start_sec || 0, fixed = s.duration_sec > 0 ? s.duration_sec : null;
      // a video not probed yet stretches like a photo until its length is known
      if (kindOf(s.path) !== 'video' || !inf) return { video: null, fixed };
      if (inf.duration > trim) return { video: inf.duration - trim, fixed };
      if (fixed == null) out.errors.push({ sec: si, msg: `Không đo được độ dài video ${base(s.path)} — đặt thời lượng cho cảnh này.` });
      return { video: null, fixed };
    });
    const r = fitSection(target, items), heard = x.voice.duration_sec > 0 || info(x.voice.path); // not probed yet: no verdict
    if (heard && !d) out.errors.push({ sec: si, msg: `Không đo được thời lượng giọng ${base(x.voice.path)}.` });
    else if (heard && r.error) out.errors.push({ sec: si, msg: r.error[0].toUpperCase() + r.error.slice(1) });
    let at = t;
    x.scenes.forEach((s, k) => { out.scenes.push({ s, sec: si, k, n: out.scenes.length, start: at, dur: r.durs[k], auto: items[k].fixed == null }); at += r.durs[k]; });
    t += target;
  });
  out.sectionsEnd = t;
  // 2. free voiceovers: own start, else back to back after the sections
  let cur = t;
  sp.voiceovers.forEach((a, i) => {
    const d = vlen(a), st = a.start_sec != null ? a.start_sec : cur; cur = st + d;
    out.voice.push({ a, sec: null, i, start: st, dur: d });
  });
  out.voiceEnd = Math.max(0, ...out.voice.map(v => v.start + v.dur));
  // 3. free scenes after the sections; with fit_scenes_to_voice photos share the narration left
  const pre = sp.scenes.map(s => {
    const k = kindOf(s.path), inf = info(s.path) || {}, trim = s.trim_start_sec || 0;
    if (s.duration_sec > 0) return s.duration_sec;
    if (k === 'video') return inf.duration > trim ? inf.duration - trim : 5;
    return null;
  });
  const auto = pre.filter(d => d == null).length, fixed = pre.reduce((a, d) => a + (d || 0), 0), left = Math.max(0, out.voiceEnd - out.sectionsEnd);
  const autoDur = sp.fit_scenes_to_voice && auto && left > fixed ? (left - fixed) / auto : 5;
  sp.scenes.forEach((s, k) => { const d = Math.max(MIN_CLIP, pre[k] == null ? autoDur : pre[k]); out.scenes.push({ s, sec: null, k, n: out.scenes.length, start: t, dur: d, auto: pre[k] == null }); t += d; });
  out.mainEnd = Math.max(t, ...out.scenes.map(c => c.start + c.dur));
  // footage / pictures (kind file, e.g. B-roll) on the lower lanes, graphics and motion above (build.rs `stack`)
  const ovs = sp.overlays.map((o, i) => ({ i, file: o.kind === 'file', start: ovBase(o, out) + (o.start_sec || 0), dur: ovDur(o) })).sort((a, b) => a.start - b.start);
  let base = 0;
  for (const group of [ovs.filter(c => c.file), ovs.filter(c => !c.file)]) {
    const lanes = [];
    for (const c of group) {
      let k = lanes.findIndex(l => l <= c.start + 1e-6);
      if (k < 0) { lanes.push(0); k = lanes.length - 1; }
      lanes[k] = c.start + c.dur; c.lane = base + k; out.overlays.push(c);
    }
    base += lanes.length;
  }
  out.lanes = Math.max(1, base);
  const ovEnd = Math.max(0, ...out.overlays.map(o => o.start + o.dur));
  out.total = Math.min(MAX_DUR, Math.max(out.mainEnd, out.voiceEnd, ovEnd));
  sp.bgm.forEach((b, i) => {
    const file = b.duration_sec > 0 ? b.duration_sec : (info(b.path) || {}).duration || 0;
    const st = b.start_sec || 0; if (!file || st >= out.total) return;
    let at = st, n = 0;
    while (at < out.total - 1e-6 && n < 500) { const d = Math.min(out.total - at, Math.max(file, 0.5)); out.bgm.push({ i, start: at, dur: d, loop: n++ > 0, file }); at += d; if (b.loop_to_fill === false) break; }
  });
  return out;
}
/** Mirror of build.rs `fit_section`: lengths adding up to `target` (voice + gap). Set durations
 *  stay, photos absorb the slack (≥ MIN_PHOTO while videos are trimmed), videos are trimmed in
 *  proportion and never stretched. On an error (the build refuses it) the clips keep a best guess. */
function fitSection(target, items) {
  const durs = items.map(() => 0), videos = [], photos = []; let fixed = 0, error = null;
  items.forEach((it, i) => {
    if (it.fixed != null) { durs[i] = it.video != null ? Math.min(it.fixed, it.video) : it.fixed; fixed += durs[i]; }
    else (it.video != null ? videos : photos).push(i);
  });
  const room = Math.max(0, target - fixed), footage = videos.reduce((a, i) => a + items[i].video, 0);
  if (!items.length) error = 'chưa có cảnh nào.';
  else if (fixed > target + 1e-6) error = `cảnh đặt sẵn thời lượng dài ${fixed.toFixed(2)}s, quá giọng đọc + khoảng lặng (${target.toFixed(2)}s).`;
  else if (!photos.length && footage < room - 1e-6) error = `thiếu hình ${(room - footage).toFixed(2)}s — video không kéo giãn được; thêm cảnh hoặc một ảnh (ảnh tự giãn).`;
  const forVideos = photos.length ? Math.min(footage, room - Math.min(MIN_PHOTO, room / (photos.length + videos.length)) * photos.length) : room;
  const k = footage > 0 ? Math.min(1, forVideos / footage) : 0;
  for (const i of videos) durs[i] = items[i].video * k;
  for (const i of photos) durs[i] = (room - Math.min(footage, forVideos)) / photos.length;
  const short = durs.findIndex(d => d < MIN_CLIP);
  if (!error && short >= 0) error = `cảnh ${short + 1} chỉ còn ${durs[short].toFixed(2)}s — bớt cảnh trong đoạn này.`;
  return { durs, error };
}
/** Where an overlay's start_sec counts from: its section's start, else 0. */
function ovBase(o, tl) { return o.section != null ? (tl.sectionStarts[o.section] || 0) : 0; }
function ovDur(o) {
  if (o.duration_sec > 0) return o.duration_sec;
  if (o.graphic && tpl(o.graphic.template)) return tpl(o.graphic.template).suggested_sec;
  if (o.path && kindOf(o.path) === 'video') return (info(o.path) || {}).duration || 4;
  return 4;
}

// ---------------------------------------------------------------- persistence
let saveTimer = null;
function changed(opts = {}) {
  if (A.on) return scheduleRender(opts.keep); // agent mode is read-only and never saved
  S.saved = false; renderTop();
  clearTimeout(saveTimer); saveTimer = setTimeout(save, 600);
  if (opts.probe) probeAll();
  if (opts.gfx) ensureGraphics();
  scheduleRender(opts.keep);
}
async function save() {
  if (!S.spec || A.on) return;
  if (!S.slug) S.slug = slugify(S.spec.project_name) + '-' + Math.random().toString(36).slice(2, 6);
  try { await api.put('/api/specs/' + S.slug, S.spec); S.saved = true; try { localStorage.setItem('ccds.slug', S.slug); } catch {} }
  catch (e) { toast('Không lưu được dự án: ' + e.message, 'err'); }
  renderTop();
}
function slugify(s) {
  return (s || 'du-an').normalize('NFD').replace(/[\u0300-\u036f]/g, '').replace(/đ/g, 'd').replace(/Đ/g, 'D')
    .toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '').slice(0, 40) || 'du-an';
}
async function openSpec(slug) {
  try {
    const sp = await api.get('/api/specs/' + slug);
    S.spec = Object.assign(newSpec(), sp); S.spec.studio = Object.assign({ accent: '#E8B04B', font: 'Segoe UI' }, sp.studio || {});
    S.slug = slug; S.sel = null; S.t = 0; S.saved = true; stop();
    try { localStorage.setItem('ccds.slug', slug); } catch {}
    probeAll(); ensureGraphics(); render();
  } catch (e) { toast('Không mở được dự án: ' + e.message, 'err'); }
}
async function refreshSpecs() { try { S.specs = await api.get('/api/specs'); } catch { S.specs = []; } }

// ---------------------------------------------------------------- crafts: graphics
function gKey(g) { return JSON.stringify([g.template, g.fields || {}, g.accent || '', g.font || '', S.spec.width, S.spec.height]); }
function effGraphic(g) { // overlay graphic with project defaults applied
  return { template: g.template, fields: g.fields || {}, accent: g.accent || S.spec.studio.accent, font: g.font || S.spec.studio.font };
}
async function renderGraphic(g) {
  const eg = effGraphic(g), k = gKey(eg);
  if (S.gfx[k]) return S.gfx[k];
  if (!craft('vectorcraft').ready) return (S.gfx[k] = { error: 'Chưa có VectorCraft', key: k, soft: true });
  const ent = S.gfx[k] = { pending: true };
  try { const r = await api.post('/api/graphic/preview', { graphic: eg, width: S.spec.width, height: S.spec.height, project_name: S.spec.project_name }); Object.assign(ent, { png: r.png, doc: r.vectorcraft_doc, pending: false }); }
  catch (e) { Object.assign(ent, { error: e.message, pending: false }); }
  scheduleRender(); return ent;
}
let gfxTimer = null;
function ensureGraphics() {
  clearTimeout(gfxTimer);
  if (A.on) return; // the agent's own build draws its graphics
  gfxTimer = setTimeout(() => { for (const o of S.spec ? S.spec.overlays : []) if (o.graphic && o.kind !== 'file') renderGraphic(o.graphic); }, 350);
}
const gfxOf = o => o.graphic ? S.gfx[gKey(effGraphic(o.graphic))] : null;
function mKey(o, dur) { return JSON.stringify([o.graphic ? gKey(effGraphic(o.graphic)) : o.path, (o.motion || {}).preset, +dur.toFixed(2)]); }
async function renderMotion(o, dur) {
  const k = mKey(o, dur), ent = S.motion[k] = { pending: true };
  render();
  try {
    const body = { preset: o.motion.preset, duration_sec: dur, width: S.spec.width, height: S.spec.height, fps: S.spec.fps, project_name: S.spec.project_name };
    if (o.graphic) body.graphic = effGraphic(o.graphic); else body.image = o.path;
    const r = await api.post('/api/motion/preview', body);
    Object.assign(ent, { video: r.video, project: r.project, pending: false });
    toast('EffectCraft đã render chuyển động “' + o.motion.preset + '”.', 'ok');
  } catch (e) { Object.assign(ent, { error: e.message, pending: false }); toast('EffectCraft: ' + e.message, 'err'); }
  render();
}

// ---------------------------------------------------------------- file picking
async function pick(kind, multiple) {
  try { const r = await api.post('/api/pick', { kind, multiple }); return r.paths || []; }
  catch (e) { toast('Không mở được hộp chọn file: ' + e.message, 'err'); return []; }
}
async function addScenes(folder, sec = null) {
  const paths = folder ? await pick('folder') : await pick('visual', true);
  if (!paths.length) return;
  const list = sceneList(sec);
  for (const p of paths) list.push({ path: p, ken_burns: kindOf(p) === 'image', zoom_start: 1, zoom_end: 1.06, pan: 'none' });
  S.sel = { type: 'scene', i: list.length - paths.length, sec };
  changed({ probe: true });
  toast(`Đã thêm ${paths.length} cảnh.`, 'ok');
}
/** One section per picked voice file; its scenes are then fitted to that voice. */
async function addSections() {
  const paths = await pick('audio', true); if (!paths.length) return;
  for (const p of paths) S.spec.sections.push({ voice: { path: p, volume: 1 }, scenes: [], gap_after_sec: 0 });
  S.sel = { type: 'section', i: S.spec.sections.length - paths.length }; S.tab = 'media'; changed({ probe: true });
  toast(`Đã thêm ${paths.length} đoạn — thêm cảnh vào từng đoạn.`, 'ok');
}
async function addVoice() {
  const paths = await pick('audio', true); if (!paths.length) return;
  for (const p of paths) S.spec.voiceovers.push({ path: p, volume: 1 });
  S.sel = { type: 'voice', i: S.spec.voiceovers.length - 1 }; changed({ probe: true });
}
async function addBgm() {
  const [p] = await pick('audio'); if (!p) return;
  S.spec.bgm.push({ path: p, volume: 0.3, fade_in_sec: 1, fade_out_sec: 2, loop_to_fill: true, duck_under_voice: true, duck_ratio: 0.4 });
  S.sel = { type: 'bgm', i: S.spec.bgm.length - 1 }; changed({ probe: true });
}
function addGraphic(t) {
  const fields = {}; for (const f of t.fields) fields[f.key] = f.default;
  S.spec.overlays.push({ kind: 'graphic', graphic: { template: t.id, fields }, ...placeAt(S.t), duration_sec: t.suggested_sec, x: 0, y: 0, scale: 1, opacity: 1, fade_in_sec: 0.4, fade_out_sec: 0.4 });
  S.sel = { type: 'overlay', i: S.spec.overlays.length - 1 };
  changed({ gfx: true });
}
async function addFileOverlay() {
  const [p] = await pick('visual'); if (!p) return;
  S.spec.overlays.push({ kind: 'file', path: p, ...placeAt(S.t), x: 0, y: 0, scale: 1, opacity: 1, fade_in_sec: 0.3, fade_out_sec: 0.3 });
  S.sel = { type: 'overlay', i: S.spec.overlays.length - 1 }; changed({ probe: true });
}
async function motionFromImage(preset) {
  const [p] = await pick('images'); if (!p) return;
  S.spec.overlays.push({ kind: 'motion', path: p, motion: { preset, format: 'prores' }, ...placeAt(S.t), duration_sec: 4, x: 0, y: 0, scale: 1, opacity: 1, fade_in_sec: 0, fade_out_sec: 0 });
  S.sel = { type: 'overlay', i: S.spec.overlays.length - 1 }; changed({ probe: true });
}
/** A new overlay at time t: timed from the section playing there, so it moves with that section. */
function placeAt(t) {
  const tl = compute(), si = t < tl.sectionsEnd ? tl.sectionStarts.findLastIndex(st => st <= t + 1e-6) : -1;
  return si >= 0 ? { section: si, start_sec: +(t - tl.sectionStarts[si]).toFixed(2) } : { start_sec: +t.toFixed(2) };
}
function removeSel() {
  const s = S.sel; if (!s || A.on) return;
  if (s.type === 'section') return removeSection(s.i);
  const arr = s.type === 'scene' ? sceneList(s.sec) : { overlay: S.spec.overlays, voice: S.spec.voiceovers, bgm: S.spec.bgm }[s.type];
  arr.splice(s.i, 1); S.sel = null; changed();
}
/** An overlay timed from a removed section keeps its time; on reorder it moves with its section. */
function removeSection(i) {
  const x = S.spec.sections[i];
  if (x.scenes.length && !confirm(`Xoá ${secLabel(x, i)} cùng ${x.scenes.length} cảnh của nó?`)) return;
  const tl = compute();
  for (const o of S.spec.overlays) {
    if (o.section === i) { o.start_sec = +(ovBase(o, tl) + (o.start_sec || 0)).toFixed(3); delete o.section; }
    else if (o.section > i) o.section--;
  }
  S.spec.sections.splice(i, 1); S.sel = null; changed();
}
function moveSection(i, d) {
  const j = i + d, secs = S.spec.sections; if (j < 0 || j >= secs.length) return;
  for (const o of S.spec.overlays) if (o.section === i) o.section = j; else if (o.section === j) o.section = i;
  [secs[i], secs[j]] = [secs[j], secs[i]];
  if (S.sel && S.sel.type === 'section' && S.sel.i === i) S.sel.i = j;
  changed();
}

// ---------------------------------------------------------------- render scheduling
let rq = null, keepInsp = false;
function scheduleRender(keep) { keepInsp = keepInsp || !!keep; if (rq) return; rq = requestAnimationFrame(() => { rq = null; const k = keepInsp; keepInsp = false; render(k); }); }
function render(keep) {
  if (!S.spec) return;
  renderTop(); renderRail(); renderPanel(); renderStage(); if (!keep) renderInsp(); renderTimeline();
}

// ---------------------------------------------------------------- top bar
function renderTop() {
  const top = $('#top'); if (!top || !S.spec) return;
  const name = $('#pname');
  if (document.activeElement !== name) name.value = S.spec.project_name;
  name.readOnly = A.on; name.title = A.on ? 'Bản dựng trực tiếp của agent' : '';
  if (A.on) put($('#savestate'), h('span', { class: 'tag capcut', title: 'Đang hiển thị trực tiếp các bước của Agent · tự động lưu vào Dự án' }, '⚡ Live Agent'));
  else put($('#savestate'), S.saved ? icon('check', 'sm') : h('span', { class: 'spinner', style: { width: '12px', height: '12px' } }), S.saved ? 'Đã lưu' : 'Đang lưu…');
  put($('#agentchip'), mcpChip(), agentChip());
  put($('#topacts'), A.on
    ? [h('button', { class: 'btn primary', onclick: () => exitAgent('media'), title: 'Chuyển sang chế độ chỉnh sửa trực tiếp dự án này trên Timeline' }, icon('wrench', 'sm'), 'Sửa trên Timeline'),
       h('button', { class: 'btn', onclick: openBuild }, icon('export', 'sm'), 'Xuất lại CapCut')]
    : h('button', { class: 'btn primary', id: 'buildbtn', onclick: openBuild }, icon('export'), 'Xuất sang CapCut'));
  const cs = $('#craftchips'); put(cs, ...['vectorcraft', 'effectcraft', 'filmcraft'].map(id => {
    const c = craft(id);
    return h('button', { class: 'chip', title: (c.role || '') + (c.ready ? '\n' + (c.version || c.path || '') : '\nChưa build — xem Cài đặt'), onclick: () => { S.tab = 'settings'; render(); } },
      h('span', { class: 'dot ' + (S.status ? (c.ready ? 'on' : 'off') : '') }), c.name || id);
  }));
}
function buildShell() {
  const app = $('#app');
  put(app, 
    h('header', { class: 'top', id: 'top' },
      h('div', { class: 'brand' }, h('img', { src: 'icon.png', alt: '' }), h('div', {}, h('b', {}, 'Draft Studio'), h('small', {}, 'VectorCraft · EffectCraft · FilmCraft → CapCut'))),
      h('input', { id: 'pname', class: 'pname', 'aria-label': 'Tên dự án', oninput: e => { S.spec.project_name = e.target.value; changed({ keep: true }); } }),
      h('span', { id: 'savestate', class: 'savestate' }),
      h('div', { class: 'grow' }),
      h('div', { id: 'agentchip', class: 'row' }),
      h('div', { class: 'crafts', id: 'craftchips' }),
      h('div', { id: 'topacts', class: 'row' })),
    h('nav', { class: 'rail', id: 'rail', 'aria-label': 'Bảng' }),
    h('aside', { class: 'panel', id: 'panel' }),
    h('main', { class: 'stage', id: 'stage' }),
    h('aside', { class: 'insp', id: 'insp' }),
    h('section', { class: 'timeline', id: 'timeline' }));
}

// ---------------------------------------------------------------- rail + panels
const TABS = [
  ['project', 'folder', 'Dự án'], ['media', 'image', 'Cảnh'], ['graphics', 'layers', 'Đồ họa'], ['motion', 'spark', 'Hiệu ứng'],
  ['audio', 'music', 'Âm thanh'], ['capcut', 'capcut', 'CapCut'], ['settings', 'gear', 'Cài đặt'],
];
function renderRail() {
  const counts = A.on ? {} : { media: allScenes().length, graphics: S.spec.overlays.length, audio: S.spec.sections.length + S.spec.voiceovers.length + S.spec.bgm.length };
  put($('#rail'), tabBtn('agent', 'agent', 'Agent', A.calls.length || null), ...TABS.map(([id, ic, label]) => {
    if (id === 'settings') return [h('div', { class: 'sp' }), tabBtn(id, ic, label)];
    return tabBtn(id, ic, label, counts[id]);
  }).flat());
  function tabBtn(id, ic, label, n) {
    const live = id === 'agent' && A.calls.some(c => callState(c) === 'running');
    return h('button', { class: (S.tab === id ? 'on' : '') + (live ? ' live' : ''), title: id === 'agent' ? 'Agent đang làm gì (MCP)' : label, onclick: () => {
      if (id === 'agent') return enterAgent(A.sel);
      if (A.on) return exitAgent(id);
      S.tab = id; if (id === 'capcut') loadDrafts(); render();
    } }, icon(ic), label, n ? h('span', { class: 'badge' }, n) : null);
  }
}
function phead(title, sub, ...acts) { return h('div', { class: 'phead' }, h('div', {}, h('h2', {}, title), sub ? h('p', {}, sub) : null), h('div', { class: 'row' }, ...acts)); }
function renderPanel() {
  if (A.on) return renderAgentPanel();
  const el = $('#panel'), f = { project: pProject, media: pMedia, graphics: pGraphics, motion: pMotion, audio: pAudio, capcut: pCapcut, settings: pSettings }[S.tab];
  const scroll = el.querySelector('.pbody') ? el.querySelector('.pbody').scrollTop : 0;
  put(el, ...f());
  const b = el.querySelector('.pbody'); if (b) b.scrollTop = scroll;
}

function pProject() {
  const sp = S.spec, canvas = `${sp.width}x${sp.height}`;
  return [phead('Dự án', 'Khung hình, cách dựng, dự án đã lưu'),
    h('div', { class: 'pbody' },
      h('div', { class: 'section' },
        h('label', { class: 'f' }, h('span', {}, 'Khung hình'), h('select', { onchange: e => { const [w, hh] = e.target.value.split('x').map(Number); sp.width = w; sp.height = hh; S.gfx = {}; changed({ gfx: true }); } },
          ...CANVAS.map(([v, l]) => h('option', { value: v, selected: v === canvas }, l + ' — ' + v.replace('x', '×'))),
          CANVAS.some(c => c[0] === canvas) ? null : h('option', { value: canvas, selected: true }, canvas))),
        h('label', { class: 'f' }, h('span', {}, 'Khung hình / giây'), h('select', { onchange: e => { sp.fps = +e.target.value; changed(); } },
          ...[24, 25, 30, 50, 60].map(f => h('option', { value: f, selected: +sp.fps === f }, f + ' fps')))),
        h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: sp.fit_scenes_to_voice, onchange: e => { sp.fit_scenes_to_voice = e.target.checked; changed(); } }), S.spec.sections.length ? 'Khớp ảnh tự do với giọng đọc tự do (cảnh trong đoạn luôn tự khớp)' : 'Khớp ảnh với độ dài giọng đọc'),
        h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: sp.register_to_capcut, onchange: e => { sp.register_to_capcut = e.target.checked; changed(); } }), 'Đưa lên đầu danh sách CapCut'),
        h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: sp.film_preview, onchange: e => { sp.film_preview = e.target.checked; changed(); } }), 'Kèm bản xem trước MP4 (FilmCraft)')),
      h('div', { class: 'section' },
        h('div', { class: 'between' }, h('span', { class: 'eyebrow' }, 'Dự án đã lưu'),
          h('div', { class: 'row' },
            h('button', { class: 'btn sm', onclick: importSpec, title: 'Mở spec JSON (v1 hoặc v2)' }, icon('open', 'sm'), 'Nhập JSON'),
            h('button', { class: 'btn sm', onclick: newProject }, icon('plus', 'sm'), 'Mới'))),
        S.specs.length ? h('div', { class: 'col' }, ...S.specs.map(p => h('button', { class: 'pcard' + (p.slug === S.slug ? ' cur' : ''), onclick: () => openSpec(p.slug) },
          icon('film'), h('div', { class: 'grow' }, h('div', { class: 'ellipsis', style: { fontWeight: 600 } }, p.name || p.slug), h('div', { class: 'small muted' }, `${p.scenes} cảnh · ${p.overlays} overlay · ${new Date(p.modified * 1000).toLocaleString('vi-VN', { dateStyle: 'short', timeStyle: 'short' })}`)),
          p.slug === S.slug ? h('span', { class: 'tag capcut' }, 'đang mở') : h('span', { class: 'btn icon sm ghost danger', title: 'Xoá dự án đã lưu', onclick: async e => { e.stopPropagation(); if (!confirm('Xoá dự án đã lưu “' + (p.name || p.slug) + '”? (draft trong CapCut không bị ảnh hưởng)')) return; await api.del('/api/specs/' + p.slug); await refreshSpecs(); render(); } }, icon('trash', 'sm')))))
          : h('div', { class: 'small muted' }, 'Chưa có dự án nào khác.'),
        h('button', { class: 'btn sm ghost', onclick: () => { const b = new Blob([JSON.stringify(S.spec, null, 2)], { type: 'application/json' }); const a = h('a', { href: URL.createObjectURL(b), download: slugify(S.spec.project_name) + '.spec.json' }); a.click(); } }, icon('save', 'sm'), 'Tải spec JSON (dùng cho CLI / agent)')))];
}
async function newProject() {
  await save(); S.spec = newSpec(); S.slug = null; S.sel = null; S.t = 0; stop(); S.tab = 'media';
  await save(); await refreshSpecs(); render();
}
async function importSpec() {
  const [p] = await pick('json'); if (!p) return;
  try {
    const sp = await api.post('/api/import', { path: p });
    await save();
    S.spec = Object.assign(newSpec(), sp); S.spec.studio = Object.assign({ accent: '#E8B04B', font: 'Segoe UI' }, sp.studio || {});
    S.slug = null; S.sel = null; S.t = 0; stop(); S.tab = 'media';
    await save(); await refreshSpecs(); probeAll(); ensureGraphics(); render();
    toast(`Đã nhập “${S.spec.project_name}”: ${S.spec.sections.length ? S.spec.sections.length + ' đoạn, ' : ''}${allScenes().length} cảnh, ${S.spec.overlays.length} overlay.`, 'ok');
  } catch (e) { toast('Không nhập được spec: ' + e.message, 'err'); }
}

function sceneThumb(path, cls = 'thumb') {
  const k = kindOf(path);
  return h('div', { class: cls }, k === 'video' ? h('video', { src: fileUrl(path) + '#t=0.5', muted: true, preload: 'metadata' }) : h('img', { src: fileUrl(path), loading: 'lazy', alt: '' }),
    h('span', { class: 'kind' + (k === 'video' ? '' : ' img') }, k === 'video' ? 'VIDEO' : 'ẢNH'));
}
function sceneItem(c) {
  const s = c.s, inf = info(s.path), list = sceneList(c.sec);
  return h('div', { class: 'mitem' + (isSel('scene', c.k, c.sec) ? ' sel' : ''), onclick: () => { select('scene', c.k, c.sec); seek(c.start + 0.01); } },
    h('span', { class: 'n' }, c.n + 1), sceneThumb(s.path),
    h('div', { class: 'grow' }, h('div', { class: 't ellipsis' }, s.desc || base(s.path)),
      h('div', { class: 's' }, fmtD(c.dur), c.auto ? (c.sec != null ? ' · khớp giọng' : ' · tự khớp') : '', s.transition ? ' · ⚡ ' + s.transition : '', s.video_effect ? ' · ✨ ' + s.video_effect : '', s.lut ? ' · 🎨 LUT' : '', s.ken_burns ? (s.pan && s.pan !== 'none' ? ' · Zoom + lia' : ' · Zoom') : '', inf && !inf.exists ? h('span', { style: { color: 'var(--err)' } }, ' · thiếu file') : '')),
    h('div', { class: 'acts' },
      h('button', { class: 'btn icon sm ghost', title: 'Lên', disabled: c.k === 0, onclick: e => { e.stopPropagation(); move(list, c.k, -1, 'scene', c.sec); } }, icon('up', 'sm')),
      h('button', { class: 'btn icon sm ghost', title: 'Xuống', disabled: c.k === list.length - 1, onclick: e => { e.stopPropagation(); move(list, c.k, 1, 'scene', c.sec); } }, icon('down', 'sm'))));
}
function sectionBlock(tl, x, si) {
  const v = tl.voice.find(c => c.sec === si) || { dur: 0 }, errs = tl.errors.filter(e => e.sec === si);
  return h('div', { class: 'section' },
    h('div', { class: 'between secrow' + (isSel('section', si) ? ' sel' : ''), title: 'Giọng đọc và khoảng lặng của đoạn', onclick: () => { select('section', si); seek(tl.sectionStarts[si] + 0.01); } },
      h('span', { class: 'eyebrow ellipsis' }, icon('voice', 'sm'), ' ', secLabel(x, si)),
      h('div', { class: 'row' }, h('span', { class: 'small muted' }, fmtD(v.dur) + (x.gap_after_sec > 0 ? ' + ' + fmtD(x.gap_after_sec) + ' lặng' : '')),
        h('button', { class: 'btn icon sm ghost', title: 'Thêm cảnh vào đoạn này', onclick: e => { e.stopPropagation(); addScenes(false, si); } }, icon('plus', 'sm')))),
    ...errs.map(e => h('div', { class: 'note err' }, e.msg)),
    x.scenes.length ? h('div', { class: 'mlist' }, ...tl.scenes.filter(c => c.sec === si).map(sceneItem)) : h('div', { class: 'small muted' }, 'Chưa có cảnh — bấm + để thêm; độ dài tự khớp theo giọng.'));
}
function pMedia() {
  const tl = compute(), sp = S.spec, n = allScenes().length, free = tl.scenes.filter(c => c.sec == null);
  const imgs = () => allScenes().filter(s => kindOf(s.path) === 'image');
  return [phead('Cảnh', `Track hình chính · ${sp.sections.length ? sp.sections.length + ' đoạn · ' : ''}${n} cảnh · ${fmtD(tl.mainEnd)}`,
    h('button', { class: 'btn sm', onclick: () => addScenes(true), title: 'Thêm cả thư mục ảnh' }, icon('folder', 'sm')),
    h('button', { class: 'btn sm primary', onclick: () => addScenes(false) }, icon('plus', 'sm'), 'Thêm')),
    h('div', { class: 'pbody' },
      ...sp.sections.map((x, si) => sectionBlock(tl, x, si)),
      sp.sections.length && free.length ? h('span', { class: 'eyebrow' }, 'Cảnh tự do · sau các đoạn') : null,
      free.length ? h('div', { class: 'mlist' }, ...free.map(sceneItem))
        : n ? null : h('div', { class: 'empty' }, icon('image'), 'Chưa có cảnh. Thêm ảnh hoặc video — chúng nối tiếp nhau trên track chính. Có giọng đọc thì thêm đoạn: cảnh trong đoạn tự khớp độ dài theo giọng.',
          h('div', { class: 'row' }, h('button', { class: 'btn sm primary', onclick: () => addScenes(false) }, icon('plus', 'sm'), 'Chọn file'), h('button', { class: 'btn sm', onclick: () => addScenes(true) }, icon('folder', 'sm'), 'Cả thư mục'))),
      h('button', { class: 'btn sm dashed block', onclick: addSections, title: 'Mỗi file giọng thành một đoạn; cảnh trong đoạn tự chia độ dài theo giọng, ranh giới đoạn trùng đúng điểm bắt đầu giọng' }, icon('voice', 'sm'), 'Thêm đoạn theo giọng đọc'),
      n ? h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, 'Áp dụng cho mọi ảnh'),
        h('div', { class: 'row wrap' },
          h('button', { class: 'btn sm', onclick: () => { imgs().forEach(s => { s.ken_burns = true; }); changed(); } }, 'Bật Ken Burns'),
          h('button', { class: 'btn sm', onclick: () => { const pans = ['none', 'right', 'none', 'left']; allScenes().forEach((s, i) => { if (kindOf(s.path) === 'image') { s.ken_burns = true; s.pan = pans[i % 4]; } }); changed(); } }, 'Zoom + lia xen kẽ'),
          h('button', { class: 'btn sm ghost', onclick: () => { allScenes().forEach(s => { delete s.duration_sec; }); changed(); } }, 'Tự khớp thời lượng'))) : null)];
}
function move(arr, i, d, type, sec = null) { const j = i + d; if (j < 0 || j >= arr.length) return; [arr[i], arr[j]] = [arr[j], arr[i]]; if (isSel(type, i, sec)) S.sel.i = j; changed(); }

function pGraphics() {
  const ready = craft('vectorcraft').ready, st = S.spec.studio;
  return [phead('Đồ họa', 'VectorCraft vẽ → PNG trong suốt trên track overlay', h('span', { class: 'tag vector' }, 'VectorCraft')),
    h('div', { class: 'pbody' },
      ready ? null : h('div', { class: 'note warn' }, h('b', {}, 'VectorCraft chưa sẵn sàng. '), 'Vẫn thêm được overlay; ảnh sẽ được vẽ khi VectorCraft build xong (xem Cài đặt).'),
      h('div', { class: 'grid2' },
        h('label', { class: 'f' }, h('span', {}, 'Màu nhấn mặc định'), h('div', { class: 'row' }, h('input', { type: 'color', value: st.accent, onchange: e => { st.accent = e.target.value; changed({ gfx: true }); } }), h('span', { class: 'mono small muted' }, st.accent))),
        h('label', { class: 'f' }, h('span', {}, 'Font'), h('select', { onchange: e => { st.font = e.target.value; changed({ gfx: true }); } },
          ...['Segoe UI', 'Arial', 'Georgia', 'Times New Roman', 'Cambria', 'Bahnschrift', 'Trebuchet MS'].map(f => h('option', { selected: st.font === f }, f))))),
      h('div', { class: 'gal' }, ...S.catalog.graphics.map(t => {
        const fields = {}; t.fields.forEach(f => fields[f.key] = f.default);
        const g = gfxOf({ graphic: { template: t.id, fields } });
        if (!g && ready) renderGraphic({ template: t.id, fields });
        return h('button', { class: 'gcard', onclick: () => addGraphic(t), title: t.description + ' — bấm để thêm tại playhead' },
          h('div', { class: 'pv checker' }, g && g.png ? h('img', { src: fileUrl(g.png), alt: '' }) : h('div', { class: 'ph' }, g && g.pending ? h('span', { class: 'spinner' }) : (ready ? '…' : t.name))),
          h('div', { class: 'meta' }, h('b', {}, t.name), h('span', {}, t.description)));
      })),
      h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, 'Overlay từ file'),
        h('button', { class: 'btn sm dashed block', onclick: addFileOverlay }, icon('plus', 'sm'), 'Ảnh / video PNG, MOV alpha… làm overlay')))];
}

function pMotion() {
  const sel = S.sel && S.sel.type === 'overlay' ? S.spec.overlays[S.sel.i] : null;
  const ready = craft('effectcraft').ready;
  const lfx = S.catalog.layer_effects || [];
  return [phead('Chuyển động & Layer FX', `EffectCraft · ${S.catalog.motion.length} Preset + ${lfx.length} Layer FX`, h('span', { class: 'tag effect' }, 'EffectCraft')),
    h('div', { class: 'pbody' },
      ready ? null : h('div', { class: 'note warn' }, h('b', {}, 'EffectCraft chưa sẵn sàng. '), 'Chọn preset vẫn được; video được render khi xuất (cần EffectCraft).'),
      sel ? h('div', { class: 'note' }, 'Áp dụng cho ', h('b', {}, overlayName(sel)), '. Chọn “Không” để giữ ảnh tĩnh với fade của CapCut.')
        : h('div', { class: 'note' }, 'Chọn một overlay trên timeline để gán chuyển động, hoặc tạo chuyển động từ một ảnh PNG:'),
      h('div', { class: 'presets' },
        sel ? h('button', { class: 'preset' + (sel.kind !== 'motion' ? ' on' : ''), onclick: () => { sel.kind = sel.graphic ? 'graphic' : 'file'; delete sel.motion; changed(); } },
          h('div', { class: 'ic' }, h('i', { style: { opacity: .5 } })), h('div', {}, h('b', { class: 'small' }, 'Không'), h('div', { class: 'small muted' }, 'Ảnh tĩnh + fade CapCut')), '') : null,
        ...S.catalog.motion.map(p => h('button', {
          class: 'preset' + (sel && sel.kind === 'motion' && sel.motion && sel.motion.preset === p.id ? ' on' : ''), style: { '--anim': 'p-' + p.id },
          onclick: () => { if (sel) { sel.kind = 'motion'; sel.motion = Object.assign({ format: 'prores' }, sel.motion || {}, { preset: p.id }); changed(); } else motionFromImage(p.id); },
        }, h('div', { class: 'ic' }, h('i')), h('div', {}, h('b', { class: 'small' }, p.name), h('div', { class: 'small muted' }, `[${p.category || 'Motion'}] ${p.description}`)), sel ? '' : h('span', { class: 'small muted' }, 'ảnh…')))),
      lfx.length ? h('details', { class: 'section' },
        h('summary', { class: 'eyebrow', style: { cursor: 'pointer' } }, `⚡ Mở rộng: ${lfx.length} Kỹ xảo Layer After Effects (ec.*)`),
        h('div', { class: 'presets', style: { marginTop: '8px' } },
          ...lfx.map(p => h('button', {
            class: 'preset' + (sel && sel.kind === 'motion' && sel.motion && sel.motion.preset === p.id ? ' on' : ''),
            onclick: () => { if (sel) { sel.kind = 'motion'; sel.motion = Object.assign({ format: 'prores' }, sel.motion || {}, { preset: p.id }); changed(); } else motionFromImage(p.id); },
          }, h('div', { class: 'ic' }, h('i')), h('div', {}, h('b', { class: 'small' }, p.name), h('div', { class: 'small muted' }, p.description)), sel ? '' : h('span', { class: 'small muted' }, 'ảnh…'))))) : null)];
}

function pAudio() {
  const tl = compute(), sp = S.spec;
  const row = (type, a, i, dur, tag) => h('div', { class: 'mitem' + (isSel(type, i) ? ' sel' : ''), style: { gridTemplateColumns: '22px 1fr auto' }, onclick: () => select(type, i) },
    h('span', { class: 'n' }, i + 1), h('div', { class: 'grow' }, h('div', { class: 't ellipsis' }, a.desc || base(a.path)), h('div', { class: 's' }, fmtD(dur), ' · ', tag)), h('div', { class: 'acts' }));
  return [phead('Âm thanh', `Giọng đọc ${fmtD(tl.voiceEnd)} · video ${fmtD(tl.total)}`),
    h('div', { class: 'pbody' },
      sp.sections.length ? h('div', { class: 'section' },
        h('div', { class: 'between' }, h('span', { class: 'eyebrow' }, 'Giọng của các đoạn'), h('button', { class: 'btn sm', onclick: addSections }, icon('plus', 'sm'), 'Thêm đoạn')),
        h('div', { class: 'mlist' }, ...sp.sections.map((x, i) => row('section', x.voice, i, (tl.voice.find(v => v.sec === i) || {}).dur || 0,
          `${x.scenes.length} cảnh khớp theo${x.gap_after_sec > 0 ? ` · lặng ${fmtD(x.gap_after_sec)}` : ''}`)))) : null,
      h('div', { class: 'section' },
        h('div', { class: 'between' }, h('span', { class: 'eyebrow' }, 'Giọng đọc'), h('button', { class: 'btn sm', onclick: addVoice }, icon('plus', 'sm'), 'Thêm')),
        sp.voiceovers.length ? h('div', { class: 'mlist' }, ...sp.voiceovers.map((a, i) => row('voice', a, i, (tl.voice.find(v => v.sec == null && v.i === i) || {}).dur || 0, `âm lượng ${Math.round(a.volume * 100)}%`)))
          : h('div', { class: 'small muted' }, sp.sections.length ? 'Giọng không thuộc đoạn nào: nối tiếp sau các đoạn, hoặc theo giờ bắt đầu riêng.' : 'Các file thoại nối tiếp nhau trên một track. Bật “Khớp ảnh với giọng đọc” để ảnh dài bằng lời — hoặc dùng “Thêm đoạn” ở tab Cảnh để mỗi giọng có nhóm cảnh riêng tự khớp.')),
      h('div', { class: 'section' },
        h('div', { class: 'between' }, h('span', { class: 'eyebrow' }, 'Nhạc nền'), h('button', { class: 'btn sm', onclick: addBgm }, icon('plus', 'sm'), 'Thêm')),
        sp.bgm.length ? h('div', { class: 'mlist' }, ...sp.bgm.map((b, i) => row('bgm', b, i, (info(b.path) || {}).duration || 0, `${Math.round(b.volume * 100)}%${b.duck_under_voice ? ' · tự hạ dưới giọng' : ''}${b.loop_to_fill ? ' · lặp' : ''}`)))
          : h('div', { class: 'small muted' }, 'Nhạc tự lặp tới hết video, fade đầu/cuối và tự hạ âm lượng khi có giọng đọc (keyframe âm lượng của CapCut).')))];
}

async function loadDrafts() { try { S.drafts = await api.get('/api/drafts'); } catch (e) { S.drafts = []; toast(e.message, 'err'); } scheduleRender(); }
function pCapcut() {
  return [phead('Draft trong CapCut', S.status ? base(S.status.capcut_root) : '', h('button', { class: 'btn icon sm', title: 'Tải lại', onclick: loadDrafts }, icon('refresh', 'sm')),
    h('button', { class: 'btn sm', onclick: () => api.post('/api/open', { what: 'capcut' }).catch(e => toast(e.message, 'err')) }, icon('open', 'sm'), 'Mở CapCut')),
    h('div', { class: 'pbody' },
      S.status && S.status.capcut_running ? h('div', { class: 'note warn' }, h('b', {}, 'CapCut đang mở. '), 'CapCut ghi đè danh sách dự án khi thoát — sau khi xuất, tắt hẳn CapCut rồi mở lại.') : null,
      S.drafts == null ? h('div', { class: 'row muted small' }, h('span', { class: 'spinner' }), 'Đang đọc…')
        : h('div', { class: 'dlist' }, ...S.drafts.map(d => h('button', { class: 'dcard', onclick: () => showDraft(d.name) },
          h('div', { class: 'cv', style: d.cover ? { backgroundImage: `url("${fileUrl(d.cover)}")` } : {} }),
          h('div', { class: 'grow', style: { minWidth: 0 } }, h('b', { class: 'ellipsis' }, d.name),
            h('div', { class: 's' }, h('span', {}, fmtD(d.duration_sec)), h('span', {}, `${d.video_clips} cảnh`), d.overlay_tracks ? h('span', {}, `${d.overlay_tracks} overlay`) : null, h('span', {}, `${d.audio_clips} audio`)),
            h('div', { class: 's' }, h('span', {}, d.modified), d.opened_in_capcut ? h('span', { class: 'tag capcut' }, 'đã mở') : h('span', { class: 'tag file' }, 'mới'))))))) ];
}
async function showDraft(name) {
  let d; try { d = await api.get('/api/draft?name=' + encodeURIComponent(name)); } catch (e) { return toast(e.message, 'err'); }
  modal(d.summary.name, h('div', { class: 'col', style: { gap: '14px' } },
    h('div', { class: 'stat', style: { gridTemplateColumns: 'repeat(4, 1fr)' } },
      h('div', {}, h('b', {}, fmtD(d.summary.duration_sec)), h('span', {}, 'thời lượng')), h('div', {}, h('b', {}, `${d.summary.width}×${d.summary.height}`), h('span', {}, `${d.fps} fps`)),
      h('div', {}, h('b', {}, d.tracks.length), h('span', {}, 'track')), h('div', {}, h('b', { style: { color: d.missing_media.length ? 'var(--err)' : '' } }, d.missing_media.length), h('span', {}, 'file thiếu'))),
    ...d.tracks.map(t => h('div', { class: 'col', style: { gap: '4px' } }, h('span', { class: 'eyebrow' }, (t.overlay ? 'overlay ' : '') + t.kind + ` · ${t.segments.length} đoạn`),
      h('table', { class: 'dtable' }, ...t.segments.slice(0, 40).map(s => h('tr', { class: s.missing ? 'miss' : '' }, h('td', {}, fmtT(s.start_sec)), h('td', {}, fmtD(s.dur_sec)), h('td', { class: 'ellipsis', title: s.path }, s.name || base(s.path), s.missing ? ' — thiếu file' : '')))))),
    h('div', { class: 'small muted mono' }, d.content_file)),
    [h('button', { class: 'btn', onclick: () => api.post('/api/open', { what: 'folder', path: d.summary.path }) }, icon('folder', 'sm'), 'Mở thư mục'),
     h('button', { class: 'btn', onclick: async () => { try { const r = await api.post('/api/repair', { name: d.summary.name }); toast(r.message, 'ok'); } catch (e) { toast(e.message, 'err'); } } }, icon('wrench', 'sm'), 'Sửa đường dẫn \\\\?\\')], true);
}

function pSettings() {
  const st = S.status || {}, se = S.settings || {};
  const mcpExe = (st.exe_dir || 'C:/path/to/app') + '/capcut_draft_app.exe';
  const cfg = JSON.stringify({ mcpServers: { 'draft-studio': { command: mcpExe.replace(/\//g, '\\'), args: ['mcp'] } } }, null, 2);
  const field = (k, label) => h('label', { class: 'f' }, h('span', {}, label), h('input', { value: se[k] || '', placeholder: 'tự dò', onchange: e => { se[k] = e.target.value.trim() || null; saveSettings(); } }));
  return [phead('Cài đặt', 'Engine craft, nơi lưu tài nguyên, kết nối agent', h('button', { class: 'btn icon sm', title: 'Kiểm tra lại', onclick: loadStatus }, icon('refresh', 'sm'))),
    h('div', { class: 'pbody' },
      licenseSection(),
      h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, 'Ba engine craft'),
        ...(st.crafts || []).map(c => h('div', { class: 'craftrow' }, h('span', { class: 'dot ' + (c.ready ? 'on' : 'off'), style: { marginTop: '5px' } }),
          h('div', { class: 'nm' }, c.name, h('span', { class: 'tag ' + c.id.replace('craft', '') }, c.ready ? 'sẵn sàng' : 'chưa có')), h('div', { class: 'rl' }, c.role),
          h('div', { class: 'pth' }, c.ready ? (c.version ? c.version + ' · ' : '') + c.path : 'Chưa tìm thấy ' + c.id + '-cli.exe — đặt vào thư mục crafts/ cạnh app, vào PATH, hoặc chỉ đường dẫn bên dưới (mã nguồn: github.com/storytold/' + c.id + ')')))),
      h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, 'Đường dẫn tuỳ chỉnh'),
        field('vectorcraft_cli', 'vectorcraft-cli.exe'), field('effectcraft_cli', 'effectcraft-cli.exe'), field('filmcraft_cli', 'filmcraft-cli.exe'),
        field('craft_engines_dir', 'Thư mục craft_engines (nơi clone và build 3 repo craft)'),
        field('assets_dir', 'Thư mục tài nguyên sinh ra (CapCut trỏ tới các file này — đừng xoá)'),
        h('div', { class: 'small muted' }, 'Hiện tại: ', h('span', { class: 'mono' }, st.assets_dir || ''))),
      h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, 'Môi trường'),
        h('div', { class: 'row small' }, h('span', { class: 'dot ' + (st.ffprobe ? 'on' : 'off') }), 'ffprobe', h('span', { class: 'dot ' + (st.ffmpeg ? 'on' : 'off'), style: { marginLeft: '10px' } }), 'ffmpeg (ảnh bìa từ video)'),
        h('div', { class: 'small muted mono', style: { wordBreak: 'break-all' } }, st.capcut_root || '')),
      h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, 'Xem agent làm việc trực tiếp (Live IDE Mode)'),
        h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: !!se.open_studio_on_agent_build, onchange: e => { se.open_studio_on_agent_build = e.target.checked; saveSettings(); } }), 'Tự mở Draft Studio khi agent bắt đầu dựng'),
        h('div', { class: 'small muted' }, 'Khi agent gọi capcut_live_edit, capcut_build_project, capcut_create_draft hoặc CLI: studio tự mở và đưa cửa sổ lên trước.'),
        h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: A.follow, onchange: e => setFollow(e.target.checked) }), 'Tự động hiển thị trực tiếp từng bước trên màn hình'),
        h('div', { class: 'small muted' }, 'Mọi cảnh, hiệu ứng chuyển cảnh, LUT màu, đồ họa VectorCraft và chuyển động EffectCraft đều hiện trực tiếp trên Timeline & Stage ngay khi agent thực hiện.')),
      mcpSection(cfg))];
}
async function loadMcp() {
  try { S.mcp = await api.get('/api/mcp-clients'); } catch {}
  scheduleRender();
}
async function toggleMcp(client, action) {
  try {
    S.mcp = await api.post('/api/mcp-clients', { client, action });
    toast(action === 'disconnect' ? `Đã ngắt kết nối MCP (${client}).` : `✔ Đã kết nối MCP thành công (${client === 'all' ? 'Tất cả Agent' : client})!`, 'ok');
    scheduleRender();
    if ($('#mcp-modal-body')) put($('#mcp-modal-body'), mcpModalContent());
  } catch (e) { toast('Lỗi kết nối MCP: ' + e.message, 'err'); }
}
function mcpChip() {
  const cs = (S.mcp && S.mcp.clients) || [];
  const conn = cs.filter(c => c.connected);
  return h('button', { class: 'chip mcp-btn' + (conn.length ? ' on' : ''), title: conn.length ? `Đã kết nối MCP: ${conn.map(c => c.name).join(', ')}\nBấm để quản lý kết nối MCP` : 'Bấm để kết nối MCP 1-Click với Antigravity, Claude Desktop, Cursor…', onclick: openMcpModal },
    h('span', { class: 'dot ' + (conn.length ? 'on' : 'run') }), conn.length ? `🔌 MCP · Đã nối (${conn.length})` : '🔌 Kết nối MCP');
}
function mcpModalContent() {
  const cs = (S.mcp && S.mcp.clients) || [];
  const exe = (S.mcp && S.mcp.mcp_exe) || ((S.status && S.status.exe_dir || 'C:/path/to/app') + '/capcut_draft_app.exe').replace(/\//g, '\\');
  const cfg = JSON.stringify({ mcpServers: { 'draft-studio': { command: exe, args: ['mcp'] } } }, null, 2);
  return h('div', { class: 'col', style: { gap: '12px' } },
    h('div', { class: 'note' }, 'Kết nối 1-Click giúp ', h('b', {}, 'Antigravity (Gemini Agent), Claude Desktop, Cursor, Windsurf'), ' điều khiển Draft Studio trực tiếp. Khi Agent chạy lệnh, từng bước dựng và kết quả sẽ hiển thị sống động ngay trên màn hình như khi sửa code trong IDE!'),
    h('button', { class: 'btn primary block', style: { height: '38px' }, onclick: () => toggleMcp('all', 'connect') }, icon('spark', 'sm'), '⚡ Kết nối tất cả Agent (1-Click)'),
    h('div', { class: 'col', style: { gap: '8px' } }, ...cs.map(c => h('div', { class: 'mcp-card' + (c.connected ? ' on' : '') },
      h('span', { class: 'dot ' + (c.connected ? 'on' : 'off') }),
      h('div', { style: { minWidth: 0 } },
        h('div', { class: 'row', style: { gap: '6px' } }, h('b', {}, c.name), c.connected ? h('span', { class: 'tag capcut' }, 'Đã kết nối') : h('span', { class: 'tag file' }, 'Chưa kết nối')),
        h('div', { class: 'small muted mono ellipsis', title: c.path }, c.path)),
      c.connected
        ? h('div', { class: 'row', style: { gap: '6px' } },
            h('button', { class: 'btn sm', onclick: () => toggleMcp(c.id, 'connect') }, 'Cập nhật'),
            h('button', { class: 'btn sm ghost danger', onclick: () => toggleMcp(c.id, 'disconnect') }, 'Ngắt'))
        : h('button', { class: 'btn sm primary', onclick: () => toggleMcp(c.id, 'connect') }, 'Kết nối ngay')))),
    h('div', { class: 'section', style: { marginTop: '4px' } },
      h('div', { class: 'between' }, h('span', { class: 'eyebrow' }, 'Cấu hình JSON thủ công'),
        h('button', { class: 'btn sm', onclick: () => navigator.clipboard.writeText(cfg).then(() => toast('Đã chép cấu hình MCP.', 'ok')) }, icon('copy', 'sm'), 'Chép JSON')),
      h('pre', { class: 'code' }, cfg)));
}
function openMcpModal() {
  loadMcp();
  modal('🔌 Kết nối Agent qua MCP (1-Click)', h('div', { id: 'mcp-modal-body' }, mcpModalContent()), [], true);
}
function mcpSection(cfg) {
  const cs = (S.mcp && S.mcp.clients) || [];
  const conn = cs.filter(c => c.connected);
  return h('div', { class: 'section' },
    h('div', { class: 'between' }, h('span', { class: 'eyebrow' }, 'Kết nối Agent (MCP 1-Click)'),
      h('button', { class: 'btn sm primary', onclick: () => toggleMcp('all', 'connect') }, icon('spark', 'sm'), 'Kết nối tất cả')),
    h('div', { class: 'col', style: { gap: '6px' } }, ...cs.map(c => h('div', { class: 'mcp-card' + (c.connected ? ' on' : '') },
      h('span', { class: 'dot ' + (c.connected ? 'on' : 'off') }),
      h('div', { style: { minWidth: 0 } }, h('b', { class: 'small' }, c.name), h('div', { class: 'small muted mono ellipsis' }, c.path)),
      h('button', { class: 'btn sm ' + (c.connected ? 'ghost danger' : 'primary'), onclick: () => toggleMcp(c.id, c.connected ? 'disconnect' : 'connect') }, c.connected ? 'Ngắt' : 'Kết nối')))),
    cfg ? h('details', {}, h('summary', { class: 'small muted' }, 'Xem JSON cấu hình MCP'), h('pre', { class: 'code', style: { marginTop: '6px' } }, cfg),
      h('button', { class: 'btn sm', style: { marginTop: '6px' }, onclick: () => navigator.clipboard.writeText(cfg).then(() => toast('Đã chép cấu hình MCP.', 'ok')) }, icon('copy', 'sm'), 'Chép')) : null);
}
async function saveSettings() { try { S.settings = await api.put('/api/settings', S.settings); toast('Đã lưu cài đặt.', 'ok'); loadStatus(); } catch (e) { toast(e.message, 'err'); } }
async function loadStatus() {
  try { S.status = await api.get('/api/status'); } catch (e) { toast('Không đọc được trạng thái: ' + e.message, 'err'); }
  for (const k of Object.keys(S.gfx)) if (S.gfx[k].soft) delete S.gfx[k];
  scheduleRender(); ensureGraphics();
}

// ---------------------------------------------------------------- selection + inspector
function select(type, i, sec = null) { S.sel = { type, i, sec }; render(); }
const isSel = (type, i, sec = null) => !!S.sel && S.sel.type === type && S.sel.i === i && (S.sel.sec ?? null) === sec;
function overlayName(o) {
  if (o.graphic && tpl(o.graphic.template)) { const t = tpl(o.graphic.template), f = o.graphic.fields || {}; return f[t.fields[0].key] || t.name; }
  return o.desc || base(o.path) || 'Overlay';
}
function sec(label, ...kids) { return h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, label), ...kids); }
function numField(label, obj, key, opts = {}) {
  const v = obj[key];
  return h('label', { class: 'f' }, h('span', {}, label, opts.unit ? h('b', {}, opts.unit) : null),
    h('input', { type: 'number', step: opts.step || 0.1, min: opts.min, max: opts.max, value: v == null ? '' : +(+v).toFixed(3), placeholder: opts.placeholder || '',
      oninput: e => { const n = parseFloat(e.target.value); if (e.target.value === '' && opts.optional) delete obj[key]; else if (Number.isFinite(n)) obj[key] = n; changed({ keep: true, gfx: opts.gfx }); } }));
}
const SLIDER_DEF = { zoom_start: 1, zoom_end: 1.06, x: 0, y: 0, scale: 1, opacity: 1, volume: 1, duck_ratio: 0.4 };
function slider(label, obj, key, min, max, step, show) {
  const v = obj[key] ?? SLIDER_DEF[key] ?? min;
  const out = h('b', {}, show(v));
  return h('label', { class: 'f' }, h('span', {}, label, out),
    h('input', { type: 'range', min, max, step, value: v, oninput: e => { obj[key] = +e.target.value; out.textContent = show(obj[key]); changed({ keep: true }); } }));
}
function renderInsp() {
  if (A.on) return renderAgentInsp();
  const el = $('#insp'), s = S.sel;
  const list = !s ? [] : s.type === 'scene' ? (s.sec == null || S.spec.sections[s.sec] ? sceneList(s.sec) : [])
    : { section: S.spec.sections, overlay: S.spec.overlays, voice: S.spec.voiceovers, bgm: S.spec.bgm }[s.type] || [];
  const it = list[s ? s.i : -1];
  if (!it) { S.sel = null; put(el, ...iOverview()); return; }
  put(el, ...({ scene: iScene, section: iSection, overlay: iOverlay, voice: iVoice, bgm: iBgm }[s.type])(it, s.i, s.sec));
}
function ihead(color, title, sub) {
  return h('div', { class: 'phead' }, h('div', { class: 'ititle' }, h('span', { class: 'sw', style: { background: color } }), h('div', {}, h('h2', {}, title), sub ? h('p', {}, sub) : null)),
    h('div', { class: 'row' }, h('button', { class: 'btn icon sm ghost', title: 'Bỏ chọn (Esc)', onclick: () => { S.sel = null; render(); } }, icon('x', 'sm'))));
}
function delBtn(label = 'Xoá') { return h('button', { class: 'btn sm danger', onclick: removeSel }, icon('trash', 'sm'), label); }

function iOverview() {
  const tl = compute(), sp = S.spec, warns = [];
  if (!allScenes().length && !sp.overlays.length) warns.push(['', 'Bắt đầu bằng tab Cảnh: thêm ảnh/video cho track chính, hoặc thêm đoạn theo từng file giọng đọc.']);
  for (const e of tl.errors) warns.push(['err', `${secLabel(sp.sections[e.sec], e.sec)}: ${e.msg}`]);
  if (tl.mainEnd && tl.voiceEnd > tl.mainEnd + 0.5) warns.push(['warn', `Hình (${fmtD(tl.mainEnd)}) ngắn hơn giọng đọc (${fmtD(tl.voiceEnd)}) — cuối video sẽ đen.`]);
  const missing = [...sp.sections.map(x => x.voice), ...allScenes(), ...sp.voiceovers, ...sp.bgm].filter(x => info(x.path) && !info(x.path).exists);
  if (missing.length) warns.push(['err', `${missing.length} file không còn tồn tại.`]);
  const needV = sp.overlays.some(o => o.kind !== 'file' && o.graphic), needE = sp.overlays.some(o => o.kind === 'motion');
  if (needV && !craft('vectorcraft').ready) warns.push(['warn', 'Có overlay đồ họa nhưng chưa có VectorCraft — xuất sẽ báo lỗi.']);
  if (needE && !craft('effectcraft').ready) warns.push(['warn', 'Có overlay chuyển động nhưng chưa có EffectCraft — xuất sẽ báo lỗi.']);
  if (sp.film_preview && !craft('filmcraft').ready) warns.push(['warn', 'Bản xem trước MP4 cần FilmCraft.']);
  return [h('div', { class: 'phead' }, h('div', {}, h('h2', {}, 'Tổng quan'), h('p', {}, `${sp.width}×${sp.height} · ${sp.fps} fps`))),
    h('div', { class: 'pbody' },
      h('div', { class: 'stat' },
        h('div', {}, h('b', {}, fmtD(tl.total)), h('span', {}, 'thời lượng')), h('div', {}, h('b', {}, allScenes().length), h('span', {}, sp.sections.length ? `cảnh · ${sp.sections.length} đoạn` : 'cảnh')),
        h('div', {}, h('b', {}, sp.overlays.length), h('span', {}, `overlay · ${tl.overlays.length ? tl.lanes : 0} track`)), h('div', {}, h('b', {}, sp.sections.length + sp.voiceovers.length + sp.bgm.length), h('span', {}, 'file âm thanh'))),
      ...warns.map(([k, m]) => h('div', { class: 'note ' + k }, m)),
      sec('Ai làm gì khi xuất',
        h('div', { class: 'col', style: { gap: '6px' } },
          h('div', { class: 'row small' }, h('span', { class: 'tag vector' }, 'VectorCraft'), h('span', { class: 'muted' }, `${sp.overlays.filter(o => o.graphic && o.kind !== 'file').length} đồ họa → PNG trong suốt`)),
          h('div', { class: 'row small' }, h('span', { class: 'tag effect' }, 'EffectCraft'), h('span', { class: 'muted' }, `${sp.overlays.filter(o => o.kind === 'motion').length} chuyển động → video alpha`)),
          h('div', { class: 'row small' }, h('span', { class: 'tag film' }, 'FilmCraft'), h('span', { class: 'muted' }, sp.film_preview ? 'đo media + MP4 xem trước' : 'đo thời lượng media')),
          h('div', { class: 'row small' }, h('span', { class: 'tag capcut' }, 'CapCut'), h('span', { class: 'muted' }, `${2 + (tl.overlays.length ? tl.lanes : 0)}+ track, Ken Burns, fade, ducking`)))),
      h('button', { class: 'btn primary block', style: { height: '40px' }, onclick: openBuild }, icon('export'), 'Xuất sang CapCut'),
      h('div', { class: 'small dim' }, 'Phím tắt: ', h('span', { class: 'kbd' }, 'Space'), ' phát · ', h('span', { class: 'kbd' }, '←'), h('span', { class: 'kbd' }, '→'), ' tua · ', h('span', { class: 'kbd' }, 'Del'), ' xoá · ', h('span', { class: 'kbd' }, 'Esc'), ' bỏ chọn'))];
}

function iScene(s, i, si) {
  const k = kindOf(s.path), c = compute().scenes.find(x => x.s === s) || { n: i }, inf = info(s.path) || {};
  return [ihead('var(--scene)', `Cảnh ${c.n + 1}`, `${k === 'video' ? 'Video' : 'Ảnh'} · ${si != null ? secLabel(S.spec.sections[si], si) + ' · ' : ''}${fmtT(c.start)} → ${fmtT(c.start + c.dur)}`),
    h('div', { class: 'pbody' },
      sceneThumb(s.path, 'preview-box checker'),
      h('div', { class: 'small mono muted', style: { wordBreak: 'break-all' } }, s.path, inf.width ? `  ·  ${inf.width}×${inf.height}` : '', inf.duration ? `  ·  ${fmtD(inf.duration)}` : ''),
      h('label', { class: 'f' }, h('span', {}, 'Mô tả'), h('input', { value: s.desc || '', placeholder: base(s.path), oninput: e => { s.desc = e.target.value || undefined; changed({ keep: true }); } })),
      sec('Thời lượng',
        h('div', { class: 'grid2' },
          numField('Giây (trống = tự)', s, 'duration_sec', { optional: true, placeholder: c.dur ? c.dur.toFixed(2) : '', min: 0.1 }),
          k === 'video' ? numField('Bỏ đầu video', s, 'trim_start_sec', { optional: true, min: 0, unit: 'giây' }) : h('div')),
        c.auto ? h('div', { class: 'small muted' }, si != null ? 'Tự khớp theo giọng của đoạn: ảnh giãn lấp chỗ trống, video chỉ bị cắt bớt đuôi theo tỉ lệ (không kéo giãn).' : S.spec.fit_scenes_to_voice ? 'Tự chia theo độ dài giọng đọc.' : 'Mặc định 5 giây.') : null),
      sec('Chuyển cảnh & Kỹ xảo (FilmCraft · 277 Hiệu ứng)',
        h('label', { class: 'f' }, h('span', {}, `Chuyển cảnh vào (${(S.catalog.transitions || []).length} Transitions)`), h('select', { onchange: e => { if (!e.target.value) delete s.transition; else s.transition = e.target.value; changed(); } },
          h('option', { value: '', selected: !s.transition }, '— Cắt thẳng (Không chuyển cảnh) —'),
          ...(S.catalog.transitions || []).map(tr => h('option', { value: tr.id, selected: s.transition === tr.id }, `[${tr.category}] ${tr.name}`)))),
        s.transition ? numField('Thời lượng chuyển cảnh', s, 'transition_sec', { optional: true, placeholder: '0.45', min: 0.15, max: 1.5, step: 0.05, unit: 'giây' }) : null,
        h('label', { class: 'f' }, h('span', {}, `Kỹ xảo cảnh (${(S.catalog.video_effects || []).length} Video FX & Presets)`), h('select', { onchange: e => { if (!e.target.value) delete s.video_effect; else s.video_effect = e.target.value; changed(); } },
          h('option', { value: '', selected: !s.video_effect }, '— Không —'),
          ...(S.catalog.video_effects || []).map(fx => h('option', { value: fx.id, selected: s.video_effect === fx.id }, `[${fx.category}] ${fx.name}`)))),
        h('label', { class: 'f' }, h('span', {}, `Màu phim điện ảnh (${(S.catalog.luts || []).length} LUTs & Lumetri)`), h('select', { onchange: e => { if (!e.target.value) delete s.lut; else s.lut = e.target.value; changed(); } },
          h('option', { value: '', selected: !s.lut }, '— Màu gốc —'),
          ...(S.catalog.luts || []).map(lt => h('option', { value: lt.id, selected: s.lut === lt.id }, `[${lt.category}] ${lt.name} — ${lt.description}`))))),
      sec('Zoom động (Ken Burns / Dynamic Zoom)',
        h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: s.ken_burns, onchange: e => { s.ken_burns = e.target.checked; changed(); } }), 'Phóng / thu động theo thời gian (FilmCraft + CapCut)'),
        s.ken_burns ? h('div', { class: 'col' },
          h('div', { class: 'grid2' }, slider('Zoom đầu', s, 'zoom_start', 1, 1.35, 0.01, v => (+v || 1).toFixed(2) + '×'), slider('Zoom cuối', s, 'zoom_end', 1, 1.35, 0.01, v => (+v || 1).toFixed(2) + '×')),
          h('div', { class: 'f' }, h('span', { class: 'small muted' }, 'Lia'), h('div', { class: 'seg' }, ...[['none', 'Không'], ['left', '← Trái'], ['right', 'Phải →'], ['up', '↑ Lên'], ['down', '↓ Xuống']].map(([v, l]) =>
            h('button', { class: (s.pan || 'none') === v ? 'on' : '', onclick: () => { s.pan = v; changed(); } }, l))))) : null),
      k === 'video' ? sec('Âm thanh gốc', slider('Âm lượng', s, 'volume', 0, 1, 0.05, v => Math.round((v == null ? 1 : v) * 100) + '%')) : null,
      h('div', { class: 'row' }, h('button', { class: 'btn sm', onclick: async () => { const [p] = await pick('visual'); if (p) { s.path = p; changed({ probe: true }); } } }, icon('image', 'sm'), 'Đổi file'), h('div', { class: 'grow' }), delBtn()))];
}

function iOverlay(o, i) {
  const c = compute().overlays.find(x => x.i === i) || { start: 0, dur: 0, lane: 0 }, t = o.graphic ? tpl(o.graphic.template) : null, g = gfxOf(o);
  const color = o.kind === 'motion' ? 'var(--effect)' : o.kind === 'graphic' ? 'var(--vector)' : 'var(--file)';
  const mk = o.kind === 'motion' ? S.motion[mKey(o, c.dur)] : null;
  let preview;
  if (mk && mk.video) preview = h('div', { class: 'preview-box checker' }, h('video', { src: fileUrl(mk.video), autoplay: true, loop: true, muted: true }));
  else if (o.kind !== 'file' && o.graphic) preview = h('div', { class: 'preview-box checker' }, g && g.png ? h('img', { src: fileUrl(g.png) }) : h('div', { class: 'row', style: { position: 'absolute', inset: 0, justifyContent: 'center', color: 'var(--dim)', fontSize: '12px' } }, g && g.pending ? [h('span', { class: 'spinner' }), 'VectorCraft đang vẽ…'] : (g && g.error ? g.error : '')));
  else if (o.path) preview = sceneThumb(o.path, 'preview-box checker');
  return [ihead(color, overlayName(o), `Overlay · track ${c.lane + 1} · ${fmtT(c.start)} → ${fmtT(c.start + c.dur)}`),
    h('div', { class: 'pbody' },
      h('div', { class: 'seg', style: { alignSelf: 'flex-start' } }, ...[['graphic', 'Đồ họa'], ['motion', 'Chuyển động'], ['file', 'File']].map(([v, l]) => h('button', {
        class: o.kind === v ? 'on' : '', onclick: () => {
          if (v === 'motion') o.motion = o.motion || { preset: 'slide_left', format: 'prores' };
          if (v !== 'file' && !o.graphic && !o.path) { const t0 = S.catalog.graphics[0]; o.graphic = { template: t0.id, fields: Object.fromEntries(t0.fields.map(f => [f.key, f.default])) }; }
          if (v === 'graphic' && !o.graphic) return toast('Overlay này là file ảnh — chọn mẫu đồ họa ở tab Đồ họa.', 'err');
          if (v === 'file' && !o.path) return toast('Chọn file bên dưới trước.', 'err');
          o.kind = v; changed({ gfx: true });
        } }, l))),
      preview,
      g && g.doc && o.kind !== 'file' ? h('div', { class: 'small muted' }, 'Sửa tiếp trong VectorCraft: ', h('span', { class: 'mono' }, base(g.doc))) : null,
      o.graphic && o.kind !== 'file' ? sec('Nội dung · ' + (t ? t.name : o.graphic.template),
        h('label', { class: 'f' }, h('span', {}, 'Mẫu'), h('select', { onchange: e => { const nt = tpl(e.target.value); const old = o.graphic.fields || {}; o.graphic = { template: nt.id, fields: Object.fromEntries(nt.fields.map(f => [f.key, old[f.key] || f.default])), accent: o.graphic.accent }; changed({ gfx: true }); } },
          ...S.catalog.graphics.map(x => h('option', { value: x.id, selected: x.id === o.graphic.template }, x.name)))),
        ...(t ? t.fields : []).map(f => h('label', { class: 'f' }, h('span', {}, f.label), h(f.multiline ? 'textarea' : 'input', { value: (o.graphic.fields || {})[f.key] ?? f.default, rows: 3,
          oninput: e => { o.graphic.fields = o.graphic.fields || {}; o.graphic.fields[f.key] = e.target.value; changed({ keep: true, gfx: true }); } }))),
        h('div', { class: 'row' }, h('input', { type: 'color', value: o.graphic.accent || S.spec.studio.accent, onchange: e => { o.graphic.accent = e.target.value; changed({ gfx: true }); } }),
          h('span', { class: 'small muted grow' }, o.graphic.accent ? 'Màu riêng' : 'Màu dự án'), o.graphic.accent ? h('button', { class: 'btn sm ghost', onclick: () => { delete o.graphic.accent; changed({ gfx: true }); } }, 'Dùng màu dự án') : null)) : null,
      o.kind === 'motion' ? sec('Chuyển động · EffectCraft (377 Presets & Layer FX)',
        h('label', { class: 'f' }, h('span', {}, 'Preset / Layer FX'), h('select', { onchange: e => { o.motion.preset = e.target.value; changed(); } },
          ...[...S.catalog.motion, ...(S.catalog.layer_effects || [])].map(p => h('option', { value: p.id, selected: o.motion.preset === p.id }, `[${p.category || 'Motion'}] ${p.name} — ${p.description}`)))),
        h('div', { class: 'f' }, h('span', { class: 'small muted' }, 'Định dạng xuất cho CapCut'), h('div', { class: 'seg' },
          ...[['prores', 'ProRes 4444 .mov'], ['webm', 'WebM VP9 alpha']].map(([v, l]) => h('button', { class: (o.motion.format || 'prores') === v ? 'on' : '', onclick: () => { o.motion.format = v; changed(); } }, l)))),
        h('button', { class: 'btn sm', disabled: !craft('effectcraft').ready || (mk && mk.pending), onclick: () => renderMotion(o, c.dur) }, mk && mk.pending ? h('span', { class: 'spinner' }) : icon('eye', 'sm'), mk && mk.pending ? 'EffectCraft đang render…' : 'Render xem trước'),
        mk && mk.error ? h('div', { class: 'note err' }, mk.error) : null) : null,
      o.kind === 'file' || (!o.graphic && o.path) ? sec('File', h('div', { class: 'small mono muted', style: { wordBreak: 'break-all' } }, o.path || '—'),
        h('button', { class: 'btn sm', onclick: async () => { const [p] = await pick('visual'); if (p) { o.path = p; changed({ probe: true }); } } }, icon('image', 'sm'), 'Chọn file')) : null,
      sec('Thời gian',
        S.spec.sections.length ? h('label', { class: 'f' }, h('span', {}, 'Tính giờ từ'), h('select', { onchange: e => {
          const tl = compute(), abs = ovBase(o, tl) + (o.start_sec || 0);
          if (e.target.value === '') delete o.section; else o.section = +e.target.value;
          o.start_sec = +Math.max(0, abs - ovBase(o, tl)).toFixed(3); changed(); } },
          h('option', { value: '', selected: o.section == null }, 'Đầu video'), ...S.spec.sections.map((x, si) => h('option', { value: si, selected: o.section === si }, 'Đầu ' + secLabel(x, si) + ' (đi theo đoạn)')))) : null,
        h('div', { class: 'grid2' }, numField(o.section != null ? 'Bắt đầu (từ đầu đoạn)' : 'Bắt đầu', o, 'start_sec', { min: 0, unit: 'giây' }), numField('Dài', o, 'duration_sec', { min: 0.2, optional: true, placeholder: c.dur.toFixed(1), unit: 'giây' })),
        h('button', { class: 'btn sm ghost', onclick: () => { o.start_sec = +Math.max(0, S.t - ovBase(o, compute())).toFixed(2); changed(); } }, 'Đặt bắt đầu tại playhead (' + fmtT(S.t) + ')')),
      sec('Vị trí (đơn vị CapCut: ±1 = nửa khung)',
        h('div', { class: 'grid2' }, slider('Ngang', o, 'x', -1, 1, 0.01, v => (+v || 0).toFixed(2)), slider('Dọc', o, 'y', -1, 1, 0.01, v => (+v || 0).toFixed(2))),
        h('div', { class: 'grid2' }, slider('Tỉ lệ', o, 'scale', 0.2, 2, 0.01, v => Math.round((v || 1) * 100) + '%'), slider('Độ mờ', o, 'opacity', 0, 1, 0.01, v => Math.round((v ?? 1) * 100) + '%'))),
      o.kind !== 'motion' ? sec('Fade trong CapCut', h('div', { class: 'grid2' }, numField('Vào', o, 'fade_in_sec', { min: 0, unit: 'giây' }), numField('Ra', o, 'fade_out_sec', { min: 0, unit: 'giây' }))) : null,
      h('div', { class: 'row' }, h('button', { class: 'btn sm', onclick: () => { const n = clone(o); n.start_sec = +(c.start + c.dur - ovBase(o, compute())).toFixed(2); S.spec.overlays.push(n); S.sel = { type: 'overlay', i: S.spec.overlays.length - 1 }; changed({ gfx: true }); } }, icon('dup', 'sm'), 'Nhân bản'), h('div', { class: 'grow' }), delBtn()))];
}
function iSection(x, i) {
  const tl = compute(), a = x.voice, v = tl.voice.find(c => c.sec === i) || { start: 0, dur: 0 };
  const scenes = tl.scenes.filter(c => c.sec === i), pics = scenes.reduce((n, c) => n + c.dur, 0), gap = x.gap_after_sec > 0 ? x.gap_after_sec : 0;
  return [ihead('var(--voice)', secLabel(x, i), `Đoạn ${i + 1} · ${fmtT(v.start)} → ${fmtT(v.start + v.dur + gap)}`),
    h('div', { class: 'pbody' },
      h('audio', { src: fileUrl(a.path), controls: true, style: { width: '100%' } }),
      h('div', { class: 'small mono muted', style: { wordBreak: 'break-all' } }, a.path),
      h('label', { class: 'f' }, h('span', {}, 'Tên đoạn'), h('input', { value: x.desc || '', placeholder: a.desc || base(a.path), oninput: e => { x.desc = e.target.value || undefined; changed({ keep: true }); } })),
      h('div', { class: 'grid2' }, numField('Lặng sau giọng', x, 'gap_after_sec', { min: 0, unit: 'giây' }), h('div')),
      slider('Âm lượng', a, 'volume', 0, 2, 0.05, n => Math.round(n * 100) + '%'),
      ...tl.errors.filter(e => e.sec === i).map(e => h('div', { class: 'note err' }, e.msg)),
      h('div', { class: 'note' }, `${scenes.length} cảnh · ${fmtD(pics)} hình cho ${fmtD(v.dur)} giọng${gap ? ' + ' + fmtD(gap) + ' lặng' : ''}. Cảnh không đặt thời lượng tự khớp: ảnh giãn lấp chỗ trống, video bị cắt bớt theo tỉ lệ (không kéo giãn). Đoạn sau bắt đầu đúng lúc giọng này hết${gap ? ' + khoảng lặng' : ''}.`),
      h('div', { class: 'row wrap' },
        h('button', { class: 'btn sm', onclick: () => addScenes(false, i) }, icon('plus', 'sm'), 'Thêm cảnh'),
        h('button', { class: 'btn sm', onclick: async () => { const [p] = await pick('audio'); if (p) { a.path = p; changed({ probe: true }); } } }, icon('voice', 'sm'), 'Đổi file giọng')),
      h('div', { class: 'row' }, h('button', { class: 'btn sm', disabled: i === 0, onclick: () => moveSection(i, -1) }, icon('up', 'sm')), h('button', { class: 'btn sm', disabled: i === S.spec.sections.length - 1, onclick: () => moveSection(i, 1) }, icon('down', 'sm')), h('div', { class: 'grow' }), delBtn('Xoá đoạn')))];
}
function iVoice(a, i) {
  const c = compute().voice.find(v => v.sec == null && v.i === i) || {};
  return [ihead('var(--voice)', a.desc || base(a.path), `Giọng đọc ${i + 1} · ${fmtT(c.start)} → ${fmtT(c.start + c.dur)}`),
    h('div', { class: 'pbody' },
      h('audio', { src: fileUrl(a.path), controls: true, style: { width: '100%' } }),
      h('div', { class: 'small mono muted', style: { wordBreak: 'break-all' } }, a.path),
      h('div', { class: 'grid2' }, numField('Bắt đầu (trống = nối tiếp)', a, 'start_sec', { optional: true, min: 0, placeholder: (c.start || 0).toFixed(2) }), h('div')),
      slider('Âm lượng', a, 'volume', 0, 2, 0.05, v => Math.round(v * 100) + '%'),
      h('div', { class: 'row' }, h('button', { class: 'btn sm', disabled: i === 0, onclick: () => move(S.spec.voiceovers, i, -1, 'voice') }, icon('up', 'sm')), h('button', { class: 'btn sm', disabled: i === S.spec.voiceovers.length - 1, onclick: () => move(S.spec.voiceovers, i, 1, 'voice') }, icon('down', 'sm')), h('div', { class: 'grow' }), delBtn()))];
}
function iBgm(b, i) {
  return [ihead('var(--bgm)', b.desc || base(b.path), 'Nhạc nền ' + (i + 1)),
    h('div', { class: 'pbody' },
      h('audio', { src: fileUrl(b.path), controls: true, style: { width: '100%' } }),
      slider('Âm lượng', b, 'volume', 0, 1, 0.01, v => Math.round(v * 100) + '%'),
      h('div', { class: 'grid2' }, numField('Fade vào', b, 'fade_in_sec', { min: 0, unit: 'giây' }), numField('Fade ra', b, 'fade_out_sec', { min: 0, unit: 'giây' })),
      numField('Bắt đầu', b, 'start_sec', { min: 0, unit: 'giây' }),
      h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: b.loop_to_fill !== false, onchange: e => { b.loop_to_fill = e.target.checked; changed(); } }), 'Lặp tới hết video'),
      h('label', { class: 'chk' }, h('input', { type: 'checkbox', checked: b.duck_under_voice !== false, onchange: e => { b.duck_under_voice = e.target.checked; changed(); } }), 'Tự hạ khi có giọng đọc'),
      b.duck_under_voice !== false ? slider('Mức khi có giọng', b, 'duck_ratio', 0, 1, 0.05, v => Math.round((v ?? 0.4) * 100) + '% của âm lượng') : null,
      h('div', { class: 'row' }, h('div', { class: 'grow' }), delBtn()))];
}

// ---------------------------------------------------------------- stage
let stageEls = null;
function renderStage() {
  const el = $('#stage'), sp = S.spec;
  if ($('#screen') && stageEls) {
    $('#stagecount').textContent = `${allScenes().length} cảnh · ${sp.overlays.length} overlay`;
    $('#stagesize').textContent = `${sp.width}×${sp.height}`;
    fitScreen(); updateStage(); stageExtras(); return;
  }
  put(el, 
    h('div', { class: 'stagebar' }, h('span', { class: 'eyebrow' }, A.on ? 'Bản dựng của agent' : 'Xem trước'), h('span', { class: 'small muted', id: 'stagesize' }, `${sp.width}×${sp.height}`), h('div', { class: 'grow' }),
      h('div', { id: 'stagetools', class: 'row' })),
    h('div', { class: 'viewer', id: 'viewer' }, h('div', { class: 'screen', id: 'screen' })),
    h('div', { class: 'transport' },
      h('div', { class: 'left' }, h('span', { class: 'small muted', id: 'stagecount' }, `${allScenes().length} cảnh · ${sp.overlays.length} overlay`)),
      h('button', { class: 'btn icon ghost', title: 'Về đầu', onclick: () => seek(0) }, icon('skipb')),
      h('button', { class: 'play', id: 'playbtn', title: 'Phát / dừng (Space)', onclick: togglePlay }, icon(S.playing ? 'pause' : 'play')),
      h('span', { class: 'time', id: 'tc' }, ''),
      h('div', { class: 'right' })));
  stageEls = { main: null, ovs: {} };
  fitScreen(); updateStage(); stageExtras();
}
function fitScreen() {
  const v = $('#viewer'), s = $('#screen'); if (!v || !s) return;
  const W = v.clientWidth - 36, H = v.clientHeight - 36, r = S.spec.width / S.spec.height;
  let w = W, hh = W / r; if (hh > H) { hh = H; w = H * r; }
  s.style.width = Math.max(40, w) + 'px'; s.style.height = Math.max(24, hh) + 'px';
}
function kbAt(s, p) {
  if (!s.ken_burns) return { z: 1, x: 0, y: 0 };
  let z0 = s.zoom_start || 1, z1 = s.zoom_end || 1.06; const pan = s.pan || 'none';
  let x = 0, y = 0;
  if (pan !== 'none') { z0 = Math.max(z0, 1.08); z1 = Math.max(z1, z0); const d = (z0 - 1) * 0.85;
    const [ax, f, t] = { left: ['x', d, -d], right: ['x', -d, d], up: ['y', -d, d], down: ['y', d, -d] }[pan]; const v = f + (t - f) * p; if (ax === 'x') x = v; else y = v; }
  return { z: z0 + (z1 - z0) * p, x, y };
}
function mediaEl(path, cls) {
  const k = kindOf(path);
  const m = k === 'video' ? h('video', { src: fileUrl(path), muted: true, preload: 'auto', playsInline: true }) : h('img', { src: fileUrl(path), alt: '' });
  return h('div', { class: 'layer ' + cls }, m);
}
function updateStage() {
  const scr = $('#screen'); if (!scr || !stageEls) return;
  const tl = compute(), t = S.t, sp = S.spec;
  $('#tc') && ($('#tc').textContent = `${fmtT(t)} / ${fmtT(tl.total)}`);
  // main
  const sc = tl.scenes.find(c => t >= c.start && t < c.start + c.dur) || (t >= tl.mainEnd && tl.scenes.length && t < tl.mainEnd + 0.001 ? tl.scenes[tl.scenes.length - 1] : null);
  const want = sc ? sc.s.path : null;
  if (!stageEls.main || stageEls.main.path !== want) {
    if (stageEls.main) stageEls.main.el.remove();
    stageEls.main = want ? { path: want, el: mediaEl(want, 'main') } : null;
    if (stageEls.main) scr.prepend(stageEls.main.el);
  }
  const anyScene = allScenes().length > 0;
  if (!anyScene && !sp.overlays.length && !scr.querySelector('.blank')) scr.append(h('div', { class: 'blank' }, icon('film'), !A.on ? 'Thêm cảnh ở bảng bên trái để bắt đầu' : A.byKey[A.stage] ? 'Đang chờ spec từ agent…' : 'Agent chưa dựng gì để xem'));
  if ((anyScene || sp.overlays.length) && scr.querySelector('.blank')) scr.querySelector('.blank').remove();
  if (stageEls.main && sc) {
    const s = sc.s, k = kbAt(s, clamp((t - sc.start) / sc.dur, 0, 1));
    stageEls.main.el.style.transform = `translate(${k.x * 50}%, ${-k.y * 50}%) scale(${k.z})`;
    const v = stageEls.main.el.querySelector('video');
    if (v) { const want = (s.trim_start_sec || 0) + (t - sc.start); if (Math.abs(v.currentTime - want) > (S.playing ? 0.35 : 0.04)) v.currentTime = want; v.volume = clamp(s.volume == null ? 1 : s.volume, 0, 1); v.muted = !S.playing; if (S.playing && v.paused) v.play().catch(() => {}); if (!S.playing && !v.paused) v.pause(); }
  }
  // overlays
  const live = new Set();
  for (const c of tl.overlays) {
    if (t < c.start || t >= c.start + c.dur) continue;
    const o = sp.overlays[c.i]; live.add(c.i);
    let src = null, kind = 'img', ph = null, phSub = 'VectorCraft sẽ vẽ khi sẵn sàng', wait = false;
    const mk = o.kind === 'motion' ? S.motion[mKey(o, c.dur)] : null;
    if (A.on) ({ src = null, kind = 'img', ph = null, phSub = '', wait = false } = agentOvSrc(o, c.i));
    else if (mk && mk.video) { src = mk.video; kind = 'video'; }
    else if (o.graphic && o.kind !== 'file') { const g = gfxOf(o); if (g && g.png) src = g.png; else ph = (tpl(o.graphic.template) || {}).name || 'Đồ họa'; }
    else if (o.path) { src = o.path; kind = kindOf(o.path) === 'video' ? 'video' : 'img'; }
    const key = (src || 'ph:' + ph + phSub + wait) + '|' + kind;
    let e = stageEls.ovs[c.i];
    if (!e || e.key !== key) {
      if (e) e.el.remove();
      const inner = src ? (kind === 'video' ? h('video', { src: fileUrl(src), muted: true, preload: 'auto', playsInline: true }) : h('img', { src: fileUrl(src), alt: '' })) : h('div', { class: 'ph-ov' + (wait ? ' wait' : '') }, ph, h('div', { style: { fontSize: '1.6cqh', opacity: .7 } }, phSub));
      e = stageEls.ovs[c.i] = { key, el: h('div', { class: 'layer ov' }, inner) }; scr.append(e.el);
    }
    const lt = t - c.start;
    let op = o.opacity ?? 1;
    if (o.kind !== 'motion' || !(A.on ? kind === 'video' : mk && mk.video)) {
      const fi = Math.min(o.fade_in_sec || 0, c.dur / 2), fo = Math.min(o.fade_out_sec || 0, c.dur / 2);
      if (o.kind === 'motion') op *= Math.min(1, lt / 0.5, (c.dur - lt) / 0.5);
      else { if (fi > 0 && lt < fi) op *= lt / fi; if (fo > 0 && lt > c.dur - fo) op *= (c.dur - lt) / fo; }
    }
    e.el.style.opacity = clamp(op, 0, 1);
    e.el.style.transform = `translate(${(o.x || 0) * 50}%, ${-(o.y || 0) * 50}%) scale(${o.scale || 1})`;
    e.el.style.zIndex = 10 + c.lane;
    const v = e.el.querySelector('video');
    if (v) { if (Math.abs(v.currentTime - lt) > (S.playing ? 0.35 : 0.04)) v.currentTime = lt; if (S.playing && v.paused) v.play().catch(() => {}); if (!S.playing && !v.paused) v.pause(); }
  }
  for (const k of Object.keys(stageEls.ovs)) if (!live.has(+k)) { stageEls.ovs[k].el.remove(); delete stageEls.ovs[k]; }
  if (!scr.querySelector('.safe')) scr.append(h('div', { class: 'safe' }));
  syncAudio(tl);
}

// ---------------------------------------------------------------- playback
const audios = {};
function audioFor(key, path) { if (!audios[key] || audios[key].path !== path) { if (audios[key]) audios[key].el.pause(); audios[key] = { path, el: new Audio(fileUrl(path)) }; } return audios[key].el; }
function syncAudio(tl) {
  const t = S.t, sp = S.spec, voiceOn = tl.voice.some(v => t >= v.start && t < v.start + v.dur);
  const want = new Set();
  tl.voice.forEach((v, n) => { const a = v.a; if (S.playing && t >= v.start && t < v.start + v.dur) { const el = audioFor('v' + n, a.path); want.add('v' + n); el.volume = clamp(a.volume, 0, 1); if (Math.abs(el.currentTime - (t - v.start)) > 0.3) el.currentTime = t - v.start; if (el.paused) el.play().catch(() => {}); } });
  tl.bgm.forEach((b, n) => { const m = sp.bgm[b.i]; if (S.playing && t >= b.start && t < b.start + b.dur) { const el = audioFor('b' + n, m.path); want.add('b' + n);
    let vol = m.volume; if (m.duck_under_voice !== false && voiceOn) vol *= (m.duck_ratio ?? 0.4); const lt = t - b.start;
    if (lt < (m.fade_in_sec || 0) && !b.loop) vol *= lt / m.fade_in_sec; if (t > tl.total - (m.fade_out_sec || 0)) vol *= Math.max(0, (tl.total - t) / m.fade_out_sec);
    el.volume = clamp(vol, 0, 1); if (Math.abs(el.currentTime - lt) > 0.3) el.currentTime = lt; if (el.paused) el.play().catch(() => {}); } });
  for (const [k, a] of Object.entries(audios)) if (!want.has(k) && !a.el.paused) a.el.pause();
}
let last = 0;
function tick(now) {
  if (!S.playing) return;
  const dt = (now - last) / 1000; last = now;
  const tl = compute(); S.t = Math.min(tl.total, S.t + dt);
  if (S.t >= tl.total) { stop(); }
  updateStage(); updatePlayhead();
  if (S.playing) requestAnimationFrame(tick);
}
function togglePlay() { if (S.playing) stop(); else { const tl = compute(); if (!tl.total) return; if (S.t >= tl.total - 0.05) S.t = 0; S.playing = true; last = performance.now(); requestAnimationFrame(tick); } $('#playbtn') && put($('#playbtn'), icon(S.playing ? 'pause' : 'play')); }
function stop() { S.playing = false; for (const a of Object.values(audios)) a.el.pause(); $('#playbtn') && put($('#playbtn'), icon('play')); updateStage(); }
function seek(t) { if (A.on && !A.autoSeek) A.userSeek = true; S.t = clamp(t, 0, compute().total || 0); updateStage(); updatePlayhead(); }

// ---------------------------------------------------------------- timeline
function updatePlayhead() { const ph = $('#playhead'); if (ph) ph.style.left = (S.t * S.zoom) + 'px'; }
function renderTimeline() {
  const el = $('#timeline'), tl = compute(), sp = S.spec, Z = S.zoom;
  const total = Math.max(tl.total, 10), width = total * Z + 240;
  const oldScroll = $('#tlscroll') ? [$('#tlscroll').scrollLeft, $('#tlscroll').scrollTop] : [0, 0];
  const lanes = [];
  for (let k = tl.lanes - 1; k >= 0; k--) lanes.push({ id: 'ov' + k, label: 'Overlay ' + (k + 1), sub: k === 0 ? 'đồ họa · chuyển động' : '', color: 'var(--vector)' });
  lanes.push({ id: 'main', label: 'Hình chính', sub: `${allScenes().length} cảnh${sp.sections.length ? ' · ' + sp.sections.length + ' đoạn' : ''}`, color: 'var(--scene)' }, { id: 'voice', label: 'Giọng đọc', sub: fmtD(tl.voiceEnd), color: 'var(--voice)' }, { id: 'bgm', label: 'Nhạc nền', sub: sp.bgm.length ? 'lặp · fade · ducking' : '', color: 'var(--bgm)' });
  const step = Math.max(Z >= 80 ? 1 : Z >= 30 ? 2 : Z >= 12 ? 5 : Z >= 5 ? 10 : 30, Math.ceil(total / 600));
  const ticks = []; for (let s = 0; s <= total + 5; s += step) ticks.push(h('div', { class: 'tk', style: { left: s * Z + 'px' } }, fmtD(s).replace('.0s', 's')));
  const laneEls = lanes.map(L => {
    const lane = h('div', { class: 'lane', 'data-lane': L.id, onpointerdown: e => { if (e.target === lane) { seekFromEvent(e); } } });
    if (L.id === 'main') tl.scenes.forEach(c => { const s = c.s, pulse = A.flashClip === 'scene:' + c.n; lane.append(h('div', { class: 'clip scene' + (isSel('scene', c.k, c.sec) ? ' sel' : '') + (pulse ? ' live-pulse' : ''), style: { left: c.start * Z + 'px', width: Math.max(4, c.dur * Z - 2) + 'px' }, title: base(s.path) + (c.sec != null ? ' — ' + secLabel(sp.sections[c.sec], c.sec) : ''), onpointerdown: e => { e.stopPropagation(); select('scene', c.k, c.sec); } },
      kindOf(s.path) === 'image' ? h('div', { class: 'tb', style: { backgroundImage: `url("${fileUrl(s.path)}")` } }) : null, h('span', {}, (c.n + 1) + '. ' + (s.transition ? '⚡ ' : '') + (s.video_effect ? '✨ ' : '') + (s.desc || base(s.path))))); });
    else if (L.id.startsWith('ov')) { const k = +L.id.slice(2); tl.overlays.filter(c => c.lane === k).forEach(c => { const o = sp.overlays[c.i], pulse = A.flashClip === 'ov:' + c.i;
      const clip = h('div', { class: 'clip ' + (o.kind === 'motion' ? 'motion' : o.kind === 'graphic' ? 'graphic' : 'file') + (isSel('overlay', c.i) ? ' sel' : '') + (pulse ? ' live-pulse' : ''), style: { left: c.start * Z + 'px', width: Math.max(6, c.dur * Z - 2) + 'px' }, title: overlayName(o) + ' — kéo để dời, kéo mép phải để đổi độ dài' },
        o.kind === 'motion' ? icon('spark', 'sm') : icon('layers', 'sm'), h('span', {}, overlayName(o)));
      clip.addEventListener('pointerdown', e => dragOverlay(e, clip, c)); lane.append(clip); }); }
    else if (L.id === 'voice') tl.voice.forEach(c => { const a = c.a, [ty, ix] = c.sec != null ? ['section', c.sec] : ['voice', c.i]; lane.append(h('div', { class: 'clip voice' + (isSel(ty, ix) ? ' sel' : ''), style: { left: c.start * Z + 'px', width: Math.max(4, c.dur * Z - 2) + 'px' }, onpointerdown: e => { e.stopPropagation(); select(ty, ix); } }, h('div', { class: 'wave' }), h('span', { style: { position: 'relative' } }, a.desc || (c.sec != null ? secLabel(sp.sections[c.sec], c.sec) : base(a.path))))); });
    else if (L.id === 'bgm') tl.bgm.forEach(c => { const b = sp.bgm[c.i]; lane.append(h('div', { class: 'clip bgm' + (c.loop ? ' loop' : '') + (isSel('bgm', c.i) ? ' sel' : ''), style: { left: c.start * Z + 'px', width: Math.max(4, c.dur * Z - 2) + 'px' }, onpointerdown: e => { e.stopPropagation(); select('bgm', c.i); } }, h('div', { class: 'wave' }), h('span', { style: { position: 'relative' } }, (c.loop ? '↻ ' : '') + base(b.path)))); });
    return lane;
  });
  const empty = !sp.sections.length && !sp.scenes.length && !sp.voiceovers.length && !sp.overlays.length;
  put(el, 
    h('div', { class: 'tl-resize', onpointerdown: resizeTimeline }),
    h('div', { class: 'tlbar' },
      h('span', { class: 'eyebrow' }, 'Timeline'), h('span', { class: 'small muted' }, `${fmtD(tl.total)} · ${2 + tl.lanes + (sp.bgm.length ? 1 : 0)} track trong CapCut`),
      h('div', { class: 'grow' }),
      A.on ? h('span', { class: 'tag capcut' }, 'Agent · chỉ xem') : h('button', { class: 'btn sm ghost', onclick: () => S.tab === 'graphics' ? null : (S.tab = 'graphics', render()) }, icon('plus', 'sm'), 'Đồ họa tại playhead'),
      h('span', { class: 'small muted' }, 'Thu phóng'), h('input', { type: 'range', class: 'zoom', min: 3, max: 160, step: 1, value: S.zoom, oninput: e => { S.zoom = +e.target.value; renderTimeline(); } })),
    h('div', { class: 'tlwrap' },
      h('div', { class: 'tlheads', id: 'tlheads' }, h('div', { class: 'rh' }), ...lanes.map(L => h('div', { class: 'tlhead' }, h('span', { class: 'sw', style: { background: L.color } }), h('div', {}, L.label, L.sub ? h('small', {}, L.sub) : null)))),
      h('div', { class: 'tlscroll', id: 'tlscroll', onscroll: e => { $('#tlheads').scrollTop = e.target.scrollTop; } },
        h('div', { class: 'tlinner', style: { width: width + 'px' } },
          h('div', { class: 'ruler', onpointerdown: e => { seekFromEvent(e); scrubStart(e); } }, ...ticks),
          ...laneEls,
          ...tl.sectionStarts.map((st, i) => h('div', { class: 'secmark', style: { left: st * Z + 'px' }, title: secLabel(sp.sections[i], i) })),
          tl.total ? h('div', { class: 'endmark', style: { left: tl.total * Z + 'px' } }) : null,
          h('div', { class: 'playhead', id: 'playhead', style: { left: S.t * Z + 'px' } })),
        empty ? h('div', { class: 'small dim', style: { position: 'absolute', left: '20px', top: '60px' } }, 'Timeline trống — cảnh, giọng đọc, nhạc và overlay sẽ xếp ở đây đúng như trong CapCut.') : null)));
  const sc = $('#tlscroll'); sc.scrollLeft = oldScroll[0]; sc.scrollTop = oldScroll[1];
}
function seekFromEvent(e) { const r = $('#tlscroll').getBoundingClientRect(); seek((e.clientX - r.left + $('#tlscroll').scrollLeft) / S.zoom); }
function scrubStart(e) {
  const mv = ev => seekFromEvent(ev), up = () => { removeEventListener('pointermove', mv); removeEventListener('pointerup', up); };
  addEventListener('pointermove', mv); addEventListener('pointerup', up);
}
function dragOverlay(e, clip, c) {
  e.stopPropagation(); e.preventDefault();
  if (A.on) return select('overlay', c.i);
  const o = S.spec.overlays[c.i], rect = clip.getBoundingClientRect(), edge = e.clientX > rect.right - 8;
  const x0 = e.clientX, s0 = c.start, d0 = c.dur, b0 = ovBase(o, compute()); let moved = false;
  if (!(S.sel && S.sel.type === 'overlay' && S.sel.i === c.i)) { S.sel = { type: 'overlay', i: c.i }; renderInsp(); renderPanel(); }
  clip.classList.add('drag', 'sel');
  const mv = ev => { const dx = (ev.clientX - x0) / S.zoom; if (Math.abs(ev.clientX - x0) > 2) moved = true;
    if (edge) { o.duration_sec = Math.max(0.3, Math.round((d0 + dx) * 10) / 10); clip.style.width = (o.duration_sec * S.zoom - 2) + 'px'; }
    else { const at = Math.max(b0, Math.round((s0 + dx) * 10) / 10); o.start_sec = +(at - b0).toFixed(2); clip.style.left = (at * S.zoom) + 'px'; } };
  const up = () => { removeEventListener('pointermove', mv); removeEventListener('pointerup', up); if (moved) changed(); else render(); };
  addEventListener('pointermove', mv); addEventListener('pointerup', up);
}
function resizeTimeline(e) {
  const y0 = e.clientY, h0 = $('#timeline').getBoundingClientRect().height;
  const mv = ev => { document.documentElement.style.setProperty('--tl-h', clamp(h0 - (ev.clientY - y0), 160, innerHeight * 0.6) + 'px'); fitScreen(); };
  const up = () => { removeEventListener('pointermove', mv); removeEventListener('pointerup', up); };
  addEventListener('pointermove', mv); addEventListener('pointerup', up);
}

// ---------------------------------------------------------------- modal + build
function modal(title, body, actions = [], wide = false, onclose) {
  const close = () => { put(root, ); onclose && onclose(); document.removeEventListener('keydown', esc); };
  const esc = e => { if (e.key === 'Escape') close(); };
  const root = $('#modal-root');
  put(root, h('div', { class: 'scrim', onclick: e => { if (e.target.classList.contains('scrim')) close(); } },
    h('div', { class: 'modal' + (wide ? ' wide' : ''), role: 'dialog', 'aria-label': title },
      h('div', { class: 'mhead' }, h('h3', { class: 'grow' }, title), h('button', { class: 'btn icon sm ghost', onclick: close, title: 'Đóng' }, icon('x', 'sm'))),
      h('div', { class: 'mbody', id: 'mbody' }, body),
      h('div', { class: 'mfoot', id: 'mfoot' }, ...actions, h('button', { class: 'btn', onclick: close }, 'Đóng')))));
  document.addEventListener('keydown', esc);
  return close;
}
async function openBuild() {
  stop(); await save();
  const sp = S.spec;
  modal('Xuất sang CapCut — ' + sp.project_name, h('div', { class: 'col', id: 'buildbody' }, h('div', { class: 'row' }, h('span', { class: 'spinner' }), 'Đang gửi dự án…')), [], true);
  let job;
  try { job = (await api.post('/api/build', { spec: sp })).job; } catch (e) { return showBuild({ state: 'error', error: e.message, log: [] }); }
  const poll = async () => {
    let j; try { j = await api.get('/api/jobs/' + job); } catch (e) { j = { state: 'error', error: e.message, log: [] }; }
    if (!$('#buildbody')) return; showBuild(j);
    if (j.state === 'running') setTimeout(poll, 350); else { loadDrafts(); loadStatus(); }
  };
  poll();
}
function showBuild(j) {
  const body = $('#buildbody'); if (!body) return;
  const steps = h('div', { class: 'steps' }, ...(j.log || []).map((l, i, a) => h('div', { class: i === a.length - 1 ? 'last' : '' }, l)));
  if (j.state === 'running') { put(body, h('div', { class: 'row' }, h('span', { class: 'spinner' }), h('b', {}, 'Đang dựng…'), h('span', { class: 'muted small' }, 'VectorCraft / EffectCraft có thể mất vài giây mỗi overlay')), steps); steps.scrollTop = 1e9; return; }
  if (j.state === 'error') { put(body, h('div', { class: 'note err' }, h('b', {}, 'Không xuất được. '), h('div', { style: { whiteSpace: 'pre-wrap', marginTop: '6px' } }, j.error)), steps); return; }
  const r = j.result, d = r.draft;
  put(body, 
    h('div', { class: 'bigok' }, h('div', { class: 'ck' }, icon('check')), h('div', { class: 'grow' }, h('b', {}, `Đã tạo draft “${r.project_name}” · ${fmtD(d.duration_sec)}`),
      h('div', { class: 'small muted mono ellipsis' }, d.draft_dir)), h('span', { class: 'small muted' }, r.seconds.toFixed(1) + ' giây')),
    r.capcut_running ? h('div', { class: 'note warn' }, h('b', {}, 'CapCut đang mở. '), 'Tắt hẳn CapCut (cả khay hệ thống) rồi mở lại để dự án hiện ở đầu tab Projects.') : (d.registered ? h('div', { class: 'note' }, 'Mở CapCut PC: dự án nằm ', h('b', {}, 'đầu tab Projects'), '.') : null),
    ...(r.warnings || []).map(w => h('div', { class: 'note warn' }, w)),
    d.replaced_previous ? h('div', { class: 'small muted' }, 'Bản cũ cùng tên đã được cất vào: ', h('span', { class: 'mono' }, d.replaced_previous)) : null,
    h('div', { class: 'grid2', style: { gap: '16px', alignItems: 'start' } },
      h('div', { class: 'col' }, h('span', { class: 'eyebrow' }, 'Track trong CapCut'), h('div', { class: 'trk' }, ...r.tracks.flatMap(t => [h('span', {}, t.label), h('span', {}, `${t.clips.length} đoạn`)]))),
      h('div', { class: 'col' }, h('span', { class: 'eyebrow' }, 'Craft đã tạo'), r.assets.length ? h('div', { class: 'assets' }, ...r.assets.map(a => h('div', { class: 'asset' },
        h('span', { class: 'tag ' + a.craft.replace('craft', '') }, a.craft), h('span', { class: 'p', title: a.path }, base(a.path)), h('span', { class: 'small muted' }, a.cached ? 'dùng lại' : 'mới')))) : h('div', { class: 'small muted' }, 'Không có overlay đồ họa / chuyển động.'))),
    r.preview ? h('div', { class: 'col' }, h('span', { class: 'eyebrow' }, 'Xem trước · FilmCraft'), h('video', { src: fileUrl(r.preview.mp4), controls: true, style: { width: '100%', borderRadius: '8px', background: '#000' } })) : null,
    h('details', {}, h('summary', { class: 'small muted' }, 'Nhật ký'), steps));
  $('#mfoot').prepend(
    h('button', { class: 'btn', onclick: () => api.post('/api/open', { what: 'folder', path: d.draft_dir }) }, icon('folder', 'sm'), 'Mở thư mục draft'),
    h('button', { class: 'btn primary', onclick: () => api.post('/api/open', { what: 'capcut' }).then(() => toast('Đang mở CapCut…', 'ok')).catch(e => toast(e.message, 'err')) }, icon('capcut', 'sm'), 'Mở CapCut'));
}

// ---------------------------------------------------------------- agent activity
// An agent building over MCP runs in another process; capcut_core::activity posts each tool call
// to the server, which streams it here (/api/activity/stream, SSE). Agent mode shows that build
// read-only on the stage and timeline. The user's own project is saved first and put back on
// exit; nothing here writes over it.
const LOST_MS = 30000; // a running call with no news (heartbeats every 5 s) for this long: the agent is gone
function connectActivity() {
  if (A.es) A.es.close();
  const es = A.es = new EventSource('/api/activity/stream' + (A.lastId ? '?after=' + A.lastId : ''));
  es.onmessage = e => { let ev; try { ev = JSON.parse(e.data); } catch { return; } onActivity(ev); };
  // after the kept events: when each agent was last heard from, then live events
  es.addEventListener('ready', e => {
    let r = {}; try { r = JSON.parse(e.data); } catch {}
    const skew = Date.now() - (r.now || Date.now());
    for (const [s, t] of Object.entries(r.seen || {})) A.seen[s] = Math.max(A.seen[s] || 0, t + skew);
    const first = !A.ready; A.ready = true;
    if (first) { const c = A.calls.find(x => x.build && callState(x) === 'running'); if (c) follow(c); }
    agentRender('full');
  });
  es.addEventListener('lagged', () => connectActivity()); // fell behind: reconnect from the last id
}
function onActivity(ev) {
  if (ev.id) A.lastId = Math.max(A.lastId, ev.id);
  const sid = ev.session; if (!sid) return;
  if (ev.seq != null) { if (ev.seq <= (A.seq[sid] || 0)) return; A.seq[sid] = ev.seq; }
  if (A.ready) A.seen[sid] = Date.now();
  if (ev.type === 'alive') return;
  const key = sid + ':' + ev.call;
  let c = A.byKey[key], level = 'log';
  if (ev.type === 'call_start') {
    c = A.byKey[key] = { key, session: sid, client: ev.client || 'Windows Agent', tool: ev.tool || '?', summary: ev.summary || '', build: !!ev.build, args: ev.args,
      started: ev.ts, ended: null, state: 'running', log: [], spec: null, assets: {}, report: null, out: null, error: null, lastStep: `▶ Bắt đầu ${ev.tool || ''} ${ev.summary || ''}` };
    A.calls.unshift(c);
    for (const x of A.calls.splice(40)) if (x.key === A.stage || x.key === A.sel) A.calls.push(x); else delete A.byKey[x.key];
    c.log.push({ ts: ev.ts, kind: 'start', text: `▶ ${c.tool}${c.summary ? '  ' + c.summary : ''}` });
    level = 'full';
  }
  if (!c) return; // the start of this call was missed
  if (ev.type === 'spec') {
    c.spec = agentSpec(ev.spec);
    const sp = c.spec, n = sp.sections.reduce((k, x) => k + x.scenes.length, sp.scenes.length);
    const slug = slugify(sp.project_name);
    c.lastStep = `⚡ Nạp cấu trúc: ${n} cảnh · ${sp.overlays.length} overlay · ${sp.bgm.length} nhạc nền`;
    c.log.push({ ts: ev.ts, kind: 'spec', text: `spec: ${sp.width}×${sp.height} · ${sp.sections.length ? sp.sections.length + ' đoạn · ' : ''}${n} cảnh · ${sp.overlays.length} overlay · ${sp.voiceovers.length + sp.sections.length} giọng · ${sp.bgm.length} nhạc` });
    // Keep A.home synced to the latest agent spec so exiting Agent view keeps the project on timeline
    A.home = { spec: clone(sp), slug, sel: null, t: 0, tab: 'media', saved: true };
    try { localStorage.setItem('ccds.slug', slug); } catch {}
    refreshSpecs();
    if (A.on && (A.stage === key || !A.stage)) showOnStage(c);
    else if (!A.on && A.follow) enterAgent(key);
    level = 'full';
  } else if (ev.type === 'log') {
    c.log.push({ ts: ev.ts, kind: 'log', text: ev.line });
    c.lastStep = ev.line;
    if (A.ready && A.on && A.stage === key) {
      const mScene = ev.line.match(/🎬 Cảnh (\d+)/);
      const mOv = ev.line.match(/\(overlay (\d+)\)/);
      if (mScene) {
        const idx = parseInt(mScene[1], 10) - 1;
        const sc = compute().scenes[idx];
        if (sc && !S.playing && !A.userSeek) {
          S.sel = { type: 'scene', i: sc.k, sec: sc.sec };
          A.flashClip = 'scene:' + idx;
          A.autoSeek = true; seek(sc.start + Math.min(sc.dur * 0.35, 1.0)); A.autoSeek = false;
          scrollTimelineTo(sc.start);
        }
        level = 'full';
      } else if (mOv) {
        const idx = parseInt(mOv[1], 10) - 1;
        const ov = compute().overlays.find(x => x.i === idx);
        if (ov && !S.playing && !A.userSeek) {
          S.sel = { type: 'overlay', i: idx };
          A.flashClip = 'ov:' + idx;
          A.autoSeek = true; seek(ov.start + Math.min(ov.dur * 0.4, 1.5)); A.autoSeek = false;
          scrollTimelineTo(ov.start);
        }
        level = 'full';
      } else if (/^[🎨✨🎵📦🎞️⚡✓]/.test(ev.line)) {
        level = 'full';
      }
    }
  } else if (ev.type === 'asset') {
    const a = ev.asset; (c.assets[a.overlay] = c.assets[a.overlay] || []).push(a);
    const stepTxt = `✓ ${a.craft === 'vectorcraft' ? 'VectorCraft' : 'EffectCraft'} · overlay ${a.overlay + 1} → ${base(a.path)}${a.cached ? ' (dùng lại)' : ''}`;
    c.log.push({ ts: ev.ts, kind: a.craft, text: stepTxt });
    c.lastStep = stepTxt;
    A.flashClip = 'ov:' + a.overlay;
    if (A.on && A.stage === key) showAsset(c, a.overlay);
    level = 'full';
  } else if (ev.type === 'result') {
    Object.assign(c, { state: 'done', ended: ev.ts, report: ev.report || null, out: ev.summary || null });
    const r = c.report;
    const notes = r && r.draft ? [`draft ${fmtD(r.draft.duration_sec)}`, r.preview ? 'MP4 FilmCraft' : null, (r.warnings || []).length ? r.warnings.length + ' cảnh báo' : null].filter(Boolean) : [];
    const doneTxt = '■ Hoàn tất' + (notes.length ? ' · ' + notes.join(' · ') : '');
    c.log.push({ ts: ev.ts, kind: 'ok', text: doneTxt });
    c.lastStep = doneTxt;
    setTimeout(() => { if (A.flashClip) { A.flashClip = null; scheduleRender(); } }, 2500);
    refreshSpecs(); loadDrafts();
    if (r && r.preview && A.on && A.stage === key) { A.preview = true; stop(); } // play the FilmCraft cut as soon as it lands
    if (c.build && A.ready && !(A.on && A.stage === key)) toast(`${c.client || 'Agent'} đã dựng xong “${nameOf(c)}”.`, 'agent', { label: 'Xem', onclick: () => enterAgent(key) });
    level = 'full';
  } else if (ev.type === 'error') {
    Object.assign(c, { state: 'error', ended: ev.ts, error: ev.error || 'lỗi' });
    c.log.push({ ts: ev.ts, kind: 'err', text: '✗ ' + c.error });
    c.lastStep = '✗ Lỗi: ' + c.error;
    level = 'full';
  }
  if (!A.ready) return; // kept events: drawn once at 'ready'
  if (ev.type === 'call_start' && c.build) follow(c);
  agentRender(level);
}
function scrollTimelineTo(sec) {
  const sc = $('#tlscroll'); if (!sc) return;
  const x = sec * S.zoom;
  if (x < sc.scrollLeft + 40 || x > sc.scrollLeft + sc.clientWidth - 120) {
    sc.scrollLeft = Math.max(0, x - sc.clientWidth * 0.35);
  }
}
/** The agent's spec in the shape the UI edits (v1 microsecond fields → seconds, studio defaults). */
function agentSpec(raw) {
  const sp = Object.assign(newSpec(raw.project_name || 'Agent'), clone(raw));
  sp.studio = Object.assign({ accent: '#E8B04B', font: 'Segoe UI' }, raw.studio || {});
  const us = o => { if (o && !(o.duration_sec > 0) && o.duration_us > 0) o.duration_sec = o.duration_us / 1e6; };
  for (const x of sp.sections) { us(x.voice); x.scenes.forEach(us); }
  [...sp.scenes, ...sp.voiceovers, ...sp.bgm].forEach(us);
  return sp;
}
function callState(c) {
  if (!c || c.state !== 'running') return c ? c.state : 'done';
  return Date.now() - (A.seen[c.session] || 0) > LOST_MS ? 'lost' : 'running';
}
const nameOf = c => c.spec ? c.spec.project_name : ((c.summary.match(/^"(.*?)"/) || [])[1] || c.tool);
const dotOf = st => ({ running: 'run', done: 'on', error: 'off' }[st] || '');
const previewOf = c => c && c.report && c.report.preview ? c.report.preview.mp4 : null;
function clockText(c) {
  const st = callState(c), s = ms => fmtD(Math.max(0, ms) / 1000);
  if (st === 'running') return 'Đang chạy · ' + s(Date.now() - c.started);
  if (st === 'lost') return 'Mất liên lạc với agent · im lặng ' + s(Date.now() - (A.seen[c.session] || c.started));
  return (st === 'done' ? 'Xong sau ' : 'Lỗi sau ') + s((c.ended || c.started) - c.started);
}
function setFollow(v) { A.follow = v; try { localStorage.setItem('ccds.followAgent', v ? '1' : '0'); } catch {} }
/** A build just started: watch it, unless the user is in the middle of something (then a toast). */
function follow(c) {
  if (A.on && A.stage === c.key) return;
  const busy = !!$('#modal-root').children.length;
  if (A.on || (A.follow && !busy)) enterAgent(c.key);
  else toast(`${c.client || 'Agent'} đang dựng ${c.summary}`, 'agent', { label: 'Xem trực tiếp', onclick: () => enterAgent(c.key) });
}
let arq = null, arLevel = null;
/** 'chip' = top bar + rail, 'log' = + agent panel and inspector, 'full' = everything. */
function agentRender(level) {
  const rank = { chip: 0, log: 1, full: 2 };
  if (!arLevel || rank[level] > rank[arLevel]) arLevel = level;
  if (arq) return;
  // a frame, or a timer where frames are suspended (window hidden behind others), whichever comes first
  const run = () => {
    if (!arq) return;
    cancelAnimationFrame(arq.raf); clearTimeout(arq.to); arq = null;
    const l = arLevel; arLevel = null;
    if (!S.spec) return;
    A.sig = A.calls.map(callState).join();
    if (A.on && l === 'full') return render();
    renderTop(); renderRail(); stageExtras();
    if (A.on) { renderAgentPanel(); renderAgentInsp(); }
  };
  arq = { raf: requestAnimationFrame(run), to: setTimeout(run, 180) };
}
setInterval(() => { // elapsed clocks; a call going quiet turns 'lost'
  if (!A.ready || !A.calls.length) return;
  if (A.calls.map(callState).join() !== A.sig) return agentRender(A.on ? 'full' : 'chip');
  const ck = $('#agentclock'), c = A.byKey[A.sel];
  if (ck && c) ck.textContent = clockText(c);
}, 1000);

async function enterAgent(key) {
  const c = key ? A.byKey[key] : null;
  if (!A.on) {
    stop(); clearTimeout(saveTimer);
    if (!S.saved) await save();
    A.home = { spec: S.spec, slug: S.slug, sel: S.sel, t: S.t, tab: S.tab, saved: S.saved };
    A.on = true; A.stage = undefined; S.tab = 'agent';
  }
  A.sel = (c && c.key) || A.sel || (A.calls[0] || {}).key || null;
  const b = c && c.build ? c : (A.byKey[A.stage] || A.calls.find(x => x.build) || null);
  if (b && b.spec) {
    const slug = slugify(b.spec.project_name);
    A.home = { spec: clone(b.spec), slug, sel: null, t: 0, tab: 'media', saved: true };
  }
  if (!b || b.key !== A.stage) { A.preview = false; showOnStage(b); } else render();
}
/** Put a build call's spec on the stage and timeline. */
function showOnStage(b) {
  stop(); A.stage = b ? b.key : null; A.userSeek = false;
  S.spec = b && b.spec ? b.spec : (A.home && A.home.spec ? A.home.spec : agentSpec({ project_name: b ? nameOf(b) : 'Agent' }));
  S.t = 0; S.sel = null; S.slug = b && b.spec ? slugify(b.spec.project_name) : null; S.saved = true;
  resetStage(); probeAll(); render();
}
/** A craft file landed: unless the user is scrubbing, park the playhead on that overlay. */
function showAsset(c, i) {
  const ov = compute().overlays.find(x => x.i === i);
  if (!ov || S.playing || A.userSeek) return;
  S.sel = { type: 'overlay', i }; A.autoSeek = true; seek(ov.start + Math.min(ov.dur * 0.6, 2)); A.autoSeek = false;
  scrollTimelineTo(ov.start);
}
function exitAgent(tab) {
  if (!A.on) return;
  stop(); const hm = A.home || {};
  const b = A.byKey[A.stage];
  const latestSpec = (b && b.spec) ? clone(b.spec) : (hm.spec || newSpec());
  const latestSlug = (b && b.spec) ? slugify(b.spec.project_name) : (hm.slug || null);
  Object.assign(A, { on: false, home: null, stage: null, preview: false, flashClip: null });
  S.spec = latestSpec; S.slug = latestSlug; S.sel = hm.sel || null; S.t = hm.t || 0; S.saved = true;
  S.tab = tab || (hm.tab && hm.tab !== 'agent' ? hm.tab : 'media');
  if (S.tab === 'capcut') loadDrafts();
  resetStage(); probeAll(); ensureGraphics(); render();
}
/** Save the agent's spec as a studio project and open it for editing right away. */
async function adoptAgent() {
  const b = A.byKey[A.stage]; if (!b || !b.spec) return exitAgent('media');
  const sp = clone(b.spec), slug = slugify(sp.project_name);
  try { await api.put('/api/specs/' + slug, sp); } catch (e) { return toast('Không lưu được dự án: ' + e.message, 'err'); }
  stop(); Object.assign(A, { on: false, home: null, stage: null, preview: false, flashClip: null }); resetStage();
  await refreshSpecs(); S.tab = 'media'; await openSpec(slug);
  toast(`Đang mở dự án “${sp.project_name}” trực tiếp trên Timeline để sửa tiếp.`, 'ok');
}
function resetStage() { stageEls = null; const s = $('#stage'); if (s) s.replaceChildren(); }

function agentChip() {
  if (!A.calls.length) return null;
  const run = A.calls.find(c => callState(c) === 'running'), c = run || A.calls[0], st = callState(c);
  const what = run ? (run.build ? nameOf(run) : run.tool) : { done: 'xong', error: 'lỗi', lost: 'mất liên lạc' }[st];
  return h('button', { class: 'chip agent' + (run ? ' run' : ''), title: `${c.client || 'Agent'} · ${c.tool} ${c.summary}\nBấm để xem`,
    onclick: () => enterAgent((run && run.build ? run : A.calls.find(x => x.build) || c).key) },
    h('span', { class: 'dot ' + dotOf(st) }), run ? (run.build ? 'Agent đang dựng' : 'Agent đang chạy') : 'Agent', what ? h('b', {}, what) : null);
}
/** Where an overlay's picture comes from in agent mode: the craft files the build has made so far. */
function agentOvSrc(o, i) {
  const c = A.byKey[A.stage], made = (c && c.assets[i]) || [];
  const vid = made.find(a => a.kind === 'motion'), png = made.find(a => a.kind === 'graphic');
  if (vid && /\.webm$/i.test(vid.path)) return { src: vid.path, kind: 'video' };
  if (png) return { src: png.path, kind: 'img' }; // a ProRes .mov does not play in the browser: show its still
  if (o.path && (o.kind === 'file' || !o.graphic)) return { src: o.path, kind: o.kind === 'file' && kindOf(o.path) === 'video' ? 'video' : 'img' };
  const running = callState(c) === 'running';
  return { ph: (o.graphic && (tpl(o.graphic.template) || {}).name) || 'Overlay', wait: running,
    phSub: running ? (o.kind === 'motion' ? 'VectorCraft → EffectCraft đang làm…' : 'VectorCraft đang vẽ…') : 'Không có file' };
}

function renderAgentPanel() {
  const el = $('#panel'), c = A.byKey[A.sel];
  let t = A.term && A.term.key === A.sel ? A.term : null;
  const atBottom = !t || t.el.scrollHeight - t.el.scrollTop - t.el.clientHeight < 40, keep = t ? t.el.scrollTop : 0;
  if (!t) t = A.term = { key: A.sel, el: h('div', { class: 'term', role: 'log', 'aria-live': 'polite' }), n: 0 };
  const cur = t.el.querySelector('.cursor'); if (cur) cur.remove();
  for (const l of (c ? c.log : []).slice(t.n)) t.el.append(h('div', { class: l.kind }, h('i', {}, '+' + ((l.ts - c.started) / 1000).toFixed(1)), l.text));
  t.n = c ? c.log.length : 0;
  if (c && callState(c) === 'running') t.el.append(h('div', { class: 'cursor' }, h('i')));
  if (!c) t.el.replaceChildren(h('div', { class: 'spec' }, h('i'), 'Chưa có lệnh nào. Khi Windows Agent (Antigravity, Claude, Cursor…) gọi Draft Studio qua MCP hoặc CLI, từng bước dựng và kết quả sẽ hiện trực tiếp trên màn hình.'));
  const clients = [...new Set(A.calls.map(x => x.client).filter(Boolean))];
  put(el, phead('Live Agent', `${clients.join(', ') || 'MCP / CLI'} · ${A.calls.length} bước`,
      h('button', { class: 'btn sm primary', onclick: openMcpModal }, '🔌 Kết nối MCP')),
    h('div', { class: 'pbody agentbody' },
      h('label', { class: 'chk small' }, h('input', { type: 'checkbox', checked: A.follow, onchange: e => setFollow(e.target.checked) }), 'Tự động hiển thị trực tiếp khi Agent dựng'),
      A.calls.length ? h('div', { class: 'acalls' }, ...A.calls.map(x => { const st = callState(x);
        return h('button', { class: 'acall' + (x.key === A.sel ? ' on' : ''), title: x.summary, onclick: () => x.build ? enterAgent(x.key) : (A.sel = x.key, agentRender('log')) },
          h('span', { class: 'dot ' + dotOf(st) }),
          h('div', { style: { minWidth: 0 } }, h('div', { class: 'ellipsis mono small' }, x.tool), h('div', { class: 'ellipsis small muted' }, x.build ? nameOf(x) : x.summary || '—')),
          h('span', { class: 'small dim' }, new Date(x.started).toLocaleTimeString('vi-VN', { hour12: false }))); })) : null,
      h('span', { class: 'eyebrow' }, 'Nhật ký từng bước trực tiếp', c ? ' · ' + c.tool : ''), t.el));
  t.el.scrollTop = atBottom ? t.el.scrollHeight : keep;
}

function renderAgentInsp() {
  const el = $('#insp'), c = A.byKey[A.sel];
  const back = h('button', { class: 'btn primary block', onclick: () => exitAgent('media') }, icon('wrench', 'sm'), 'Chỉnh sửa trực tiếp trên Timeline');
  if (!c) return put(el, h('div', { class: 'phead' }, h('div', {}, h('h2', {}, 'Live Agent'), h('p', {}, 'Sẵn sàng nhận lệnh từ Agent'))),
    h('div', { class: 'pbody' },
      mcpSection(),
      h('div', { class: 'empty' }, icon('agent'), 'Khi Agent gọi capcut_live_edit hoặc capcut_build_project, từng cảnh, hiệu ứng chuyển cảnh, LUT màu, đồ họa VectorCraft và chuyển động EffectCraft sẽ biến đổi trực tiếp trên màn hình ngay trước mắt bạn.'), back));
  const st = callState(c), r = c.report, sp = c.build && c.spec;
  const color = { running: 'var(--accent)', done: 'var(--ok)', error: 'var(--err)' }[st] || 'var(--dim)';
  const tl = sp && A.stage === c.key ? compute() : null;
  const crafts = sp ? sp.overlays.map((o, i) => ({ o, i })).filter(({ o }) => o.kind !== 'file' && (o.graphic || o.kind === 'motion')) : [];
  const mp4 = previewOf(c);
  put(el, h('div', { class: 'phead' }, h('div', { class: 'ititle' }, h('span', { class: 'sw', style: { background: color } }),
      h('div', { style: { minWidth: 0 } }, h('h2', { class: 'ellipsis' }, c.build ? nameOf(c) : c.tool), h('p', {}, `${c.client || 'Agent'} · ${c.tool}`)))),
    h('div', { class: 'pbody' },
      h('div', { class: 'row small' }, st === 'running' ? h('span', { class: 'spinner', style: { width: '14px', height: '14px' } }) : h('span', { class: 'dot ' + dotOf(st) }), h('span', { id: 'agentclock' }, clockText(c))),
      tl ? h('div', { class: 'stat' },
        h('div', {}, h('b', {}, fmtD(tl.total)), h('span', {}, 'thời lượng')), h('div', {}, h('b', {}, tl.scenes.length), h('span', {}, sp.sections.length ? `cảnh · ${sp.sections.length} đoạn` : 'cảnh')),
        h('div', {}, h('b', {}, sp.overlays.length), h('span', {}, 'overlay')), h('div', {}, h('b', {}, sp.sections.length + sp.voiceovers.length + sp.bgm.length), h('span', {}, 'file âm thanh'))) : null,
      c.build && !sp && st === 'running' ? h('div', { class: 'note' }, 'Đang chờ spec từ agent…') : null,
      crafts.length ? sec('Craft · ' + crafts.filter(({ i }) => (c.assets[i] || []).length).length + '/' + crafts.length + ' overlay đã có file', h('div', { class: 'col', style: { gap: '6px' } }, ...crafts.map(({ o, i }) => {
        const made = c.assets[i] || [], png = made.find(a => a.kind === 'graphic'), vid = made.find(a => a.kind === 'motion');
        const needV = !!o.graphic, needE = o.kind === 'motion', done = (!needV || png) && (!needE || vid), cached = made.length && made.every(a => a.cached);
        return h('button', { class: 'crow', title: made.map(a => a.path).join('\n') || overlayName(o), onclick: () => { const ov = tl && tl.overlays.find(x => x.i === i); if (ov) { S.sel = { type: 'overlay', i }; seek(ov.start + Math.min(ov.dur * 0.6, 2)); renderTimeline(); } } },
          h('div', { class: 'pv checker' }, png ? h('img', { src: fileUrl(png.path), alt: '' }) : null),
          h('div', { style: { minWidth: 0 } }, h('div', { class: 'ellipsis' }, `${i + 1}. ${overlayName(o)}`),
            h('div', { class: 'row', style: { gap: '4px', marginTop: '3px' } }, needV ? h('span', { class: 'tag vector' }, 'VectorCraft') : null, needE ? h('span', { class: 'tag effect' }, 'EffectCraft') : null)),
          done ? h('span', { class: 'small', style: { color: 'var(--ok)' } }, cached ? '✓ dùng lại' : '✓ mới')
            : st === 'running' ? h('span', { class: 'spinner', style: { width: '14px', height: '14px' } }) : h('span', { class: 'small dim' }, '—'));
      }))) : null,
      r && r.draft ? h('div', { class: 'bigok' }, h('div', { class: 'ck' }, icon('check')), h('div', { class: 'grow' }, h('b', {}, `Draft “${r.project_name}” · ${fmtD(r.draft.duration_sec)}`),
        h('div', { class: 'small muted mono ellipsis', title: r.draft.draft_dir }, r.draft.draft_dir))) : null,
      r && r.capcut_running ? h('div', { class: 'note warn' }, h('b', {}, 'CapCut đang mở. '), 'Tắt hẳn CapCut rồi mở lại để thấy draft ở đầu tab Projects.') : null,
      ...(r ? r.warnings || [] : []).map(w => h('div', { class: 'note warn' }, w)),
      r && r.draft ? h('div', { class: 'row wrap' },
        h('button', { class: 'btn sm', onclick: () => api.post('/api/open', { what: 'folder', path: r.draft.draft_dir }).catch(e => toast(e.message, 'err')) }, icon('folder', 'sm'), 'Mở thư mục draft'),
        h('button', { class: 'btn sm', onclick: () => api.post('/api/open', { what: 'capcut' }).then(() => toast('Đang mở CapCut…', 'ok')).catch(e => toast(e.message, 'err')) }, icon('capcut', 'sm'), 'Mở CapCut'),
        mp4 && A.stage === c.key ? h('button', { class: 'btn sm', onclick: () => { A.preview = !A.preview; stop(); stageExtras(); renderAgentInsp(); } }, icon(A.preview ? 'film' : 'play', 'sm'), A.preview ? 'Xem bản dựng' : 'Xem MP4 FilmCraft') : null) : null,
      c.error ? h('div', { class: 'note err' }, h('b', {}, 'Agent gặp lỗi. '), h('div', { style: { whiteSpace: 'pre-wrap', marginTop: '6px' } }, c.error)) : null,
      !c.build && c.args && Object.keys(c.args).length ? sec('Tham số', h('pre', { class: 'code' }, JSON.stringify(c.args, null, 2))) : null,
      c.out ? sec('Kết quả', h('pre', { class: 'code' }, c.out)) : null,
      h('div', { class: 'grow' }),
      back));
}
/** Stage bar note, Live Agent HUD, and the FilmCraft MP4 over the viewer in agent mode. */
function stageExtras() {
  const tools = $('#stagetools'), viewer = $('#viewer'); if (!tools || !viewer) return;
  const c = A.byKey[A.stage] || A.calls[0] || null;
  const mp4 = A.on ? previewOf(c) : null, show = !!(mp4 && A.preview);
  put(tools, mp4 ? h('div', { class: 'seg' },
      h('button', { class: show ? '' : 'on', onclick: () => { A.preview = false; stageExtras(); renderAgentInsp(); } }, 'Bản dựng Timeline'),
      h('button', { class: show ? 'on' : '', onclick: () => { A.preview = true; stop(); stageExtras(); renderAgentInsp(); } }, '🎬 MP4 FilmCraft'))
    : h('span', { class: 'small dim' }, A.on ? '⚡ Đang đồng bộ trực tiếp từng bước từ Agent' : 'Ảnh tĩnh + overlay theo vị trí CapCut · motion hiện sau khi render xem trước'));
  $('#stage').classList.toggle('mp4', show);
  let v = $('#mp4view');
  if (show && (!v || v.dataset.src !== mp4)) {
    if (v) v.remove();
    v = h('div', { class: 'mp4view', id: 'mp4view' }, h('video', { src: fileUrl(mp4), controls: true, autoplay: true, muted: true, playsInline: true }));
    v.dataset.src = mp4; viewer.append(v);
  } else if (!show && v) v.remove();

  // Floating Live Agent Step HUD (like watching live code edits in an IDE)
  let hud = $('#agent-hud');
  const st = c ? callState(c) : null;
  const showHud = c && (A.on || st === 'running' || (c.ended && Date.now() - c.ended < 8000));
  if (showHud) {
    if (!hud) { hud = h('div', { id: 'agent-hud', class: 'agent-hud' }); viewer.append(hud); }
    hud.className = 'agent-hud' + (st === 'done' ? ' done' : '');
    put(hud,
      st === 'running' ? h('span', { class: 'spinner', style: { width: '14px', height: '14px', flexShrink: 0 } }) : h('span', { class: 'dot ' + dotOf(st) }),
      h('b', { style: { flexShrink: 0, color: 'var(--accent2)' } }, c.client || 'Agent'),
      h('div', { class: 'hud-step', title: c.lastStep || c.summary }, c.lastStep || c.summary || c.tool),
      h('span', { class: 'hud-badge' }, `${c.log.length} bước`),
      A.on ? h('button', { class: 'btn sm primary', onclick: () => exitAgent('media') }, 'Sửa trực tiếp') : h('button', { class: 'btn sm', onclick: () => enterAgent(c.key) }, 'Xem chi tiết'));
  } else if (hud) {
    hud.remove();
  }
}

// ---------------------------------------------------------------- keys + boot
document.addEventListener('keydown', e => {
  if ($('#modal-root').children.length) return;
  const tag = (document.activeElement || {}).tagName;
  if (['INPUT', 'TEXTAREA', 'SELECT'].includes(tag)) return;
  if (e.code === 'Space') { e.preventDefault(); togglePlay(); }
  else if (e.key === 'ArrowLeft') { seek(S.t - (e.shiftKey ? 5 : 0.5)); }
  else if (e.key === 'ArrowRight') { seek(S.t + (e.shiftKey ? 5 : 0.5)); }
  else if (e.key === 'Delete' || e.key === 'Backspace') { if (S.sel) { e.preventDefault(); removeSel(); } }
  else if (e.key === 'Escape') { S.sel = null; render(); }
  else if ((e.ctrlKey || e.metaKey) && e.key === 's') { e.preventDefault(); save(); }
});
addEventListener('resize', () => fitScreen());

// ---------------------------------------------------------------- access key
// The server answers only /api/ping and /api/license until this machine has a key (license.rs).
/** Full-screen key prompt; resolves with the new status once a key is accepted. */
function askKey(lic) {
  return new Promise(done => {
    const input = h('input', { class: 'keyin', placeholder: 'DS-XXXX-XXXX-XXXX-XXXX', autocomplete: 'off', spellcheck: false, 'aria-label': 'Access key' });
    const err = h('div', { class: 'note err hidden', role: 'alert' });
    const btn = h('button', { class: 'btn primary block', type: 'submit' }, 'Kích hoạt');
    const gate = h('div', { class: 'gate' }, h('form', { class: 'gatebox', onsubmit: async e => {
        e.preventDefault(); btn.disabled = true;
        try { const st = await api.post('/api/license', { key: input.value }); gate.remove(); done(st); }
        catch (x) { err.textContent = x.message; err.classList.remove('hidden'); btn.disabled = false; input.select(); }
      } },
      h('img', { src: 'icon.png', alt: '' }), h('h1', {}, 'Draft Studio'),
      h('p', {}, 'Nhập access key để dùng Draft Studio. Mỗi thành viên nhóm Zalo được gửi riêng một key.'),
      lic.invalid_saved ? h('div', { class: 'note warn' }, 'Key đã lưu trên máy này không còn được chấp nhận. Nhập key khác.') : null,
      input, err, btn,
      h('p', { class: 'small dim' }, 'Kích hoạt một lần cho máy này: app, dòng lệnh (capcut_draft_app) và agent qua MCP đều dùng được.')));
    document.body.append(gate); input.focus();
  });
}
function licenseSection() {
  const l = S.license || {};
  return h('div', { class: 'section' }, h('span', { class: 'eyebrow' }, 'Phiên bản'),
    h('div', { class: 'row small' }, h('span', { class: 'dot on' }), 'Bản mở khóa vĩnh viễn (Unlocked Pro · Không cần key)', l.key ? h('span', { class: 'mono muted' }, l.key) : null));
}

async function boot() {
  let lic = null; try { lic = await api.get('/api/license'); } catch {}
  if (lic && !lic.activated) lic = await askKey(lic);
  S.license = lic || {};
  buildShell();
  const [st, cat, se, mcp] = await Promise.all([
    api.get('/api/status').catch(() => null),
    api.get('/api/catalog').catch(() => ({ graphics: [], motion: [], transitions: [], luts: [], video_effects: [] })),
    api.get('/api/settings').catch(() => ({})),
    api.get('/api/mcp-clients').catch(() => ({ mcp_exe: '', clients: [] })),
  ]);
  S.status = st; S.catalog = cat; S.settings = se || {}; S.mcp = mcp || { mcp_exe: '', clients: [] };
  await refreshSpecs();
  let slug = null; try { slug = localStorage.getItem('ccds.slug'); } catch {}
  if (slug && S.specs.some(p => p.slug === slug)) await openSpec(slug);
  else if (S.specs.length) await openSpec(S.specs[0].slug);
  else { S.spec = newSpec(); render(); }
  loadDrafts();
  connectActivity();
}
boot();
