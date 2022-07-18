// floors
const GRASS_TILE = 0;
define_tile(#{
    index:GRASS_TILE,
    solid:false
});

const SAND_TILE = 1;
define_tile(#{
    index:SAND_TILE,
    solid:false
});

const STONE_TILE = 2;
define_tile(#{
    index:STONE_TILE,
    solid:false
});

const WOOD_TILE = 5;
define_tile(#{
    index:WOOD_TILE,
    solid:false
});

// solids
const FOLIAGE_TILE = 16;
define_tile(#{
    index:FOLIAGE_TILE,
    solid:true
});

const ROCK_TILE = 18;
define_tile(#{
    index:ROCK_TILE,
    solid:true
});

const WOODWALL_TILE = 21;
define_tile(#{
    index:WOODWALL_TILE,
    solid:true
});

load_map("assets/maps/test.tmj");