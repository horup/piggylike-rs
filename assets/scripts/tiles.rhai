import "atlases" as atlases;

// floors
const GRASS_TILE = 0;
define_tile(GRASS_TILE, #{
    atlas:atlases::TILES
    ,
    atlas_index:0,
    solid:false
});

const SAND_TILE = 1;
define_tile(SAND_TILE, #{
    atlas:atlases::TILES
    ,
    atlas_index:1,
    solid:false
});

const STONE_TILE = 2;
define_tile(STONE_TILE, #{
    atlas:atlases::TILES
    ,
    atlas_index:2,
    solid:false
});

const WOOD_TILE = 5;
define_tile(WOOD_TILE, #{
    atlas:atlases::TILES
    ,
    atlas_index:5,
    solid:false
});



// solids
const FOLIAGE_TILE = 16;
define_tile(FOLIAGE_TILE, #{
    atlas:atlases::TILES
    ,
    atlas_index:16,
    solid:true
});

const ROCK_TILE = 18;
define_tile(ROCK_TILE, #{
    atlas:atlases::TILES
    ,
    atlas_index:18,
    solid:true
});

const WOODWALL_TILE = 21;
define_tile(WOODWALL_TILE, #{
    atlas:atlases::TILES
    ,
    atlas_index:21,
    solid:true
});