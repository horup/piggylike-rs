import "atlases" as atlases;

// things
const PLAYER_THING = 0;
define_thing(PLAYER_THING, #{
    atlas:atlases::THINGS,
    atlas_index:0,
    player:true,
    solid:true
});

const PIGGY_THING = 1;
define_thing(PIGGY_THING, #{
    atlas:atlases::THINGS,
    atlas_index:1,
    solid:true
});

const RED_KEY = 16;
define_thing(RED_KEY, #{
    atlas:atlases::THINGS,
    atlas_index:16
});

const BLUE_KEY = 17;
define_thing(BLUE_KEY, #{
    atlas:atlases::THINGS,
    atlas_index:17
});

const PURPLE_KEY = 18;
define_thing(PURPLE_KEY, #{
    atlas:atlases::THINGS,
    atlas_index:18
});

const RED_DOOR = 32;
define_thing(RED_DOOR, #{
    atlas:atlases::THINGS,
    atlas_index:32,
    solid:true
});

const BLUE_DOOR = 33;
define_thing(BLUE_DOOR, #{
    atlas:atlases::THINGS,
    atlas_index:33,
    solid:true
});

const YELLOW_DOOR = 34;
define_thing(YELLOW_DOOR, #{
    atlas:atlases::THINGS,
    atlas_index:34,
    solid:true
});

const EXIT = 35;
define_thing(EXIT, #{
    atlas:atlases::THINGS,
    atlas_index:35
});


