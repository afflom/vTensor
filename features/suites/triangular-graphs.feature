Feature: GGML triangular selection
  @VT-22 @build
  Scenario: Prism defines all four TRI variants and Verdant preserves retained encodings and zeroes masked elements for every type supported by the imported GGML CPU oracle
    Given every TRI variant with F32 F16 and BF16 square batched and empty inputs
    When vTensor executes the Prism selection plan through Verdant storage transfers
    Then the complete output matches the imported CPU graph and preserves padding
    And invalid variants, extents, types and missing sources fail before writes
