Feature: GGML mixed float storage copies
  @VT-24 @build
  Scenario: Prism plans mixed F32 F16 and BF16 casts and copies by logical element order and Verdant conversion matches the imported GGML CPU oracle across reshaped strided and empty layouts
    Given all six mixed float type pairs and both cast and copy constructors
    When Prism addresses encoded elements and Verdant converts them across batched strided reshaped and empty tensors
    Then complete outputs match the imported CPU graph including NaN bits and preserved padding
    And invalid counts, missing sources or targets and unsupported type pairs fail before writes
