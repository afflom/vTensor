Feature: GGML typed unary and integer rounding graphs
  @VT-27 @build
  Scenario: Prism plans thirteen unary operations for all five supported GGML float source destination pairs and Verdant matches copied in-place strided and empty CPU graphs with exact finite outputs and preserved padding
    Given thirteen operations and F32 F16 BF16 words at selection and rounding boundaries
    When Prism plans same-type or promoted output with copied or in-place storage
    Then every result matches the imported CPU graph and all padding survives
    And unsupported type pairs, variants, shapes and allocation states fail before writes
