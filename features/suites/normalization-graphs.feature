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

  @VT-48 @build
  Scenario: Prism models copied and in-place F32 NORM with stride-selected accumulation and grouped contiguous variance while Verdant matches the pinned imported GGML CPU oracle
    Given the pinned GGML constructors CPU implementation and factory cases
    When Prism selects contiguous or strided mean and variance instruction phases
    Then Verdant results match every imported NORM case
    And copied and in-place edge cases preserve padding and atomic metadata refusals
    And cancellation and grouped-rounding defects are rejected

  @VT-49 @build
  Scenario: Prism models copied and in-place F32 GROUP_NORM with nested F64 row subtotals and Verdant matches the imported ordinary and fused GGML CPU cases
    Given the pinned GROUP_NORM constructors CPU implementation and ordinary and fused test factory
    When Prism plans channel groups row subtotals and centered variance phases
    Then every imported GROUP_NORM and GROUP_NORM_MUL_ADD case matches the authoritative CPU
    And copied and in-place edge cases preserve physical output spacing padding and atomic refusals
    And altered row-subtotal grouping and parameter bindings are rejected

  @VT-50 @build
  Scenario: Prism models F32 RMS_NORM_BACK with independent source strides and its specified narrowing order while Verdant matches every imported GGML CPU case
    Given the pinned RMS_NORM_BACK constructor CPU implementation and test factory
    When Prism accumulates both input reductions and orders the narrowed coefficient phases
    Then every imported backward normalization case matches the authoritative CPU
    And independent source layouts physical output spacing aliases and nonfinite parameters preserve CPU behavior
    And malformed metadata and overlapping output writes are typed atomic refusals
    And wrong source bindings and premature or delayed narrowing are rejected

  @VT-51 @build
  Scenario: Prism models composed normalization graphs with mixed scalar storage broadcast and aliases while Verdant matches every imported non-RoPE composed normalization factory case
    Given all 187 NORM_SCALE RMS_NORM_SCALE NORM_MUL_ADD RMS_NORM_MUL_ADD and ADD_RMS_NORM cases in the pinned factory
    When vTensor executes the complete imported graphs using certified Prism plans and Verdant scalar arithmetic
    Then every required case passes the independent CPU comparison without unsupported or omitted graphs
    And mixed BF16 and F32 weights and residuals preserve intermediate storage rounding
    And broadcasts in-place residuals and post-multiply graphs preserve authoritative alias behavior
