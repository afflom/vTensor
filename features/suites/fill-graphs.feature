Feature: GGML constant fill
  @VT-23 @build
  Scenario: Prism plans copied and in-place FILL and Verdant converts and transfers F32 and F16 constants with upstream-equivalent encodings and preserved padding
    Given F32 and F16 constants including rounding boundaries and exceptional encodings
    When copied and in-place FILL graphs run across batched, strided and empty layouts
    Then complete outputs match the imported CPU graph and preserve padding
    And unallocated unused inputs are accepted while invalid shapes, types and missing descriptors fail before writes
