Feature: oracle-import

  @VT-01 @build
  Scenario: The imported GGML backend contract, implementation interface, and operator oracle match their pinned upstream source
    Given the upstream commit and source-file digests are recorded
    When the oracle import test checks every recorded file
    Then The imported GGML backend contract, implementation interface, and operator oracle match their pinned upstream source
    And the upstream backend API, implementation interface, and operator test are included
