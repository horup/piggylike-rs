// atlases
const TILES_ATLAS = 0;
define_atlas(TILES_ATLAS, 16, 16, "assets/textures/tiles.png");

const THINGS_ATLAS = 1;
define_atlas(THINGS_ATLAS, 16, 16, "assets/textures/things.png");

// floors
const GRASS_TILE = 0;
define_tile(GRASS_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:GRASS_TILE,
    solid:false
});

const SAND_TILE = 1;
define_tile(SAND_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:SAND_TILE,
    solid:false
});

const STONE_TILE = 2;
define_tile(STONE_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:STONE_TILE,
    solid:false
});

const WOOD_TILE = 5;
define_tile(WOOD_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:WOOD_TILE,
    solid:false
});

// solids
const FOLIAGE_TILE = 16;
define_tile(FOLIAGE_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:FOLIAGE_TILE,
    solid:true
});

const ROCK_TILE = 18;
define_tile(ROCK_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:ROCK_TILE,
    solid:true
});

const WOODWALL_TILE = 21;
define_tile(WOODWALL_TILE, #{
    atlas:TILES_ATLAS,
    atlas_index:WOODWALL_TILE,
    solid:true
});

load_map("assets/maps/test.tmj");