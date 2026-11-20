Feature: verdant-api

  @VT-02 @build
  Scenario: The Verdant integration exposes the fifteen unified tensor kernel descriptors from its pinned API
    Given the fixture VT-02 needs
    When the suite exercises VT-02
    Then The Verdant integration exposes the fifteen unified tensor kernel descriptors from its pinned API

  @VT-03 @build
  Scenario: vTensor delegates fixed 4x4 F64 matrix multiplication to Verdant Tensor and preserves column-major results
    Given the fixture VT-03 needs
    When the suite exercises VT-03
    Then vTensor delegates fixed 4x4 F64 matrix multiplication to Verdant Tensor and preserves column-major results
