Feature: GGML row-local cumulative sums
  @VT-28 @build
  Scenario: Prism models row-local F32 cumulative sums with exact first-word preservation and Verdant matches the imported GGML CPU scan across strided and empty graphs
    Given F32 rows with signed zeros cancellation subnormals infinities NaNs and random words
    When Prism selects first-word copies and ordered prefix additions for independent tensor strides
    Then every prefix and padding byte matches the imported CPU graph
    And invalid types shapes and allocation states are refused before writes
