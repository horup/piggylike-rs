// atlases
const TILES_ATLAS = 0;
define_atlas(TILES_ATLAS, 16, 16, "assets/textures/tiles.png");

const THINGS_ATLAS = 1;
define_atlas(THINGS_ATLAS, 16, 16, "assets/textures/things.png");


// things
const PLAYER_THING = 0;
define_thing(PLAYER_THING, #{
    atlas:THINGS_ATLAS,
    atlas_index:0,
    player:true
});

const PIGGY_THING = 1;
define_thing(PIGGY_THING, #{
    atlas:THINGS_ATLAS,
    atlas_index:1
});


// floors
const GRASS_TILE = 0;
define_tile(GRASS_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:0,
    solid:false
});

const SAND_TILE = 1;
define_tile(SAND_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:1,
    solid:false
});

const STONE_TILE = 2;
define_tile(STONE_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:2,
    solid:false
});

const WOOD_TILE = 5;
define_tile(WOOD_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:5,
    solid:false
});

// solids
const FOLIAGE_TILE = 16;
define_tile(FOLIAGE_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:16,
    solid:true
});

const ROCK_TILE = 18;
define_tile(ROCK_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:18,
    solid:true
});

const WOODWALL_TILE = 21;
define_tile(WOODWALL_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:21,
    solid:true
});

load_map("assets/maps/test.tmj");