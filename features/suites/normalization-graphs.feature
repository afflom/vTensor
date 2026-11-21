Feature: normalization-graphs

  @VT-38 @build
  Scenario: Prism models GGML L2 normalization and Verdant matches every imported ordinary and batched upstream case with atomic invalid-parameter refusals
    Given the pinned unchanged upstream backend operation test factory and CPU reference backend
    When vTensor executes every L2_NORM and L2_NORM_BATCH case through Prism instructions and Verdant primitives
    Then all 61 required cases pass the imported comparison without unsupported or omitted cases
    And copied and in-place constructors match the actual CPU across contiguous and padded layouts
    And invalid epsilon type shape and unsafe alias metadata are refused before any output write
    And altered numeric and state bindings are rejected
