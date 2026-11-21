Feature: GGML typed scalar transform graphs
  @VT-26 @build
  Scenario: Prism plans scale bias clamp and leaky ReLU graphs and Verdant matches the imported GGML CPU kernels for every supported float storage type across copied in-place strided and empty tensors
    Given float parameter encodings, independently strided tensors and aliased output
    When Prism admits scale, scale bias, clamp and leaky ReLU through Verdant
    Then every logical output matches the imported CPU graph and padding is preserved
    And invalid shapes, types and allocation states fail before writes
