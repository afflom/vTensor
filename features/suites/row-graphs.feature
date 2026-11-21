Feature: GGML selection and equality row graphs
  @VT-30 @build
  Scenario: Prism models ARGMAX last-tie and NaN state and logical COUNT_EQUAL across all four tensor axes and Verdant matches imported GGML CPU row operations with typed refusals and exact integer outputs
    Given F32 matrices with ties NaNs and empty widths and paired I32 tensors with independent strides
    When Prism orders comparison state updates and logical equality accumulation through Verdant
    Then ARGMAX indices and COUNT_EQUAL totals match the imported CPU graph with exact padding
    And invalid types shapes overflow and allocation states fail before writes
