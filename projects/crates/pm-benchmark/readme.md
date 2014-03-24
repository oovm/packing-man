# pm-benchmark

按 `ProblemFamily` / `ProblemArchetype` 分类的基准实例，唯一数据源在 `fixtures/`。

每个 JSON 含完整 `problem` 合同与 `expect` 校验字段，不以第三方站点名称组织目录。

```text
fixtures/
  catalog.json
  circle_sphere_packing/
    equal_circles_in_circle/n19.json
    equal_circles_in_square/n10.json
    variable_circles_max_sum/unit_square_n26.json
    variable_circles_in_perim_rect/n21.json
  manufacturer_pallet_loading/
    one_dim_bin_packing/ffd_tight.json
    cubes_in_cubes/n8_side3.json
```
