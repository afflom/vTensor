Feature: normalization-graphs

  @VT-45 @build
  Scenario: Verdant supplies certified F64 division and unsigned count to F64 conversion for normalization and preserves the imported GGML accumulator division before narrowing regression
    Given the imported GGML RMS normalization mean expression and its frozen rounding counterexample
    When Verdant converts the row count divides the F64 accumulator and narrows the quotient
    Then the result matches the authoritative expression without premature rounding
    And large unsigned counts preserve F64 nearest even conversion
    And unsupported operation codes are typed refusals

  @VT-38 @build
  Scenario: Prism models GGML L2 normalization and Verdant matches every imported ordinary and batched upstream case with atomic invalid-parameter refusals
    Given the pinned unchanged upstream backend operation test factory and CPU reference backend
    When vTensor executes every L2_NORM and L2_NORM_BATCH case through Prism instructions and Verdant primitives
    Then all 61 required cases pass the imported comparison without unsupported or omitted cases
    And copied and in-place constructors match the actual CPU across contiguous and padded layouts
    And invalid epsilon type shape and unsafe alias metadata are refused before any output write
    And altered numeric and state bindings are rejected

  @VT-46 @build
  Scenario: Prism models copied and in-place F32 and BF16 RMS normalization and Verdant matches every imported GGML CPU case with F64 mean ordering and atomic numeric refusals
    Given the pinned unchanged upstream RMS_NORM constructors test factory and CPU reference backend
    When vTensor executes F32 and BF16 copied and in-place graphs through certified Prism instructions and Verdant arithmetic
    Then every imported ordinary RMS_NORM case passes without unsupported or omitted cases
    And F64 accumulation is divided before narrowing the mean to F32
    And all row scales are validated before any output write
    And invalid parameters layouts aliases and numeric rows are typed atomic refusals
