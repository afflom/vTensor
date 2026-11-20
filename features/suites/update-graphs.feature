Feature: GGML strided tensor updates
  @VT-19 @build
  Scenario: Prism plans SET and ACC initialization, bounded update views and aliases and Verdant execution matches the imported GGML CPU oracle for supported float and integer storage
    Given copied and aliased sources with empty, sparse and overlapping update views
    When vTensor performs in-place and out-of-place SET and ACC through Prism and Verdant
    Then whole outputs match the imported CPU graph and untouched values remain unchanged
    And invalid update bounds, missing sources and false alias declarations fail before writes
