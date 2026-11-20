Feature: GGML concatenation graphs
  @VT-17 @build
  Scenario: Prism selects and addresses CONCAT source blocks on every axis and Verdant execution matches the imported GGML CPU oracle across all live storage types
    Given every live storage type with unequal concatenated extents and padded sources
    When vTensor concatenates on each of the four axes using Prism and Verdant
    Then encoded output bytes match the imported CPU graph and padding is preserved
    And malformed shapes, axes and missing operands are refused before writes
