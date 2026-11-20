Feature: GGML repeat graphs
  @VT-16 @build
  Scenario: Prism models four-axis REPEAT addressing and Verdant preserves encoded values for every type admitted by the imported GGML CPU repeat oracle
    Given all five upstream repeat storage types and varying four-axis repetition
    When vTensor executes contiguous and padded repeat graphs through Prism and Verdant
    Then complete output bytes match the imported CPU reference and padding is preserved
    And malformed repeat extents are refused before writes
