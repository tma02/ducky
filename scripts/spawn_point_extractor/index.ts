const convert = require('tscn2json');

// Note: clear the {} brackets from the main_zone.tscn

interface Vector3 {
    x: number,
    y: number,
    z: number,
};

(async () => {
    let root = await convert({
        input: 'main_zone.tscn',
    });
    let out = {
        fish_spawn: [] as Vector3[],
        trash_point: [] as Vector3[],
        shoreline_point: [] as Vector3[],
        hidden_spot: [] as Vector3[],
    };
    for (let entity of root.entities) {
        if (entity?.heading?.groups?.[0] == "fish_spawn") {
            let points = entity.props.transform.params.slice(9, 12);
            out.fish_spawn.push({
                x: points[0],
                y: points[1],
                z: points[2],
            });
        } else if (entity?.heading?.groups?.[0] == "trash_point") {
            let points = entity.props.transform.params.slice(9, 12);
            out.trash_point.push({
                x: points[0],
                y: points[1],
                z: points[2],
            });
        } else if (entity?.heading?.groups?.[0] == "shoreline_point") {
            let points = entity.props.transform.params.slice(9, 12);
            out.shoreline_point.push({
                x: points[0],
                y: points[1],
                z: points[2],
            });
        } else if (entity?.heading?.groups?.[0] == "hidden_spot") {
            let points = entity.props.transform.params.slice(9, 12);
            out.hidden_spot.push({
                x: points[0],
                y: points[1],
                z: points[2],
            });
        }
    }
    await Bun.write("./spawn_points.json", JSON.stringify(out));
})();
