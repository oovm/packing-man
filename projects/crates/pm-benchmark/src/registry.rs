//! 按问题分类索引的 fixture 注册表（显式路径，禁止 glob）。

pub struct FixtureEntry {
    pub path: &'static str,
    pub json: &'static str,
}

pub const ENTRIES: &[FixtureEntry] = &[
    FixtureEntry {
        path: "circle_sphere_packing/equal_circles_in_circle/n19",
        json: include_str!(
            "../fixtures/circle_sphere_packing/equal_circles_in_circle/n19.json"
        ),
    },
    FixtureEntry {
        path: "circle_sphere_packing/equal_circles_in_square/n10",
        json: include_str!(
            "../fixtures/circle_sphere_packing/equal_circles_in_square/n10.json"
        ),
    },
    FixtureEntry {
        path: "circle_sphere_packing/variable_circles_max_sum/unit_square_n26",
        json: include_str!(
            "../fixtures/circle_sphere_packing/variable_circles_max_sum/unit_square_n26.json"
        ),
    },
    FixtureEntry {
        path: "circle_sphere_packing/variable_circles_in_perim_rect/n21",
        json: include_str!(
            "../fixtures/circle_sphere_packing/variable_circles_in_perim_rect/n21.json"
        ),
    },
    FixtureEntry {
        path: "manufacturer_pallet_loading/one_dim_bin_packing/ffd_tight",
        json: include_str!(
            "../fixtures/manufacturer_pallet_loading/one_dim_bin_packing/ffd_tight.json"
        ),
    },
    FixtureEntry {
        path: "manufacturer_pallet_loading/cubes_in_cubes/n8_side3",
        json: include_str!(
            "../fixtures/manufacturer_pallet_loading/cubes_in_cubes/n8_side3.json"
        ),
    },
];

pub fn all_paths() -> Vec<&'static str> {
    ENTRIES.iter().map(|e| e.path).collect()
}

pub fn get(path: &str) -> Option<&'static str> {
    ENTRIES
        .iter()
        .find(|e| e.path == path || e.path.ends_with(path))
        .map(|e| e.json)
}
