define([], function(m) {
// This file has been automatically generated.

var exports = {};

var val = 0;
exports.FLOOR_GREY_DIRT = val++;
val = exports.FLOOR_NORMAL = exports.FLOOR_GREY_DIRT; val++;
exports.FLOOR_PEBBLE = val++;
val = exports.FLOOR_PEBBLE_LIGHTGRAY = exports.FLOOR_PEBBLE; val++;
exports.FLOOR_PEBBLE_BROWN = val++;
exports.FLOOR_PEBBLE_BLUE = val++;
exports.FLOOR_PEBBLE_GREEN = val++;
exports.FLOOR_PEBBLE_RED = val++;
exports.FLOOR_PEBBLE_MAGENTA = val++;
exports.FLOOR_PEBBLE_YELLOW = val++;
exports.FLOOR_PEBBLE_WHITE = val++;
exports.FLOOR_PEBBLE_DARKBROWN = val++;
exports.FLOOR_HALL = val++;
exports.FLOOR_MUD = val++;
exports.FLOOR_ICE = val++;
exports.FLOOR_LAIR = val++;
exports.FLOOR_MOSS = val++;
exports.FLOOR_SLIME = val++;
exports.FLOOR_SLIME_ACIDIC = val++;
exports.FLOOR_ICY = val++;
exports.FLOOR_SALT = val++;
exports.FLOOR_SALT_1 = val++;
exports.FLOOR_SAND = val++;
exports.FLOOR_SANDSTONE = val++;
exports.FLOOR_VOLCANIC = val++;
exports.FLOOR_CRYSTAL_SQUARES = val++;
exports.FLOOR_GRASS = val++;
exports.HALO_GRASS = val++;
exports.HALO_GRASS_1 = val++;
exports.HALO_GRASS_2 = val++;
exports.HALO_GRASS_3 = val++;
exports.HALO_GRASS_4 = val++;
exports.HALO_GRASS_5 = val++;
exports.HALO_GRASS_6 = val++;
exports.HALO_GRASS_7 = val++;
exports.HALO_GRASS_8 = val++;
exports.FLOOR_NERVES = val++;
val = exports.FLOOR_NERVES_RED = exports.FLOOR_NERVES; val++;
exports.FLOOR_NERVES_LIGHTGRAY = val++;
exports.FLOOR_LIMESTONE = val++;
exports.FLOOR_W_MARBLE = val++;
exports.SIGIL_CURVE_N_E = val++;
exports.SIGIL_CURVE_N_W = val++;
exports.SIGIL_CURVE_S_E = val++;
exports.SIGIL_CURVE_S_W = val++;
exports.SIGIL_STRAIGHT_E_W = val++;
exports.SIGIL_STRAIGHT_N_S = val++;
exports.SIGIL_STRAIGHT_NE_SW = val++;
exports.SIGIL_STRAIGHT_NW_SE = val++;
exports.SIGIL_CROSS = val++;
exports.SIGIL_CIRCLE = val++;
exports.SIGIL_RHOMBUS = val++;
exports.SIGIL_Y = val++;
exports.SIGIL_Y_INVERTED = val++;
exports.SIGIL_Y_RIGHT = val++;
exports.SIGIL_Y_LEFT = val++;
exports.SIGIL_Y_LEFTLEANING = val++;
exports.SIGIL_Y_RIGHTLEANING = val++;
exports.SIGIL_ALGIZ_LEFT = val++;
exports.SIGIL_ALGIZ_RIGHT = val++;
exports.SIGIL_STRAIGHT_E_NW = val++;
exports.SIGIL_STRAIGHT_E_SW = val++;
exports.SIGIL_STRAIGHT_W_NE = val++;
exports.SIGIL_STRAIGHT_W_SE = val++;
exports.SIGIL_STRAIGHT_N_SE = val++;
exports.SIGIL_STRAIGHT_N_SW = val++;
exports.SIGIL_STRAIGHT_S_NE = val++;
exports.SIGIL_STRAIGHT_S_NW = val++;
exports.SIGIL_FOURWAY = val++;
exports.SIGIL_SHARP_E_NE = val++;
exports.SIGIL_SHARP_W_SW = val++;
exports.SIGIL_STRAIGHT_E_NE_SW = val++;
exports.FLOOR_GREEN_BONES = val++;
exports.FLOOR_WOODGROUND = val++;
exports.FLOOR_FROZEN = val++;
exports.FLOOR_STUDIO = val++;
exports.DNGN_ENDLESS_SALT = val++;
exports.DNGN_LAVA_SEA = val++;
exports.DNGN_OPEN_SEA = val++;
exports.DNGN_SHALLOW_WATER = val++;
exports.DNGN_SHALLOW_WATER_MURKY = val++;
exports.SHOALS_DEEP_WATER = val++;
exports.SHOALS_SHALLOW_WATER = val++;
exports.VTTFLOOR_MAX = exports.TILE_VTTFLOOR_MAX = val++;

var tile_info = [
  {w: 32, h: 32, ox: 0, oy: 0, sx: 0, sy: 0, ex: 32, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 32, sy: 0, ex: 64, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 64, sy: 0, ex: 96, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 96, sy: 0, ex: 128, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 128, sy: 0, ex: 160, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 160, sy: 0, ex: 192, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 192, sy: 0, ex: 224, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 224, sy: 0, ex: 256, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 256, sy: 0, ex: 288, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 288, sy: 0, ex: 320, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 320, sy: 0, ex: 352, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 352, sy: 0, ex: 384, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 384, sy: 0, ex: 416, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 416, sy: 0, ex: 448, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 448, sy: 0, ex: 480, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 480, sy: 0, ex: 512, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 512, sy: 0, ex: 544, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 544, sy: 0, ex: 576, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 576, sy: 0, ex: 608, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 608, sy: 0, ex: 640, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 640, sy: 0, ex: 672, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 672, sy: 0, ex: 704, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 704, sy: 0, ex: 736, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 736, sy: 0, ex: 768, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 768, sy: 0, ex: 800, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 800, sy: 0, ex: 832, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 832, sy: 0, ex: 864, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 864, sy: 0, ex: 896, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 896, sy: 0, ex: 928, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 928, sy: 0, ex: 960, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 960, sy: 0, ex: 992, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 992, sy: 0, ex: 1024, ey: 32},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 0, sy: 32, ex: 32, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 32, sy: 32, ex: 64, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 64, sy: 32, ex: 96, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 96, sy: 32, ex: 128, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 128, sy: 32, ex: 160, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 160, sy: 32, ex: 192, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 192, sy: 32, ex: 224, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 224, sy: 32, ex: 256, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 256, sy: 32, ex: 288, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 288, sy: 32, ex: 320, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 320, sy: 32, ex: 352, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 352, sy: 32, ex: 384, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 384, sy: 32, ex: 416, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 416, sy: 32, ex: 448, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 448, sy: 32, ex: 480, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 480, sy: 32, ex: 512, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 512, sy: 32, ex: 544, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 544, sy: 32, ex: 576, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 576, sy: 32, ex: 608, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 608, sy: 32, ex: 640, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 640, sy: 32, ex: 672, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 672, sy: 32, ex: 704, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 704, sy: 32, ex: 736, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 736, sy: 32, ex: 768, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 768, sy: 32, ex: 800, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 800, sy: 32, ex: 832, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 832, sy: 32, ex: 864, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 864, sy: 32, ex: 896, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 896, sy: 32, ex: 928, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 928, sy: 32, ex: 960, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 960, sy: 32, ex: 992, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 992, sy: 32, ex: 1024, ey: 64},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 0, sy: 64, ex: 32, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 32, sy: 64, ex: 64, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 64, sy: 64, ex: 96, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 96, sy: 64, ex: 128, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 128, sy: 64, ex: 160, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 160, sy: 64, ex: 192, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 192, sy: 64, ex: 224, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 224, sy: 64, ex: 256, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 256, sy: 64, ex: 288, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 288, sy: 64, ex: 320, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 320, sy: 64, ex: 352, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 352, sy: 64, ex: 384, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 384, sy: 64, ex: 416, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 416, sy: 64, ex: 448, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 448, sy: 64, ex: 480, ey: 96},
  {w: 32, h: 32, ox: 0, oy: 0, sx: 480, sy: 64, ex: 512, ey: 96},
];

exports.get_tile_info = function (idx)
{
    return tile_info[idx - 0];
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
    1,
    2,
    1,
    1,
    1,
    1,
    1,
    1,
    9,
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
    1,
    1,
    1,
];

exports.tile_count = function (idx)
{
    return _tile_count[idx - 0];
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
    17,
    18,
    18,
    20,
    21,
    22,
    23,
    24,
    25,
    25,
    25,
    25,
    25,
    25,
    25,
    25,
    25,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41,
    42,
    43,
    44,
    45,
    46,
    47,
    48,
    49,
    50,
    51,
    52,
    53,
    54,
    55,
    56,
    57,
    58,
    59,
    60,
    61,
    62,
    63,
    64,
    65,
    66,
    67,
    68,
    69,
    70,
    71,
    72,
    73,
    74,
    75,
    76,
    77,
    78,
    79,
];

exports.basetile = function (idx)
{
    return _basetiles[idx - 0] + 0;
};

exports.get_img = function (idx) {
    return "vttfloor";
};

return exports;
});
