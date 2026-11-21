Feature: GGML selection and equality row graphs
  @VT-30 @build
  Scenario: Prism models ARGMAX last-tie and NaN state and logical COUNT_EQUAL across all four tensor axes and Verdant matches imported GGML CPU row operations with typed refusals and exact integer outputs
    Given F32 matrices with ties NaNs and empty widths and paired I32 tensors with independent strides
    When Prism orders comparison state updates and logical equality accumulation through Verdant
    Then ARGMAX indices and COUNT_EQUAL totals match the imported CPU graph with exact padding
    And invalid types shapes overflow and allocation states fail before writes

  @VT-31 @build
  Scenario: Prism models indexed F16 and F32 row-gradient accumulation into F32 and Verdant matches imported GGML GET_ROWS_BACK graphs with duplicate indices empty tensors and atomic invalid-index refusals
    Given F16 and F32 gradients and independently strided I32 row indices
    When Prism validates indices before clearing and orders duplicate-row additions through Verdant
    Then logical F32 results match the imported CPU oracle and padding remains unchanged
    And invalid indices and descriptors are refused before output writes

  @VT-32 @build
  Scenario: Prism models four-dimensional indexed row gathering and every executable GGML CPU source format and Verdant decodes the packed blocks with imported authoritative codebooks while preserving integer words padding empty tensors and atomic invalid-index refusals
    Given float integer and every CPU-decodable packed source format with independently strided batched I32 indices
    When Prism admits every indexed address and orders Verdant block decoding and source-word transfers
    Then gathered logical words match the imported GGML CPU and output padding remains unchanged
    And empty tensors need no buffers and every invalid index is refused before any output write
