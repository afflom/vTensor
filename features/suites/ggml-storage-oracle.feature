Feature: ggml-storage-oracle

  @VT-05 @build
  Scenario: The pinned GGML storage oracle executes upstream byte-span semantics for scalar and block-encoded tensors
    Given the complete GGML sources are archived from the pinned authority revision
    When a test-only executable calls unmodified upstream ggml_nbytes
    Then scalar padded four-dimensional block-encoded and empty tensor cases match their fixtures
    And malformed transport is refused
    And native build reuse binds source transport build recipe compiler SDK and executable bytes
