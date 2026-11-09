// pairs-patch.js: the screen of pairs-app, a memory game for the hallway panel: a sidebar on the
// left (the time, moves, pairs found, the round's clock, the joker of the weather now, the
// round's result and New round) and 12 cards in a 4 x 3 grid on the right. The cards are two
// grid slots of the same pitch: #pairs/cards (12 #button, the backs that take the taps) and
// #pairs/faces (12 pictures of the app's own #pairs-weather, weather-pictures.js, over the backs' middles, the faces; the app hides a face's
// clone, so a face down card shows nothing of it). The app (packages/services/display_safety/utils/pairs-app) drives the
// #pairs/* names. Applied with everyware's roundtrip.js to the EveryWare Library .fig, then ui-lib's dark-theme-patch.js for the dark Library:
//
//   design/build.sh     # the Library, this patch, the dark theme, data/pairs.dcf

const SIDE_W = 400, PAD = 48;
// the grid: 4 x 3 cards of the button's size, centred in the space right of the sidebar
const COLS = 4, ROWS = 3, GAP_X = 64, GAP_Y = 72;

// roundtrip.js hands over fig-helpers.js (the node authoring) as the second argument
module.exports = function (msg, { key, C, FONT, authoring }) {
  const A = authoring(msg, 'pairs');
  const { nodes, kids, frame, text, instance, solid, log } = A;
  // the app's own pictures, a set next to the Library's (weather-pictures.js)
  require('./weather-pictures.js')(A, C, '#pairs-weather');
  // the Library's .fig (ui-lib design/build-library.sh): the screen goes on its Page 1
  const page = nodes.find(n => n.type === 'CANVAS' && n.name === 'Page 1');
  if (!page) throw new Error('no Page 1: build on the EveryWare Library .fig');
  if (nodes.some(n => n.name === '#pairs-root')) throw new Error('#pairs-root exists already');
  const variant = (set, name) => {
    const s = nodes.find(n => n.isStateGroup && n.name === set);
    const v = s && kids(key(s.guid)).find(n => n.type === 'SYMBOL' && n.name === name);
    if (!v) throw new Error(`no variant ${set} / ${name}`);
    return v;
  };
  const slot = (parent, name, x, y, v) => {
    const f = frame(parent, name, x, y, v.size.x, v.size.y, {
      bordersTakeSpace: false, stackMode: 'HORIZONTAL', stackPrimaryAlignItems: 'CENTER', stackCounterAlignItems: 'CENTER',
      stackPrimarySizing: 'FIXED', stackCounterSizing: 'FIXED', stackLayoutVersion: 1,
    });
    instance(f, `${name}/preview`, 0, 0, v);
    return f;
  };
  // texts the code writes: cut with an ellipsis after their lines
  const lines = n => ({ maxLines: n, textTruncation: 'ENDING' });

  // its own place on Page 1, away from the other apps' roots
  const root = frame(page, '#pairs-root', -2400, 2800, 1920, 1080, { fillPaints: [solid(C.page)], frameMaskDisabled: false });

  // ---- the sidebar
  const side = frame(root, 'Sidebar', 0, 0, SIDE_W, 1080, { fillPaints: [solid(C.surface)] });
  const W = SIDE_W - 2 * PAD;
  text(side, '#pairs/time', PAD, 40, W, 100, '12:41', 80, FONT.regular, C.ink, 'LEFT', 'TOP', lines(1));
  const stat = (label, name, preview, y) => {
    text(side, label, PAD, y, W, 22, label.toUpperCase(), 16, FONT.semibold, C.ink2);
    text(side, name, PAD, y + 26, W, 56, preview, 44, FONT.medium, C.ink, 'LEFT', 'TOP', lines(1));
  };
  stat('Moves', '#pairs/moves', '7', 200);
  stat('Pairs', '#pairs/found', '2/6', 316);
  stat('Time', '#pairs/clock', '01:23', 432);
  text(side, 'Joker', PAD, 556, W, 22, 'JOKER', 16, FONT.semibold, C.ink2);
  slot(side, '#pairs/joker', PAD, 588, variant('#pairs-weather', 'kind=rain'));
  text(side, '#pairs/joker-text', PAD + 56, 576, W - 56, 64, 'Rain counts double', 20, FONT.regular, C.ink, 'LEFT', 'CENTER', lines(2));
  text(side, '#pairs/status', PAD, 680, W, 120, 'Done in 14 moves, 01:52', 28, FONT.semibold, C.accent, 'LEFT', 'TOP', lines(3));
  slot(side, '#pairs/new-round', PAD, 976, variant('#button', 'kind=primary, state=default, size=regular'));

  // ---- the cards: the back (#button) and the face (#pairs-weather) over its middle
  const back = variant('#button', 'kind=secondary, state=default, size=regular');
  const kinds = ['clear', 'partly', 'cloudy', 'fog', 'rain', 'snow', 'thunder'];
  const gridW = COLS * back.size.x + (COLS - 1) * GAP_X, gridH = ROWS * back.size.y + (ROWS - 1) * GAP_Y;
  const x0 = SIDE_W + Math.round((1920 - SIDE_W - gridW) / 2), y0 = Math.round((1080 - gridH) / 2);
  // the app lays the cells out at this pitch (presenter.rs: PITCH): the same numbers in both
  const face0 = variant('#pairs-weather', 'kind=clear');
  const fx = (back.size.x - face0.size.x) / 2, fy = (back.size.y - face0.size.y) / 2;
  const cards = frame(root, '#pairs/cards', x0, y0, gridW, gridH);
  const faces = frame(root, '#pairs/faces', x0 + fx, y0 + fy, gridW - 2 * fx, gridH - 2 * fy);
  log(`grid pitch ${back.size.x + GAP_X} x ${back.size.y + GAP_Y}`);
  for (let i = 0; i < COLS * ROWS; i++) {
    const x = (i % COLS) * (back.size.x + GAP_X), y = Math.floor(i / COLS) * (back.size.y + GAP_Y);
    instance(cards, `#pairs/cards/preview-${i + 1}`, x, y, back);
    instance(faces, `#pairs/faces/preview-${i + 1}`, x, y, variant('#pairs-weather', `kind=${kinds[i % kinds.length]}`));
  }

  log(`#pairs-root: ${A.count()} nodes`);
  return msg;
};
