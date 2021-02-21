-- Block type: Air
terrain.addBlockType({
    id = 0,
    name = "Air",
    opaque = false,
    collidable = false,
});

-- Block type: Grass
terrain.addBlockType({
    id = 1,
    name = "Grass",
    opaque = true,
    collidable = true,
    tex = {
        side = 15 * 16 + 0,
        top = 15 * 16 + 1,
        bottom = 15 * 16 + 2,
    }
})