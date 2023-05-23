define(["./tileinfo-gui"], function(m) {
// This file has been automatically generated.

var exports = {};

var val = m.TILEG_GUI_MAX;
exports.CURSOR = val++;
exports.CURSOR2 = val++;
exports.CURSOR3 = val++;
exports.TUTORIAL_CURSOR = val++;
exports.DISABLED = val++;
exports.TRAVEL_EXCLUSION_FG = val++;
exports.TRAVEL_EXCLUSION_CENTRE_FG = val++;
exports.NUM0 = val++;
exports.NUM1 = val++;
exports.NUM2 = val++;
exports.NUM3 = val++;
exports.NUM4 = val++;
exports.NUM5 = val++;
exports.NUM6 = val++;
exports.NUM7 = val++;
exports.NUM8 = val++;
exports.NUM9 = val++;
exports.VTTICON_MAX = exports.TILEI_VTTICON_MAX = val++;

var tile_info = [
  {w: 32, h: 32, ox: 0, oy: 0, sx: 0, sy: 0, ex: 32, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 32, sy: 0, ex: 64, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 64, sy: 0, ex: 96, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 96, sy: 0, ex: 128, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 128, sy: 0, ex: 160, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 160, sy: 0, ex: 192, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 192, sy: 0, ex: 224, ey: 32},
  {w: 8, h: 14, ox: 0, oy: 2, sx: 224, sy: 0, ex: 232, ey: 9},
  {w: 6, h: 14, ox: 0, oy: 2, sx: 224, sy: 9, ex: 230, ey: 18},
  {w: 8, h: 14, ox: 0, oy: 2, sx: 224, sy: 18, ex: 232, ey: 27},
  {w: 8, h: 14, ox: 0, oy: 2, sx: 232, sy: 0, ex: 240, ey: 10},
  {w: 8, h: 14, ox: 0, oy: 1, sx: 232, sy: 10, ex: 240, ey: 21},
  {w: 8, h: 14, ox: 0, oy: 2, sx: 232, sy: 21, ex: 240, ey: 31},
  {w: 8, h: 14, ox: 0, oy: 1, sx: 240, sy: 0, ex: 248, ey: 10},
  {w: 8, h: 14, ox: 0, oy: 2, sx: 240, sy: 10, ex: 248, ey: 20},
  {w: 8, h: 14, ox: 0, oy: 1, sx: 240, sy: 20, ex: 248, ey: 30},
  {w: 8, h: 14, ox: 0, oy: 2, sx: 248, sy: 0, ex: 256, ey: 10},
];

exports.get_tile_info = function (idx)
{
    return tile_info[idx - m.TILEG_GUI_MAX];
};

var _tile_count =
[
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
];

exports.tile_count = function (idx)
{
    return _tile_count[idx - m.TILEG_GUI_MAX];
}

var _basetiles =
[
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
];

exports.basetile = function (idx)
{
    return _basetiles[idx - m.TILEG_GUI_MAX] + m.TILEG_GUI_MAX;
};

exports.get_img = function (idx) {
    return "vtticon";
};

return exports;
});
