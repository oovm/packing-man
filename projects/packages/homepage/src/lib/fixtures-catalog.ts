/** 与 `pm-benchmark/fixtures/catalog.json` 对齐（展示用元数据）。 */

export interface FixtureMeta {
    id: string;
    title: string;
    family: string;
    archetype: string;
    note: string;
}

export const FIXTURES: FixtureMeta[] = [
    {
        id: "circle_sphere_packing/equal_circles_in_circle/n19",
        title: "等圆装入圆盘 n=19",
        family: "circle_sphere_packing",
        archetype: "equal_circles_in_circle",
        note: "CIC 经典实例，已知最优",
    },
    {
        id: "circle_sphere_packing/equal_circles_in_square/n10",
        title: "等圆装入单位正方形 n=10",
        family: "circle_sphere_packing",
        archetype: "equal_circles_in_square",
        note: "CIS 冒烟",
    },
    {
        id: "circle_sphere_packing/variable_circles_max_sum/unit_square_n26",
        title: "可变半径和 · 单位正方形 n=26",
        family: "circle_sphere_packing",
        archetype: "variable_circles_max_sum",
        note: "AlphaEvolve 风格",
    },
    {
        id: "circle_sphere_packing/variable_circles_in_perim_rect/n21",
        title: "周长矩形内可变圆 n=21",
        family: "circle_sphere_packing",
        archetype: "variable_circles_in_perim_rect",
        note: "周长约束矩形",
    },
    {
        id: "manufacturer_pallet_loading/one_dim_bin_packing/ffd_tight",
        title: "1D 装箱 FFD 紧例",
        family: "manufacturer_pallet_loading",
        archetype: "one_dim_bin_packing",
        note: "First-Fit Decreasing",
    },
    {
        id: "manufacturer_pallet_loading/cubes_in_cubes/n8_side3",
        title: "立方体装入立方体 n=8",
        family: "manufacturer_pallet_loading",
        archetype: "cubes_in_cubes",
        note: "3D 极点法冒烟",
    },
];

export function fixtureJsonUrl(id: string): string {
    return `/fixtures/${id}.json`;
}
